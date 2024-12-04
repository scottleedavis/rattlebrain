mod extract;
mod convert;
mod plot;
mod ai;

use std::env;
use std::fs;
use std::process;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rattlebrain <command> [options]");
        println!("Commands:");
        println!(" analysis <path/some.replay> - Analyze replay data. (runs extract, convert,plot and ai in sequence)");
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
                println!("Usage: rattlebrain extract <input>");
                return;
            }
            let input = &args[2];
            println!("Extracting replay data...");
            match extract::extract_replay(input) {
                Ok(match_guid) => {
                    println!("Extract command completed successfully.");
                    println!("Match GUID: {}", match_guid);
                }
                Err(e) => eprintln!("Error extracting replay: {}", e),
            }
        }
        "convert" => {
            if args.len() < 3 {
                println!("Usage: rattlebrain convert <input>");
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
                println!("Usage: rattlebrain plot <csv>");
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
                println!("Usage: rattlebrain analysis <file.replay>");
                return;
            }
            let input = &args[2];

            println!("Starting analysis...");

            // Step 1: Extract
            println!("Extracting replay data: {}", input);
            let match_guid = match extract::extract_replay(input) {
                Ok(match_guid) => {
                    println!("Extraction successful. Match GUID: {}", match_guid);
                    match_guid // Store the match_guid for Step 2
                }
                Err(e) => {
                    eprintln!("Error during extraction: {}", e);
                    process::exit(1);
                }
            };

            // Step 2: Convert 
            let replay_file = format!("./output/{}.replay.frames.json", match_guid);
            let player_statistics_file = format!("./output/{}.player_stats.json", match_guid);
            let goals_file = format!("./output/{}.goals.json", match_guid);
            let highlights_file = format!("./output/{}.highlights.json", match_guid);

            // Process each file
            for file in [replay_file, player_statistics_file, goals_file, highlights_file].iter() {
                process_conversion(file);
            }

            // Step 3: Plot
            let csv_file = format!("./output/{}.replay.frames.json.csv",match_guid);
            println!("Plotting data from csv: {}", csv_file);
            if let Err(e) = plot::plot_csv(&csv_file) {
                eprintln!("Error during plotting: {}", e);
                process::exit(1);
            }

            // Step 4: AI
            println!("TOOD AI analysis....");
        }
        "ai" => {
            if args.len() < 3 {
                println!("Usage: rattlebrain ai <input>");
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
            println!("Usage: rattlebrain <command> [options]");
        }
    }
}

fn process_conversion(file_path: &str) {
    println!("Converting replay data to CSV: {}", file_path);

    let file_content = match fs::read_to_string(file_path) {
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

    if let Err(e) = convert::convert_replay(json_data, file_path) {
        eprintln!("Error during conversion: {}", e);
        process::exit(1);
    }

    println!("Conversion completed successfully for file: {}", file_path);
}
