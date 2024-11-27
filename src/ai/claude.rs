use crate::ai::generic::AIInterface;

pub struct Claude {
    api_key: String,
}

impl Claude {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl AIInterface for Claude {
    fn query(&self, input: &str) -> Result<String, String> {
        // Placeholder for API request logic
        Ok(format!("Claude response for: {}", input))
    }
}


// use crate::types::{AnalyzerError, ApiRequest, ApiResponse, Message};
// use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};

// pub struct ClaudeClient {
//     api_key: String,
//     client: reqwest::Client,
//     model: String,
// }

// impl ClaudeClient {
//     pub fn new(api_key: String, model: Option<String>) -> Self {
//         ClaudeClient {
//             api_key,
//             client: reqwest::Client::new(),
//             model: model.unwrap_or_else(|| "claude-3-sonnet-20240229".to_string()),
//         }
//     }

//     pub async fn get_analysis(&self, prompt: &str) -> Result<String, AnalyzerError> {
//         let mut headers = HeaderMap::new();
//         headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
//         headers.insert(
//             AUTHORIZATION,
//             HeaderValue::from_str(&format!("Bearer {}", self.api_key))
//                 .map_err(|_| AnalyzerError::MissingData("Invalid API key".to_string()))?,
//         );

//         let messages = vec![Message {
//             role: "user".to_string(),
//             content: prompt.to_string(),
//         }];

//         let request = ApiRequest {
//             model: self.model.clone(),
//             messages,
//         };

//         let response: ApiResponse = self.client
//             .post("https://api.anthropic.com/v1/messages")
//             .headers(headers)
//             .json(&request)
//             .send()
//             .await?
//             .json()
//             .await?;

//         Ok(response.content[0].text.clone())
//     }
// }