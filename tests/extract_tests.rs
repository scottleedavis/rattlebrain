use rocket_league_replay_ai_analysis::extract;

use std::fs::File;
use std::io::Write;

#[test]
fn test_extract_valid_replay() {
    let valid_replay_path = "tests/valid.replay";
    let output_path = "tests/output.json";

    // Call extract_replay
    let result = extract::extract_replay(valid_replay_path, output_path);
    assert!(result.is_ok());

    // Verify the CSV file exists
    let output_contents = std::fs::read_to_string(output_path).unwrap();
    assert!(output_contents.contains("Property,Value"));

    // Clean up
    std::fs::remove_file(valid_replay_path).unwrap();
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_extract_invalid_replay() {
    let invalid_replay_path = "tests/invalid.replay";
    let output_path = "tests/output.csv";

    // Create a dummy invalid replay file
    let mut file = File::create(invalid_replay_path).unwrap();
    file.write_all(b"invalid replay data").unwrap();

    // Call extract_replay
    let result = extract::extract_replay(invalid_replay_path, output_path);
    assert!(result.is_err());

    // Clean up
    std::fs::remove_file(invalid_replay_path).unwrap();
}
