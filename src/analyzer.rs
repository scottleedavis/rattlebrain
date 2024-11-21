use boxcars::{ParseError, ParserBuilder, Replay};
use std::path::Path;
use crate::types::AnalyzerError;
use crate::claude::ClaudeClient;

pub struct ReplayAnalyzer {
    claude_client: ClaudeClient,
}

impl ReplayAnalyzer {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        ReplayAnalyzer {
            claude_client: ClaudeClient::new(api_key, model),
        }
    }

    pub async fn analyze_replay(&self, replay_path: &Path) -> Result<String, AnalyzerError> {
        let replay_data = self.read_replay_file(replay_path)?;
        let replay = self.parse_replay(&replay_data)?;
        let analysis_prompt = self.create_analysis_prompt(&replay)?;
        let analysis = self.claude_client.get_analysis(&analysis_prompt).await?;
        
        Ok(analysis)
    }

    fn read_replay_file(&self, path: &Path) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(path)
    }

    fn parse_replay(&self, data: &[u8]) -> Result<Replay, ParseError> {
        ParserBuilder::new(data).parse()
    }

    fn create_analysis_prompt(&self, replay: &Replay) -> Result<String, AnalyzerError> {
        let properties = replay.header.properties.as_ref()
            .ok_or_else(|| AnalyzerError::MissingData("No properties found in replay".to_string()))?;

        Ok(format!(
            "Please analyze this Rocket League replay with the following details:\n\n\
            Match Type: {}\n\
            Map: {}\n\
            Date: {}\n\
            Game Version: {}\n\n\
            Please provide:\n\
            1. Overall match summary\n\
            2. Key gameplay events\n\
            3. Player performance analysis\n\
            4. Tactical insights\n\
            5. Areas for improvement",
            properties.match_type.as_deref().unwrap_or("Unknown"),
            properties.map_name.as_deref().unwrap_or("Unknown"),
            properties.date.as_deref().unwrap_or("Unknown"),
            replay.header.engine_version
        ))
    }
}