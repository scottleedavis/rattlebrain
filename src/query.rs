use crate::ai::openai;
use crate::ai::anthropic;

use std::fs;
use std::env;
use std::io;
use base64::{Engine as _};
use std::io::Write;
use csv::{ReaderBuilder, Writer};
use std::error::Error;
use std::collections::HashMap;

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

    let frames_subset_csv = match filter_compress_and_encode_frames(&frames_csv) {
        Ok(compressed_base64) => compressed_base64,
        Err(e) => format!("Error processing frames CSV: {}", e),
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

        Nth Frame Filtered, Compressed, Base64'd Frames Data:
{}
        "#,
        strategy_template, mechanics_template, decision_making_template, player_stats_csv, goals_csv, highlights_csv, frames_subset_csv
    );

    // println!("Processing AI query: {}", query);
    let query_file_path = format!("./output/{}.query.txt", match_guid);
    fs::write(&query_file_path, &query).map_err(|e| {
        eprintln!("Failed to save query to file: {}", query_file_path);
        e
    })?;

    // Detect available AI providers and collect responses
    let mut responses = Vec::new();
    let header_response = "# Rattlebrain Replay Analysis\n\n\n".to_string();
    responses.push(header_response);

    // OpenAI
    if let Ok(openai_key) = env::var("OPENAI_API_KEY") {
        println!("Using OpenAI with key: {}****", &openai_key[0..8]);
        match openai::query_openai(&query).await {
            Ok(response) => responses.push(format!("## OpenAI response\n\n {}", response)),
            Err(e) => eprintln!("Error querying OpenAI: {}", e),
        }
    }

    // Anthropic
    if let Ok(claude_key) = env::var("ANTHROPIC_API_KEY") {
        println!("Using Anthropic with key: {}****", &claude_key[0..8]);
        match anthropic::query_anthropic(&query).await {
            Ok(response) => responses.push(format!("## Anthropic response\n\n {}", response)),
            Err(e) => eprintln!("Error querying Claude: {}", e),
        }
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
            "No AI providers configured. Please set one or more of the following: OPENAI_API_KEY, ANTHROPIC_API_KEY, GEMINI_API_KEY, COPILOT_API_KEY.",
        ));
    }


    // Combine all responses
    let combined_response = responses.join("\n");
    Ok(combined_response)
}



// Function to filter, compress, and Base64 encode the frames CSV
fn filter_compress_and_encode_frames(csv_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Filter and clean the CSV
    let filtered_csv = filter_frames_csv(csv_content)?;

    // Step 2: Compress the filtered CSV using gzip
    let compressed_csv = compress_csv(&filtered_csv)?;

    // Step 3: Base64 encode the compressed CSV
    let base64_encoded_csv = base64::engine::general_purpose::STANDARD.encode(&compressed_csv);

    Ok(base64_encoded_csv)
}

// Function to compress CSV content
fn compress_csv(content: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content.as_bytes())?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}


fn filter_frames_csv(csv_content: &str) -> Result<String, Box<dyn Error>> {
    // Read the CSV
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(csv_content.as_bytes());

    // Prepare a writer for the filtered CSV
    let mut wtr = Writer::from_writer(Vec::new());

    // Write headers for the filtered CSV
    wtr.write_record(&[
        "frame",
        "player",
        "boost",
        "location_x",
        "location_y",
        "location_z",
        "rotation_x",
        "rotation_y",
        "rotation_z",
        "rotation_w",
        "angular_velocity_magnitude",
        "linear_velocity_magnitude",
    ])?;

    // Group rows by frame number
    let mut frame_groups: HashMap<String, Vec<csv::StringRecord>> = HashMap::new();
    for result in reader.records() {
        let record = result?;

        // Skip rows where location is (0, 0, 0) unless the player is "_ball_"
        let player_name = record.get(3).unwrap_or("_unknown_");
        let location_x: f64 = record.get(5).unwrap_or("0").parse().unwrap_or(0.0);
        let location_y: f64 = record.get(6).unwrap_or("0").parse().unwrap_or(0.0);
        let location_z: f64 = record.get(7).unwrap_or("0").parse().unwrap_or(0.0);

        if location_x == 0.0 && location_y == 0.0 && location_z == 0.0 && player_name != "_ball_" {
            continue;
        }
        // Add record to the group for the corresponding frame
        let frame = record.get(0).unwrap_or_default().to_string();
        frame_groups.entry(frame).or_default().push(record);
    }

    // Collect and sort all unique frames numerically
    let mut sorted_frames: Vec<_> = frame_groups.keys().cloned().collect();
    sorted_frames.sort_by(|a, b| a.parse::<usize>().unwrap_or(usize::MAX).cmp(&b.parse::<usize>().unwrap_or(usize::MAX)));

    // Process every nth frame
    let nth = 30;
    for (i, frame) in sorted_frames.into_iter().enumerate() {
        if i % nth == 0 {
            if let Some(records) = frame_groups.get(&frame) {
                for record in records {
                    let player_name = record.get(3).unwrap_or("_unknown_");
                    let boost = record.get(4).unwrap_or("0").parse().unwrap_or(0);
                    let location_x: f64 = record.get(5).unwrap_or("0").parse().unwrap_or(0.0);
                    let location_y: f64 = record.get(6).unwrap_or("0").parse().unwrap_or(0.0);
                    let location_z: f64 = record.get(7).unwrap_or("0").parse().unwrap_or(0.0);
                    let rotation_x: f64 = record.get(8).unwrap_or("0").parse().unwrap_or(0.0);
                    let rotation_y: f64 = record.get(9).unwrap_or("0").parse().unwrap_or(0.0);
                    let rotation_z: f64 = record.get(10).unwrap_or("0").parse().unwrap_or(0.0);
                    let rotation_w: f64 = record.get(11).unwrap_or("0").parse().unwrap_or(0.0);

                    let angular_velocity_x: f64 = record.get(12).unwrap_or("0").parse().unwrap_or(0.0);
                    let angular_velocity_y: f64 = record.get(13).unwrap_or("0").parse().unwrap_or(0.0);
                    let angular_velocity_z: f64 = record.get(14).unwrap_or("0").parse().unwrap_or(0.0);

                    let linear_velocity_x: f64 = record.get(15).unwrap_or("0").parse().unwrap_or(0.0);
                    let linear_velocity_y: f64 = record.get(16).unwrap_or("0").parse().unwrap_or(0.0);
                    let linear_velocity_z: f64 = record.get(17).unwrap_or("0").parse().unwrap_or(0.0);

                    // Compute velocity magnitudes
                    let angular_velocity_magnitude = (angular_velocity_x.powi(2)
                        + angular_velocity_y.powi(2)
                        + angular_velocity_z.powi(2))
                    .sqrt();
                    let linear_velocity_magnitude = (linear_velocity_x.powi(2)
                        + linear_velocity_y.powi(2)
                        + linear_velocity_z.powi(2))
                    .sqrt();

                    // Write filtered and aggregated row
                    wtr.write_record(&[
                        frame.clone(),
                        player_name.to_string(),
                        boost.to_string(),
                        location_x.to_string(),
                        location_y.to_string(),
                        location_z.to_string(),
                        rotation_x.to_string(),
                        rotation_y.to_string(),
                        rotation_z.to_string(),
                        rotation_w.to_string(),
                        angular_velocity_magnitude.to_string(),
                        linear_velocity_magnitude.to_string(),
                    ])?;
                }
            }
        }
    }

    // Return the filtered CSV as a String
    let filtered_csv = String::from_utf8(wtr.into_inner()?)?;
    // println!("{}", filtered_csv);
    Ok(filtered_csv)
}
