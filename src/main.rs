use clap::Parser;
use dotenv::dotenv;
use std::path::PathBuf;
use tracing::{info, error};

mod analyzer;
mod claude;
mod types;

use analyzer::ReplayAnalyzer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the replay file
    #[arg(short, long)]
    replay: PathBuf,

    /// Analysis focus (tactical, mechanical, positioning)
    #[arg(short, long, default_value = "general")]
    focus: String,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Get API key from environment
    let api_key = match std::env::var("CLAUDE_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("CLAUDE_API_KEY environment variable not set");
            std::process::exit(1);
        }
    };

    // Get optional model override
    let model = std::env::var("RL_ANALYZER_MODEL").ok();

    // Initialize analyzer
    let analyzer = ReplayAnalyzer::new(api_key, model);

    // Process replay file
    match analyzer.analyze_replay(&cli.replay).await {
        Ok(analysis) => {
            info!("Analysis complete");
            println!("\nAnalysis Results:\n{}", analysis);
        }
        Err(e) => {
            error!("Error analyzing replay: {}", e);
            std::process::exit(1);
        }
    }
}