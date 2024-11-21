use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use boxcars::{NetworkParse, ParserBuilder};
use crate::analyzer::analyze_replay;

mod analyzer;

fn main() -> Result<(), Box<dyn Error>> {
    // Get file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <replay_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    // Validate file existence
    if !path.exists() {
        eprintln!("Error: Replay file '{}' does not exist.", file_path);
        std::process::exit(1);
    }

    // Read the replay file
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse the replay
    let replay = ParserBuilder::new(&buffer)
        .with_network_parse(NetworkParse::Always)
        .parse()
        .map_err(|e| {
            eprintln!("Failed to parse replay file: {}", e);
            e
        })?;

    // Analyze the replay
    let analysis = analyze_replay(&replay).map_err(|e| {
        eprintln!("Failed to analyze replay data: {}", e);
        e
    })?;

    // Print analysis results
    println!("\nGame Analysis:");
    println!("-------------");
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
