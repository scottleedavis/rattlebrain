use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

pub async fn query_openai(prompt: &str) -> Result<String, Box<dyn Error>> {
    // Get the OpenAI API key from the environment
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url = "https://api.openai.com/v1/chat/completions";

    // Create the OpenAI request payload
    let request_body = OpenAIRequest {
        model: "gpt-4".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a world-class Rocket League coach.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    // Send the request to OpenAI
    let client = Client::new();
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    // Handle the response
    if response.status().is_success() {
        let openai_response: OpenAIResponse = response.json().await?;
        if let Some(choice) = openai_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response content from OpenAI.".into())
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        Err(format!("OpenAI API Error ({}): {}", status, error_text).into())
    }
}
