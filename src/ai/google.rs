use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)] // Added Deserialize for request and response
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)] // Added Deserialize for request and response
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

/// Sends a query to Gemini AI and returns the response.
///
/// # Arguments
/// * `prompt` - The prompt string to send to Gemini.
///
/// # Returns
/// A `Result` containing the response string, or an error if the request fails.
pub async fn query_gemini(prompt: &str) -> Result<String, Box<dyn Error>> {
    // Retrieve the API key from the environment
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    // API URL
    let base_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    // Prepare the request body
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    // Initialize HTTP client
    let client = Client::new();

    // Send the request
    let response = client
        .post(&base_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Extract status and response text for debugging
    let status = response.status();
    let response_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

    if status.is_success() {
        // eprintln!("Successful response: {}", response_text); // Debugging raw response
        // Parse the response JSON
        let response_body: GeminiResponse = serde_json::from_str(&response_text)?;

        // Extract and concatenate text from the candidate's content parts
        let output = response_body
            .candidates
            .into_iter()
            .flat_map(|candidate| candidate.content.parts)
            .map(|part| part.text)
            .collect::<Vec<String>>()
            .join("\n");

        Ok(output)
    } else {
        eprintln!("Failed response: {}", response_text); // Debugging raw error response
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status: {} - {}", status, response_text),
        )))
    }
}
