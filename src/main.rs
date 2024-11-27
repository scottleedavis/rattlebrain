mod replay_parser;
mod analysis;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rocket-league-replay-ai-analysis <command> [options]");
        println!("Commands:");
        println!("  extract <input> <output> - Extract replay data to CSV.");
        println!("  analyze <input>          - Analyze replay data.");
        println!("  ai <query>               - Query AI for replay insights.");
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
            match replay_parser::extract_replay(input, output) {
                Ok(_) => println!("Extract command completed successfully."),
                Err(e) => eprintln!("Error extracting replay: {}", e),
            }
        }
        "analysis" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis analyze <input>");
                return;
            }
            let input = &args[2];
            match analysis::analyze_replay(input) {
                Ok(_) => println!("Analyze command completed successfully."),
                Err(e) => eprintln!("Error analyzing replay: {}", e),
            }
        }
        "ai" => {
            println!("Querying AI...");
            // Placeholder for AI functionality
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
