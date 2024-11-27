use serde_json::Value;
use std::collections::HashMap;
use std::fs;

/// Analyzes the replay and extracts data into structured JSON files.
/// Returns a Result to propagate errors if any occur.
pub fn analyze_replay(data: Value) -> Result<(), Box<dyn std::error::Error>> {
    // Match GUID for file naming
    let match_guid = data
        .pointer("/header/body/properties/MatchGuid/value/str")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown_match_guid");

    // Ensure the output directory exists
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    // Parse Header
    let header_map = parse_header(&data);
    save_to_file(&header_map, output_dir, match_guid, "header")?;

    // Parse and save Goals
    let goals = parse_array(
        data.pointer("/header/body/properties/Goals/value/array")
            .unwrap_or(&Value::Array(vec![])),
        &["frame", "PlayerName", "PlayerTeam"],
    );
    save_to_file(&goals, output_dir, match_guid, "goals")?;

    // Parse and save PlayerStats
    let player_stats = parse_array(
        data.pointer("/header/body/properties/PlayerStats/value/array")
            .unwrap_or(&Value::Array(vec![])),
        &[
            "Name", "Platform", "Team", "Score", "Goals", "Assists", "Saves", "Shots", "bBot",
        ],
    );
    save_to_file(&player_stats, output_dir, match_guid, "player_stats")?;

    // Parse and save Highlights
    let highlights = parse_array(
        data.pointer("/header/body/properties/HighLights/value/array")
            .unwrap_or(&Value::Array(vec![])),
        &["frame", "CarName", "BallName", "GoalActorName"],
    );
    save_to_file(&highlights, output_dir, match_guid, "highlights")?;

    Ok(())
}

/// Parses the header into a structured HashMap.
fn parse_header(data: &Value) -> HashMap<String, Value> {
    let mut header_map = HashMap::new();
    let properties = data.pointer("/header/body/properties/elements").unwrap_or(&Value::Null);

    header_map.insert(
        "engine_version".to_string(),
        data.pointer("/header/body/engine_version")
            .cloned()
            .unwrap_or(Value::Null),
    );
    header_map.insert(
        "licensee_version".to_string(),
        data.pointer("/header/body/licensee_version")
            .cloned()
            .unwrap_or(Value::Null),
    );
    header_map.insert(
        "patch_version".to_string(),
        data.pointer("/header/body/patch_version")
            .cloned()
            .unwrap_or(Value::Null),
    );
    header_map.insert(
        "team_size".to_string(),
        find_property(properties, "TeamSize").unwrap_or(Value::Null),
    );
    header_map.insert(
        "unfair_team_size".to_string(),
        find_property(properties, "UnfairTeamSize").unwrap_or(Value::Null),
    );
    header_map.insert(
        "primary_player_team".to_string(),
        find_property(properties, "PrimaryPlayerTeam").unwrap_or(Value::Null),
    );
    header_map.insert(
        "team_0_score".to_string(),
        find_property(properties, "Team0Score").unwrap_or(Value::Null),
    );
    header_map.insert(
        "team_1_score".to_string(),
        find_property(properties, "Team1Score").unwrap_or(Value::Null),
    );

    header_map
}

/// Helper function to find a property by its name.
fn find_property(array: &Value, key: &str) -> Option<Value> {
    array
        .as_array()
        .and_then(|elements| {
            elements.iter().find_map(|e| {
                if e.get(0)?.as_str()? == key {
                    Some(e.get(1)?.get("value")?.clone())
                } else {
                    None
                }
            })
        })
}

/// Parses an array of elements and extracts specified keys into JSON objects.
fn parse_array(array: &Value, keys: &[&str]) -> Vec<Value> {
    array
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| {
            let mut obj = serde_json::Map::new();
            for &key in keys {
                obj.insert(
                    key.to_string(),
                    item.pointer(&format!("/elements/{}", key))
                        .cloned()
                        .unwrap_or(Value::Null),
                );
            }
            Value::Object(obj)
        })
        .collect()
}

/// Helper function to save a JSON object to a file.
fn save_to_file(
    data: &impl serde::Serialize,
    output_dir: &str,
    match_guid: &str,
    section: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("{}/{}.{}.json", output_dir, match_guid, section);
    fs::write(&file_path, serde_json::to_string_pretty(data)?)?;
    println!("Saved: {}", file_path);
    Ok(())
}
