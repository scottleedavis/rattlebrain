use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("Failed to read replay file: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Failed to parse replay: {0}")]
    ParseError(#[from] boxcars::ParseError),
    
    #[error("API error: {0}")]
    ApiError(#[from] reqwest::Error),
    
    #[error("Missing replay data: {0}")]
    MissingData(String),
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ApiRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub text: String,
}