use rocket_league_replay_ai_analysis::analysis::analyze_replay;
use serde_json::Value;
use std::{fs, path::Path};

#[test]
fn test_analyze_replay_with_valid_json() {
    // Read the valid.json file
    let valid_json_path = "tests/valid.json";
    let valid_json_content = fs::read_to_string(valid_json_path)
        .expect("Failed to read valid.json file");

    // Parse the JSON content
    let example_json: Value = serde_json::from_str(&valid_json_content)
        .expect("Failed to parse valid.json content");

    // Analyze the replay
    analyze_replay(example_json).expect("Failed to analyze replay");

    // Define output directory and expected files
    let output_dir = "output";
    let match_guid = "test_match_guid"; // Ensure this matches the MatchGuid in valid.json
    let header_file = format!("{}/{}.header.json", output_dir, match_guid);
    let goals_file = format!("{}/{}.goals.json", output_dir, match_guid);
    let player_stats_file = format!("{}/{}.player_stats.json", output_dir, match_guid);
    let highlights_file = format!("{}/{}.highlights.json", output_dir, match_guid);

    // Ensure files exist
    assert!(Path::new(&header_file).exists(), "Header file does not exist");
    assert!(Path::new(&goals_file).exists(), "Goals file does not exist");
    assert!(Path::new(&player_stats_file).exists(), "Player stats file does not exist");
    assert!(Path::new(&highlights_file).exists(), "Highlights file does not exist");

    // Optionally, read and validate the content of the output files
    // For example, to validate the header file content:
    let header_content: Value = serde_json::from_str(&fs::read_to_string(&header_file).unwrap())
        .expect("Failed to parse header file");
    assert_eq!(header_content["engine_version"], 868);
    assert_eq!(header_content["licensee_version"], 32);
    assert_eq!(header_content["patch_version"], 10);
    assert_eq!(header_content["team_size"], 3);
    assert_eq!(header_content["unfair_team_size"], 1);
    assert_eq!(header_content["team_1_score"], 4);
    assert_eq!(header_content["team_0_score"], 5);
    assert_eq!(header_content["primary_player_team"], 0);

    // Similar validations can be added for goals, player_stats, and highlights
}
