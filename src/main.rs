use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::Parser;
use crate::analyzer::analyze_replay;

mod analyzer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the replay file
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    // Read the replay file
    let mut file = File::open(&args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse the replay
    let replay = match boxcars::ParserBuilder::new(&buffer).parse() {
        Ok(replay) => replay,
        Err(e) => return Err(format!("Failed to parse replay: {}", e).into()),
    };

    // Analyze the replay
    let analysis = analyze_replay(&replay)?;

    // Print the analysis results
    println!("Replay Analysis:");
    println!("---------------");
    println!("Engine Version: {}", analysis.engine_version);
    println!("Score: {} - {}", analysis.game_score.team_0_score, analysis.game_score.team_1_score);
    println!("Primary Player: {}", analysis.primary_player);
    println!("Team Sizes: {} vs {}", analysis.team_sizes.blue, analysis.team_sizes.orange);
    println!("Match Type: {}", analysis.match_type);
    println!("Arena: {}", analysis.arena);
    println!("Platform: {}", analysis.platform);
    println!("Date: {}", analysis.date);
    println!("Total Actor Updates: {}", analysis.actor_stats.total_updates);

    Ok(())
}