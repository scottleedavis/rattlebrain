use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

/// Sends a query to Claude AI and returns the response.
///
/// # Arguments
/// * `prompt` - The prompt string to send to Claude.
///
/// # Returns
/// A `Result` containing the response string, or an error if the request fails.
pub async fn query_anthropic(prompt: &str) -> Result<String, Box<dyn Error>> {
    // Retrieve the API key from the environment
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");

    // Hardcoded configuration
    let base_url = "https://api.anthropic.com/v1/messages";
    let model = "claude-3-5-sonnet-20241022";
    let max_tokens = 8192;

    // Prepare the request body
    let request_body = ClaudeRequest {
        model: model.to_string(),
        max_tokens,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    // Initialize HTTP client
    let client = Client::new();

    // Send the request
    let response = client
        .post(base_url)
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Extract status and response text for debugging
    let status = response.status();
    let response_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

    if status.is_success() {
        // eprintln!("Successful response: {}", response_text); // Debugging raw response
        // Parse the response JSON
        let response_body: ClaudeResponse = serde_json::from_str(&response_text)?;

        // Extract and concatenate the text from the content array
        let response_text = response_body
            .content
            .into_iter()
            .map(|c| c.text)
            .collect::<Vec<String>>()
            .join(" ");

        Ok(response_text)
    } else {
        eprintln!("Failed response: {}", response_text); // Debugging raw error response
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status: {} - {}", status, response_text),
        )))
    }
}
