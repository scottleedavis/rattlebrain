use std::env;
use std::io;
use crate::ai::openai;

use std::fs;
use flate2::write::GzEncoder;
use flate2::Compression;
use base64::{engine::general_purpose, Engine as _};
use std::io::Write;

pub async fn query_ai(match_guid: &str, focus: &str) -> io::Result<String> {
    // Define file paths based on the match_guid
    let player_stats_csv_path = format!("./output/{}.player_stats.json.csv", match_guid);
    let goals_csv_path = format!("./output/{}.goals.json.csv", match_guid);
    let highlights_csv_path = format!("./output/{}.highlights.json.csv", match_guid);
    let frames_csv_path = format!("./output/{}.replay.frames.json.csv", match_guid);

    // Read the contents of each CSV file
    let player_stats_csv = fs::read_to_string(&player_stats_csv_path)
        .unwrap_or_else(|_| format!("Error reading {}", player_stats_csv_path));
    let goals_csv = fs::read_to_string(&goals_csv_path)
        .unwrap_or_else(|_| format!("Error reading {}", goals_csv_path));
    let highlights_csv = fs::read_to_string(&highlights_csv_path)
        .unwrap_or_else(|_| format!("Error reading {}", highlights_csv_path));
    let frames_csv = fs::read_to_string(&frames_csv_path)
        .unwrap_or_else(|_| format!("Error reading {}", frames_csv_path));

    let strategy_template = if focus == "strategy" || focus == "all" {
        format!(
            r#"Strategy Analysis:
            Analyze team positioning, rotations, and overall synergy.
            "#,
        )
    } else {
        String::new()
    };

    let mechanics_template = if focus == "mechanics" || focus == "all" {
        format!(
            r#"Mechanics Analysis:
            Evaluate boost efficiency, aerial control, and shot accuracy.
            "#,
        )
    } else {
        String::new()
    };

    let decision_making_template = if focus == "decision_making" || focus == "all" {
        format!(
            r#"Decision-Making Analysis:
            Provide insights on situational awareness and risk/reward trade-offs.
            "#,
        )
    } else {
        String::new()
    };

    let frames_subset_csv = match compress_and_encode_csv(&frames_csv) {
        Ok(compressed_base64) => compressed_base64,
        Err(e) => format!("Error compressing and encoding frames CSV: {}", e),
    };


    // Combine all selected templates
    let query = format!(
        r#"
        You are a world-class Rocket League team coach providing helpful feedback for improvement.
        {}
        {}
        {}

        Player statistics:
{}

        Goal breakdown:
{}

        Highlights:
{}

        Compressed Frames Data (Base64 gziped)::
{}
        "#,
        strategy_template, mechanics_template, decision_making_template, player_stats_csv, goals_csv, highlights_csv, frames_subset_csv
    );

    println!("Processing AI query: {}", query);
    let query_file_path = format!("./output/{}_query.txt", match_guid);
    fs::write(&query_file_path, &query).map_err(|e| {
        eprintln!("Failed to save query to file: {}", query_file_path);
        e
    })?;

    // Detect available AI providers and collect responses
    let mut responses = Vec::new();

    // OpenAI
    if let Ok(openai_key) = env::var("OPENAI_API_KEY") {
        println!("Using OpenAI with key: {}", &openai_key[0..4]);
        match openai::query_openai(&query).await {
            Ok(response) => responses.push(format!("OpenAI response: {}", response)),
            Err(e) => eprintln!("Error querying OpenAI: {}", e),
        }
    }


    // Claude
    if let Ok(claude_key) = env::var("CLAUDE_API_KEY") {
        println!("Using Claude with key: {}", &claude_key[0..4]);
        responses.push(format!(
            "Claude response to '{}': [This is a stubbed response from Claude]",
            query
        ));
    }

    // Gemini
    if let Ok(gemini_key) = env::var("GEMINI_API_KEY") {
        println!("Using Gemini with key: {}", &gemini_key[0..4]);
        responses.push(format!(
            "Gemini response to '{}': [This is a stubbed response from Gemini]",
            query
        ));
    }

    // GitHub Copilot
    if let Ok(copilot_key) = env::var("COPILOT_API_KEY") {
        println!("Using GitHub Copilot with key: {}", &copilot_key[0..4]);
        responses.push(format!(
            "GitHub Copilot response to '{}': [This is a stubbed response from Copilot]",
            query
        ));
    }

    // Handle case where no providers are configured
    if responses.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No AI providers configured. Please set one or more of the following: OPENAI_API_KEY, CLAUDE_API_KEY, GEMINI_API_KEY, COPILOT_API_KEY.",
        ));
    }


    // Combine all responses
    let combined_response = responses.join("\n");
    Ok(combined_response)
}



// Function to compress and Base64-encode the CSV
fn compress_and_encode_csv(csv_content: &str) -> std::io::Result<String> {
    // Compress the CSV content using gzip
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(csv_content.as_bytes())?;
    let compressed_data = encoder.finish()?;

    // Encode the compressed data in Base64
    let base64_encoded = general_purpose::STANDARD.encode(&compressed_data);
    Ok(base64_encoded)
}

