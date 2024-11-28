use serde_json::{json, Value};
use std::fs;
use std::collections::HashSet;


/// Analyzes the replay and extracts data into structured JSON files.
pub fn analyze_replay(data: Value) -> Result<(), Box<dyn std::error::Error>> {
    // let match_guid = find_property(
    //     data.pointer("/header/body/properties/elements").unwrap_or(&Value::Null),
    //     "MatchGuid",
    // )
    // .and_then(|v| v.as_str().map(|s| s.to_string()))
    // .unwrap_or_else(|| "unknown_match_guid".to_string());

    // let output_dir = "output";
    // fs::create_dir_all(output_dir)?;
    Ok(())
}
