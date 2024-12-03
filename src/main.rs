mod extract;
mod convert;
mod plot;
mod ai;
use std::path::Path;
use std::env;
use std::fs;
use std::process;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rocket-league-replay-ai-analysis <command> [options]");
        println!("Commands:");
        println!(" analysis <path/some.replay> - Analyze replay data. (runs extract, convert, plot and ai in sequence)");
        println!(" extract <path/some.replay> - Extract replay data to CSV.");
        println!(" convert <path/some.replay.json> - Convert replay JSON to processed data.");
        println!(" plot <<path/some.replay.csv> - Plot replay data.");
        println!(" ai <query> - Query AI for replay insights.");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "extract" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis extract <input>");
                return;
            }
            let input = &args[2];
            println!("Extracting replay data...");
            match extract::extract_replay(input) {
                Ok(_) => println!("Extract command completed successfully."),
                Err(e) => eprintln!("Error extracting replay: {}", e),
            }
        }
        "convert" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis convert <input>");
                return;
            }
            let input = &args[2];
            println!("Converting replay data...");

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

            // Convert the replay
            match convert::convert_replay(json_data,input) {
                Ok(_) => println!("Convert command completed successfully."),
                Err(e) => eprintln!("Error converting replay: {}", e),
            }
        }
        "plot" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis plot <csv>");
                return;
            }
            let csv_file = &args[2];
            println!("Plotting CSV...");
            match plot::plot_csv(csv_file) {
                Ok(_response) => println!("Plot command completed successfully: "),
                Err(e) => eprintln!("Error plotting: {}", e),
            }
        }
        "analysis" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis analysis <file.replay>");
                return;
            }
            let input = &args[2];

            println!("Starting analysis...");

            // Step 1: Extract
            println!("Extracting replay data: {}", input);
            if let Err(e) = extract::extract_replay(input) {
                eprintln!("Error during extraction: {}", e);
                process::exit(1);
            }

            // Step 2: Convert 
            let json_replay_filename = Path::new(input)
                .file_name()
                .unwrap_or_default() // Safely handle missing filenames
                .to_string_lossy();  // Convert to a readable string
            let json_replay = format!("./output/{}.frames.json",json_replay_filename);
            println!("Converting replay data to csv: {}", json_replay);
            let file_content = match fs::read_to_string(json_replay.clone()) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading input file: {}", e);
                    process::exit(1);
                }
            };

            let json_data: Value = match serde_json::from_str(&file_content) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    process::exit(1);
                }
            };

            if let Err(e) = convert::convert_replay(json_data, &json_replay.clone()) {
                eprintln!("Error during conversion: {}", e);
                process::exit(1);
            }

            // // Step 3: Plot
            let json_frames_replay = format!("./output/{}.frames.json",json_replay_filename);
            let csv_file = format!("{}.csv", json_frames_replay); 
            println!("Plotting data from csv: {}", csv_file);
            if let Err(e) = plot::plot_csv(&csv_file) {
                eprintln!("Error during plotting: {}", e);
                process::exit(1);
            }

            println!("Now AI analysis....");
        }
        "ai" => {
            if args.len() < 3 {
                println!("Usage: rocket-league-replay-ai-analysis ai <input>");
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
