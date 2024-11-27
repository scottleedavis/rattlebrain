use serde_json::{json, Value};
use std::fs;

/// Analyzes the replay and extracts data into structured JSON files.
pub fn analyze_replay(data: Value) -> Result<(), Box<dyn std::error::Error>> {
    let match_guid = find_property(
        data.pointer("/header/body/properties/elements").unwrap_or(&Value::Null),
        "MatchGuid",
    )
    .and_then(|v| v.as_str().map(|s| s.to_string()))
    .unwrap_or_else(|| "unknown_match_guid".to_string());

    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    let header_map = parse_header(&data);
    save_to_file(&header_map, output_dir, &match_guid, "header")?;

    let goals = parse_goals(
        data.pointer("/header/body/properties/Goals/value/array")
            .unwrap_or(&Value::Array(vec![])),
    );
    save_to_file(&Value::Array(goals), output_dir, &match_guid, "goals")?;

    let player_stats = parse_array(
        data.pointer("/header/body/properties/PlayerStats/value/array")
            .unwrap_or(&Value::Array(vec![])),
        &[
            "Name", "Platform", "Team", "Score", "Goals", "Assists", "Saves", "Shots", "bBot",
        ],
    );
    save_to_file(&Value::Array(player_stats), output_dir, &match_guid, "player_stats")?;

    let highlights = parse_array(
        data.pointer("/header/body/properties/HighLights/value/array")
            .unwrap_or(&Value::Array(vec![])),
        &["frame", "CarName", "BallName", "GoalActorName"],
    );
    save_to_file(&Value::Array(highlights), output_dir, &match_guid, "highlights")?;

    Ok(())
}

/// Parses the header into a structured JSON object.
fn parse_header(data: &Value) -> Value {
    let properties = data.pointer("/header/body/properties/elements").unwrap_or(&Value::Null);

    json!({
        "engine_version": data.pointer("/header/body/engine_version").unwrap_or(&Value::Null),
        "team_size": find_property(properties, "TeamSize").unwrap_or(Value::Null),
        "team_1_score": find_property(properties, "Team1Score").unwrap_or(Value::Null),
        "primary_player_team": find_property(properties, "PrimaryPlayerTeam").unwrap_or(Value::Null),
        "licensee_version": data.pointer("/header/body/licensee_version").unwrap_or(&Value::Null),
        "unfair_team_size": find_property(properties, "UnfairTeamSize").unwrap_or(Value::Null),
        "patch_version": data.pointer("/header/body/patch_version").unwrap_or(&Value::Null),
        "team_0_score": find_property(properties, "Team0Score").unwrap_or(Value::Null),
    })
}

fn parse_goals(array: &Value) -> Vec<Value> {
    array
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|goal| {
            goal.get("elements").and_then(|elements| {
                elements.as_array().map(|fields| {
                    let mut goal_map = serde_json::Map::new();
                    for field in fields {
                        if let Some(key) = field.get(0).and_then(|v| v.as_str()) {
                            if let Some(value) = field.get(1).and_then(|v| v.get("value")) {
                                if let Some(int_value) = value.get("int") {
                                    goal_map.insert(key.to_string(), int_value.clone());
                                } else if let Some(str_value) = value.get("str") {
                                    goal_map.insert(key.to_string(), str_value.clone());
                                }
                            }
                        }
                    }
                    Value::Object(goal_map)
                })
            })
        })
        .collect()
}


fn find_property(array: &Value, key: &str) -> Option<Value> {
    array
        .as_array()
        .and_then(|elements| {
            elements.iter().find_map(|e| {
                if e.get(0)?.as_str()? == key {
                    let value = e.get(1)?.get("value")?;
                    value.get("int").or_else(|| value.get("str")).cloned()
                } else {
                    None
                }
            })
        })
}

/// Helper function to parse an array of structured objects.
fn parse_array(array: &Value, keys: &[&str]) -> Vec<Value> {
    array
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|entry| {
            let mut map = serde_json::Map::new();
            for key in keys {
                if let Some(value) = entry.pointer(&format!("/elements/{}", key)) {
                    map.insert((*key).to_string(), value.clone());
                }
            }
            Value::Object(map)
        })
        .collect()
}

/// Helper function to save a JSON object to a file.
fn save_to_file(
    data: &Value,
    output_dir: &str,
    match_guid: &str,
    section: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("{}/{}.{}.json", output_dir, match_guid, section);
    fs::write(&file_path, serde_json::to_string_pretty(data)?)?;
    println!("Saved: {}", file_path);
    Ok(())
}
