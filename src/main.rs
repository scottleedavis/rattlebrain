// use std::env;

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
            println!("Extracting replay data...");
            // Placeholder for `extract` functionality
        }
        "analyze" => {
            println!("Analyzing replay...");
            // Placeholder for `analyze` functionality
        }
        "ai" => {
            println!("Querying AI...");
            // Placeholder for `ai` functionality
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
