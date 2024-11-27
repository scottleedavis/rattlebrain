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
        data.pointer("/header/body/properties/elements")
            .unwrap_or(&Value::Array(vec![])),
    );
    save_to_file(&Value::Array(goals), output_dir, &match_guid, "goals")?;


    let player_stats = parse_player_stats(
        data.pointer("/header/body/properties/elements")
            .unwrap_or(&Value::Array(vec![])),
    );
    save_to_file(&Value::Array(player_stats), output_dir, &match_guid, "player_stats")?;


    let highlights = parse_highlights(
        data.pointer("/header/body/properties/elements")
            .unwrap_or(&Value::Array(vec![])),
    );
    save_to_file(&Value::Array(highlights), output_dir, &match_guid, "highlights")?;

    // let frames = parse_frames(&data);
    // save_to_file(&Value::Array(frames), output_dir, &match_guid, "frames")?;

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

fn parse_goals(elements: &Value) -> Vec<Value> {
    println!("Debug: Full elements = {:?}", elements); // Debug the input

    // Create a longer-lived empty vector
    let empty_vec = vec![];
    let goals_property = elements
        .as_array()
        .unwrap_or(&empty_vec) // Use the longer-lived empty vector here
        .iter()
        .find(|item| item.get(0).and_then(|v| v.as_str()) == Some("Goals"));

    println!("Debug: Found Goals Property = {:?}", goals_property); // Debug the "Goals" property

    let goals_array = goals_property
        .and_then(|item| item.get(1)) // Access the second element in the "Goals" property
        .and_then(|details| details.get("value")) // Access the "value" field
        .and_then(|value| value.get("array")); // Access the "array" field

    println!("Debug: Extracted Goals Array = {:?}", goals_array); // Debug the raw goals array

    let parsed_goals = goals_array
        .and_then(|array| array.as_array()) // Ensure it's an array
        .map(|array| {
            array
                .iter()
                .filter_map(|goal| {
                    goal.get("elements")
                        .and_then(|elements| elements.as_array())
                        .map(|fields| {
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
                .collect()
        })
        .unwrap_or_default(); // Default to an empty vector if parsing fails

    println!("Debug: Parsed Goals = {:?}", parsed_goals); // Debug the parsed goals

    parsed_goals
}

fn parse_player_stats(elements: &Value) -> Vec<Value> {
    let empty_vec = vec![]; // Create a longer-lived empty vector
    let player_stats_property = elements
        .as_array()
        .unwrap_or(&empty_vec) // Use the longer-lived empty vector
        .iter()
        .find(|item| item.get(0).and_then(|v| v.as_str()) == Some("PlayerStats"));

    let player_stats_array = player_stats_property
        .and_then(|item| item.get(1))
        .and_then(|details| details.get("value"))
        .and_then(|value| value.get("array"));

    player_stats_array
        .and_then(|array| array.as_array())
        .map(|array| {
            array
                .iter()
                .map(|entry| {
                    let mut map = serde_json::Map::new();
                    if let Some(elements) = entry.get("elements").and_then(|v| v.as_array()) {
                        for key in &["Name", "Platform", "Team", "Score", "Goals", "Assists", "Saves", "Shots", "bBot"] {
                            if let Some(value) = elements.iter().find_map(|field| {
                                field.get(0).and_then(|k| {
                                    if k.as_str() == Some(key) {
                                        field.get(1).and_then(|v| v.get("value"))
                                    } else {
                                        None
                                    }
                                })
                            }) {
                                map.insert((*key).to_string(), value.clone());
                            }
                        }
                    }
                    Value::Object(map)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_highlights(elements: &Value) -> Vec<Value> {
    let empty_vec = vec![]; // Create a longer-lived empty vector
    let highlights_property = elements
        .as_array()
        .unwrap_or(&empty_vec) // Use the longer-lived empty vector
        .iter()
        .find(|item| item.get(0).and_then(|v| v.as_str()) == Some("HighLights"));

    let highlights_array = highlights_property
        .and_then(|item| item.get(1))
        .and_then(|details| details.get("value"))
        .and_then(|value| value.get("array"));

    highlights_array
        .and_then(|array| array.as_array())
        .map(|array| {
            array
                .iter()
                .map(|entry| {
                    let mut map = serde_json::Map::new();
                    if let Some(elements) = entry.get("elements").and_then(|v| v.as_array()) {
                        for key in &["frame", "CarName", "BallName", "GoalActorName"] {
                            if let Some(value) = elements.iter().find_map(|field| {
                                field.get(0).and_then(|k| {
                                    if k.as_str() == Some(key) {
                                        field.get(1).and_then(|v| v.get("value"))
                                    } else {
                                        None
                                    }
                                })
                            }) {
                                map.insert((*key).to_string(), value.clone());
                            }
                        }
                    }
                    Value::Object(map)
                })
                .collect()
        })
        .unwrap_or_default()
}


// fn parse_frames(data: &Value) -> Vec<Value> {
//     let frames = data.pointer("/body/frames").unwrap_or(&Value::Array(vec![]));
//     let empty_vec = vec![];
//     let mut actors = serde_json::Map::new();

//     frames
//         .as_array()
//         .unwrap_or(&empty_vec) // Ensure frames is an array
//         .iter()
//         .map(|frame| {
//             let mut frame_map = serde_json::Map::new();

//             // Extract `delta`
//             if let Some(delta) = frame.get("delta") {
//                 frame_map.insert("delta".to_string(), delta.clone());
//             }

//             // Extract `replications`
//             let replications = frame
//                 .get("replications")
//                 .and_then(|r| r.as_array())
//                 .unwrap_or(&empty_vec)
//                 .iter()
//                 .map(|replication| {
//                     let mut replication_map = serde_json::Map::new();

//                     if let Some(actor_id) = replication.pointer("/actor_id/value") {
//                         let actor_id_str = actor_id.to_string();

//                         // Handle `object_name` and its specific cases
//                         if let Some(object_name) = replication.pointer("/value/spawned/class_name").and_then(|v| v.as_str()) {
//                             match object_name {
//                                 "Engine.Actor:RemoteRole" => {
//                                     if let Some(remote_role) = replication.pointer("/value/enum") {
//                                         actors.entry(actor_id_str.clone())
//                                             .or_insert_with(serde_json::Map::new)
//                                             .insert("remote_role".to_string(), remote_role.clone());
//                                     }
//                                 }
//                                 "TAGame.PRI_TA:PersistentCamera" |
//                                 "TAGame.CameraSettingsActor_TA:PRI" |
//                                 "TAGame.Ball_TA:GameEvent" |
//                                 "TAGame.CarComponent_TA:Vehicle" |
//                                 "Engine.Pawn:PlayerReplicationInfo" => {
//                                     if let Some(flagged_int) = replication.pointer("/value/flagged_int/int") {
//                                         actors.entry(actor_id_str.clone())
//                                             .or_insert_with(serde_json::Map::new)
//                                             .entry("parent_ids".to_string())
//                                             .or_insert_with(|| Value::Array(vec![]))
//                                             .as_array_mut()
//                                             .unwrap()
//                                             .push(flagged_int.clone());
//                                     }
//                                 }
//                                 "TAGame.CarComponent_Boost_TA:ReplicatedBoost" => {
//                                     if let Some(boost_amount) = replication.pointer("/value/boost/boostAmount") {
//                                         actors.entry(actor_id_str.clone())
//                                             .or_insert_with(serde_json::Map::new)
//                                             .insert("boost".to_string(), boost_amount.clone());
//                                     }
//                                 }
//                                 // Add more cases here as needed
//                                 _ => {
//                                     // Handle unrecognized cases or pass
//                                     println!("Unrecognized object_name: {}", object_name);
//                                 }
//                             }
//                         }

//                         replication_map.insert("actor_id".to_string(), actor_id.clone());
//                     }

//                     Value::Object(replication_map)
//                 })
//                 .collect::<Vec<Value>>();

//             frame_map.insert("replications".to_string(), Value::Array(replications));
//             Value::Object(frame_map)
//         })
//         .collect()
// }

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
