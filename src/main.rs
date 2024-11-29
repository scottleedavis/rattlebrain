mod extract;
mod analysis;
mod ai;

use std::env;
use std::fs;
use std::process;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rocket-league-replay-ai-analysis <command> [options]");
        println!("Commands:");
        println!(" extract <input> <output> - Extract replay data to CSV.");
        println!(" analysis <input> - Analyze replay data.");
        println!(" ai <query> - Query AI for replay insights.");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "extract" => {
            if args.len() < 4 {
                println!("Usage: rocket-league-replay-ai-analysis extract <input> <output>");
                return;
            }
            let input = &args[2];
            let output = &args[3];
            println!("Extracting replay data...");
            match extract::extract_replay(input, output) {
                Ok(_) => println!("Extract command completed successfully."),
                Err(e) => eprintln!("Error extracting replay: {}", e),
            }
        }
        "analysis" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis analysis <input>");
                return;
            }
            let input = &args[2];
            println!("Analyzing replay data...");

            // Read the input file
            let file_content = match fs::read_to_string(input) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading input file: {}", e);
                    process::exit(1);
                }
            };

            // Parse the JSON content
            let json_data: Value = match serde_json::from_str(&file_content) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    process::exit(1);
                }
            };

            // Analyze the replay
            match analysis::analyze_replay(json_data,input) {
                Ok(_) => println!("Analysis command completed successfully."),
                Err(e) => eprintln!("Error analyzing replay: {}", e),
            }
        }
        "ai" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis ai <query>");
                return;
            }
            let query = &args[2];
            println!("Querying AI for insights...");
            match ai::query_ai(query) {
                Ok(response) => println!("AI Response: {}", response),
                Err(e) => eprintln!("Error querying AI: {}", e),
            }
        }
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: rocket-league-replay-ai-analysis <command> [options]");
        }
    }
}
