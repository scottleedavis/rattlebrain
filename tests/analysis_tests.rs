use rocket_league_replay_ai_analysis::analysis::{analyze_replay};
use serde_json::Value;
use std::{fs, path::Path};

#[test]
fn test_analyze_replay_with_valid_json() {
    // Read valid.json file
    let valid_json_path = "tests/valid.json";
    let valid_json_content = fs::read_to_string(valid_json_path)
        .expect("Failed to read valid.json file");

    // Parse JSON content
    let example_json: Value = serde_json::from_str(&valid_json_content)
        .expect("Failed to parse valid.json content");

    // Run analyze_replay
    analyze_replay(example_json).expect("Failed to analyze replay");

    // Define output files
    let output_dir = "./output";
    let match_guid = "383F0B0411EFAC27082CAFA884251EFF";
    let header_file = format!("{}/{}.header.json", output_dir, match_guid);
    let goals_file = format!("{}/{}.goals.json", output_dir, match_guid);
    let player_stats_file = format!("{}/{}.player_stats.json", output_dir, match_guid);
    let highlights_file = format!("{}/{}.highlights.json", output_dir, match_guid);
    let frames_file = format!("{}/{}.frames.json", output_dir, match_guid);

    // Ensure files exist
    assert!(Path::new(&header_file).exists(), "Header file does not exist");
    assert!(Path::new(&goals_file).exists(), "Goals file does not exist");
    assert!(Path::new(&player_stats_file).exists(), "Player stats file does not exist");
    assert!(Path::new(&highlights_file).exists(), "Highlights file does not exist");
    assert!(Path::new(&frames_file).exists(), "Frames file does not exist");

    // Debug: Read and validate the header file content
    // let header_content = fs::read_to_string(&header_file)
    //     .expect("Failed to read header file");
    // println!("Header content: {}", header_content);

    // Cleanup: Remove all output files after the test
    let output_files = vec![
        header_file,
        goals_file,
        player_stats_file,
        highlights_file,
        frames_file,
    ];

    for file in output_files {
        if Path::new(&file).exists() {
            fs::remove_file(&file).expect("Failed to delete output file");
        }
    }

}
