use crate::ai::generic::AIInterface;
use reqwest::Client;

pub struct OpenAI {
    api_key: String,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl AIInterface for OpenAI {
    fn query(&self, input: &str) -> Result<String, String> {
        // Placeholder for API request logic
        Ok(format!("OpenAI response for: {}", input))
    }
}
