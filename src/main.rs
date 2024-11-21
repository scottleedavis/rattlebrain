use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use boxcars::{NetworkParse, ParserBuilder};
use crate::analyzer::{
    extract_primary_player, extract_match_type, extract_arena, extract_platform, extract_date,
    get_property_value,
};

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

    // Extract replay analysis
    let primary_player = extract_primary_player(&replay);
    let match_type = extract_match_type(&replay);
    let arena = extract_arena(&replay);
    let platform = extract_platform(&replay);
    let date = extract_date(&replay);
    let total_actor_updates = replay.network_frames.iter().count();

    // Extract additional information
    let engine_version = get_property_value(&replay.properties, "EngineVersion")
        .and_then(|v| v.as_i32())
        .unwrap_or(0);
    let team0_score = get_property_value(&replay.properties, "Team0Score")
        .and_then(|v| v.as_i32())
        .unwrap_or(0);
    let team1_score = get_property_value(&replay.properties, "Team1Score")
        .and_then(|v| v.as_i32())
        .unwrap_or(0);
    let team0_size = get_property_value(&replay.properties, "Team0Size")
        .and_then(|v| v.as_i32())
        .unwrap_or(0);
    let team1_size = get_property_value(&replay.properties, "Team1Size")
        .and_then(|v| v.as_i32())
        .unwrap_or(0);

    // Print analysis results
    println!("\nGame Analysis:");
    println!("-------------");
    println!("Engine Version: {}", engine_version);
    println!("Score: {} - {}", team0_score, team1_score);
    println!("Primary Player: {}", primary_player);
    println!("Team Sizes: {} vs {}", team0_size, team1_size);
    println!("Match Type: {}", match_type);
    println!("Arena: {}", arena);
    println!("Platform: {}", platform);
    println!("Date: {}", date);
    println!("Total Actor Updates: {}", total_actor_updates);

    Ok(())
}
