use rocket_league_replay_ai_analysis::extract;
use std::process::Command;
use std::fs;
use jsonschema::{JSONSchema, Draft};
use serde_json::Value;

use std::fs::File;
use std::io::Write;

#[test]
fn test_extract_replay_json_schema_validation() {
    // Paths to the rattletrap binary, input replay, and output JSON
    let rattletrap_path = "./rattletrap"; // Adjust this path as needed
    let input_replay = "tests/valid.replay"; // Provide a valid replay file for testing
    let output_json = "tests/output.json";

    // Run rattletrap to extract replay data to JSON
    let output_status = Command::new(rattletrap_path)
        .arg("--input")
        .arg(input_replay)
        .arg("--output")
        .arg(output_json)
        .output()
        .expect("Failed to execute rattletrap");

    assert!(
        output_status.status.success(),
        "Rattletrap failed with error: {}",
        String::from_utf8_lossy(&output_status.stderr)
    );

    // Load the generated JSON schema
    let schema_str = fs::read_to_string("replay_schema.json")
        .expect("Failed to read replay_schema.json");
    let schema_json: Value = serde_json::from_str(&schema_str)
        .expect("Invalid JSON schema");

    // Compile the JSON schema
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .expect("Invalid JSON schema");

    // Load the extracted JSON data
    let data_str = fs::read_to_string(output_json)
        .expect("Failed to read output.json");
    let data_json: Value = serde_json::from_str(&data_str)
        .expect("Invalid JSON data");

    // Validate the JSON data against the schema
    if let Err(errors) = compiled_schema.validate(&data_json) {
        for error in errors {
            println!("Validation error: {}", error);
        }
        panic!("JSON data does not conform to the schema");
    }

    // Clean up
    fs::remove_file(output_json).expect("Failed to remove output.json");
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
