use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;

// Main function to analyze replay data
pub fn analyze_replay(data: Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    if filename.ends_with(".header.json") {
        handle_header(&data, filename)?;
    }

    if filename.ends_with(".goals.json") {
        handle_goals(&data, filename)?;
    }

    if filename.ends_with(".highlights.json") {
        handle_highlights(&data, filename)?;
    }

    if filename.ends_with(".player_stats.json") {
        handle_player_stats(&data, filename)?;
    }

    if filename.ends_with(".frames.json") {
        handle_frames(&data, filename)?;
    }

    Ok(())
}

fn sanitize_filename(filename: &str) -> String {
    filename.replace("/", "_").replace("\\", "_").replace(".", "_")
}

fn handle_header(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    // Write CSV header row
    writeln!(
        file,
        "engine_version,licensee_version,patch_version,primary_player_team,team_0_score,team_1_score,team_size,unfair_team_size"
    )?;

    // Extract values and write a single row
    let engine_version = data.get("engine_version").unwrap_or(&Value::Null).to_string();
    let licensee_version = data.get("licensee_version").unwrap_or(&Value::Null).to_string();
    let patch_version = data.get("patch_version").unwrap_or(&Value::Null).to_string();
    let primary_player_team = data.get("primary_player_team").unwrap_or(&Value::Null).to_string();
    let team_0_score = data.get("team_0_score").unwrap_or(&Value::Null).to_string();
    let team_1_score = data.get("team_1_score").unwrap_or(&Value::Null).to_string();
    let team_size = data.get("team_size").unwrap_or(&Value::Null).to_string();
    let unfair_team_size = data.get("unfair_team_size").unwrap_or(&Value::Null).to_string();

    // Write the data row
    writeln!(
        file,
        "{},{},{},{},{},{},{},{}",
        engine_version,
        licensee_version,
        patch_version,
        primary_player_team,
        team_0_score,
        team_1_score,
        team_size,
        unfair_team_size
    )?;

    println!("Processed header: {}", filename);
    Ok(())
}

fn handle_goals(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    // Write CSV header row
    writeln!(file, "PlayerName,PlayerTeam,Frame")?;

    // Define a stable default value for the array
    let empty_array: Vec<Value> = vec![];
    let goals = data.as_array().unwrap_or(&empty_array); // Use the stable reference

    for goal in goals {
        let player_name = goal.get("PlayerName").unwrap_or(&Value::Null).to_string();
        let player_team = goal.get("PlayerTeam").unwrap_or(&Value::Null).to_string();
        let frame = goal.get("frame").unwrap_or(&Value::Null).to_string();

        writeln!(file, "{},{},{}", player_name, player_team, frame)?;
    }

    println!("Processed goals: {}", filename);
    Ok(())
}


fn handle_highlights(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    // Write CSV header row
    writeln!(file, "BallName,CarName,GoalActorName,Frame")?;

    // Define a stable default value for the array
    let empty_array: Vec<Value> = vec![];
    let highlights = data.as_array().unwrap_or(&empty_array);

    for highlight in highlights {
        let ball_name = highlight
            .pointer("/BallName/name")
            .unwrap_or(&Value::Null)
            .to_string();
        let car_name = highlight
            .pointer("/CarName/name")
            .unwrap_or(&Value::Null)
            .to_string();
        let goal_actor_name = highlight
            .pointer("/GoalActorName/name")
            .unwrap_or(&Value::Null)
            .to_string();
        let frame = highlight
            .pointer("/frame/int")
            .unwrap_or(&Value::Null)
            .to_string();

        writeln!(file, "{},{},{},{}", ball_name, car_name, goal_actor_name, frame)?;
    }

    println!("Processed highlights: {}", filename);
    Ok(())
}

fn handle_player_stats(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    // Write CSV header row
    writeln!(
        file,
        "Name,Platform,Goals,Assists,Saves,Score,Shots,Team,bBot"
    )?;

    // Define a stable default value for the array
    let empty_array: Vec<Value> = vec![];
    let players = data.as_array().unwrap_or(&empty_array);

    for player in players {
        let name = player
            .pointer("/Name/str")
            .unwrap_or(&Value::Null)
            .to_string();
        let platform = player
            .pointer("/Platform/byte/1/Right")
            .unwrap_or(&Value::Null)
            .to_string();
        let goals = player.pointer("/Goals/int").unwrap_or(&Value::Null).to_string();
        let assists = player.pointer("/Assists/int").unwrap_or(&Value::Null).to_string();
        let saves = player.pointer("/Saves/int").unwrap_or(&Value::Null).to_string();
        let score = player.pointer("/Score/int").unwrap_or(&Value::Null).to_string();
        let shots = player.pointer("/Shots/int").unwrap_or(&Value::Null).to_string();
        let team = player.pointer("/Team/int").unwrap_or(&Value::Null).to_string();
        let b_bot = player.pointer("/bBot/bool").unwrap_or(&Value::Null).to_string();

        writeln!(
            file,
            "{},{},{},{},{},{},{},{},{}",
            name, platform, goals, assists, saves, score, shots, team, b_bot
        )?;
    }

    println!("Processed player stats: {}", filename);
    Ok(())
}

fn handle_frames(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    parse_frames(data, &mut file)?;

    println!("Processed frames: {}", filename);
    Ok(())
}

use std::io::BufWriter;
pub fn parse_frames(data: &Value, file: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
    let empty_array: Vec<Value> = vec![];
    let frames = data.as_array().unwrap_or(&empty_array);
    let mut player_map: HashMap<String, String> = HashMap::new();
    let mut team_map: HashMap<String, String> = HashMap::new();
    let mut car_map: HashMap<String, String> = HashMap::new();
    let mut lines: Vec<String> = Vec::new();
    lines.push("Time,Team,PlayerName,Location_X,Location_Y,Location_Z,Rotation_X,Rotation_Y,Rotation_Z,Rotations_W,AngularVelocity_X,AngularVelocity_Y,AngularVelocity_Z,LinearVelocity_X,LinearVelocity_Y,LinearVelocity_Z".to_string());
    
    for frame in frames {
        let delta = frame.get("delta").unwrap_or(&Value::Null).to_string();
        let time = frame.get("time").unwrap_or(&Value::Null).to_string();

        if let Some(replications) = frame.get("replications").and_then(|r| r.as_array()) {
            for replication in replications {
                let actor_id = replication
                    .pointer("/actor_id/value")
                    .unwrap_or(&Value::Null)
                    .to_string();
                let limit = replication
                    .pointer("/actor_id/limit")
                    .unwrap_or(&Value::Null)
                    .to_string();

                if let Some(updated) = replication.pointer("/value/updated") {
                    for update in updated.as_array().unwrap_or(&empty_array) {
                        let component = "updated".to_string();
                        let name = update.get("name").unwrap_or(&Value::Null).as_str().unwrap_or("");
                        if name == "Engine.PlayerReplicationInfo:PlayerName" {
                            if let Some(value_string) = update
                                .get("value")
                                .and_then(|value| value.get("string"))
                                .and_then(|string_value| string_value.as_str())
                            {
                                player_map.insert(actor_id.clone(), value_string.to_string());
                            }
                        }
                        if name == "Engine.PlayerReplicationInfo:Team" {
                            if let Some(value_int) = update
                                .get("value")
                                .and_then(|value| value.get("flagged_int"))
                                .and_then(|flagged_int| flagged_int.get("int"))
                                .and_then(|int_value| int_value.as_i64()) 
                            {
                                team_map.insert(actor_id.clone(), value_int.to_string());
                            }

                        }
                        if (name == "Engine.Pawn:PlayerReplicationInfo" || 
                        name == "TAGame.CarComponent_TA:Vehicle" ||
                        name == "Engine.Pawn:PlayerReplicationInfo" ) {
                            if let Some(value_int) = update
                                .get("value")
                                .and_then(|value| value.get("flagged_int"))
                                .and_then(|value| value.get("int"))
                                .and_then(|int_value| int_value.as_i64())

                            {
                                if let Some(name) = player_map.get(&value_int.to_string()) {
                                    car_map.insert(actor_id.clone(),value_int.to_string() );
                                }

                            }
                        }
                    }
                }
            }
        }

        if let Some(replications) = frame.get("replications").and_then(|r| r.as_array()) {
            for replication in replications {
                let actor_id = replication
                    .pointer("/actor_id/value")
                    .unwrap_or(&Value::Null)
                    .to_string();
                let limit = replication
                    .pointer("/actor_id/limit")
                    .unwrap_or(&Value::Null)
                    .to_string();

                if let Some(spawned) = replication.pointer("/value/spawned") {
                    let component = "spawned".to_string();
                    let name = spawned.get("class_name").unwrap_or(&Value::Null).to_string();
                    let obj_name = spawned.get("object_name").unwrap_or(&Value::Null).to_string();
                    let obj_id = spawned.get("object_id").unwrap_or(&Value::Null).to_string();
                    if let Some(cname) = car_map.get(&actor_id).map(String::as_str) {
                        if obj_name == "\"Archetypes.Car.Car_Default\""{
                            let pname = player_map.get(cname).map(String::as_str).unwrap_or("Unknown");
                            let tname = team_map.get(cname).map(String::as_str).unwrap_or("Unknown");
                            let location_x = spawned.pointer("/initialization/location/x")
                                .and_then(Value::as_i64)
                                .unwrap_or(0);
                            let location_y = spawned.pointer("/initialization/location/y")
                                .and_then(Value::as_i64)
                                .unwrap_or(0);
                            let location_z = spawned.pointer("/initialization/location/z")
                                .and_then(Value::as_i64)
                                .unwrap_or(0);

                            let rotation_x = spawned.pointer("/initialization/rotation/x")
                                .and_then(Value::as_f64)
                                .unwrap_or(0.0);
                            let rotation_y = spawned.pointer("/initialization/rotation/y")
                                .and_then(Value::as_f64)
                                .unwrap_or(0.0);
                            let rotation_z = spawned.pointer("/initialization/rotation/z")
                                .and_then(Value::as_f64)
                                .unwrap_or(0.0);

                            lines.push(format!(
                                "{},{},\"{}\",{},{},{},{},{},{},0.0,0,0,0,0,0,0,0",
                                time, tname, pname, 
                                location_x, location_y, location_z, 
                                rotation_x, rotation_y, rotation_z
                            ));
                        }
                    }
                }

                // Handle `updated` components
                if let Some(updated) = replication.pointer("/value/updated") {
                    for update in updated.as_array().unwrap_or(&empty_array) {
                        let component = "updated".to_string();
                        let name = update.get("name").unwrap_or(&Value::Null).to_string();

                        if name == "\"TAGame.RBActor_TA:ReplicatedRBState\"" {

                            let value = serde_json::to_string(update.get("value").unwrap_or(&Value::Null))
                                            .unwrap_or_else(|_| "{}".to_string())
                                            .replace("\"", "\\\"");

                            if let Some(cname) = car_map.get(&actor_id).map(String::as_str) {
                                if cname != "Unknown" {
                                    let pname = player_map.get(cname).map(String::as_str).unwrap_or("Unknown");
                                    let tname = team_map.get(cname).map(String::as_str).unwrap_or("Unknown");

                                    let location_x = update.pointer("/value/rigid_body_state/location/x")
                                        .and_then(Value::as_i64)
                                        .unwrap_or(0);
                                    let location_y = update.pointer("/value/rigid_body_state/location/y")
                                        .and_then(Value::as_i64)
                                        .unwrap_or(0);
                                    let location_z = update.pointer("/value/rigid_body_state/location/z")
                                        .and_then(Value::as_i64)
                                        .unwrap_or(0);

                                    let rotation_x = update.pointer("/value/rigid_body_state/rotation/quaternion/x")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);
                                    let rotation_y = update.pointer("/value/rigid_body_state/rotation/quaternion/y")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);
                                    let rotation_z = update.pointer("/value/rigid_body_state/rotation/quaternion/z")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);
                                    let rotation_w = update.pointer("/value/rigid_body_state/rotation/quaternion/w")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);

                                    let angular_velocity_x = update.pointer("/value/rigid_body_state/angular_velocity/x")
                                        .and_then(Value::as_i64)
                                        .unwrap_or(0);
                                    let  angular_velocity_y = update.pointer("/value/rigid_body_state/angular_velocity/y")
                                        .and_then(Value::as_i64)
                                        .unwrap_or(0);
                                    let  angular_velocity_z = update.pointer("/value/rigid_body_state/angular_velocity/z")
                                        .and_then(Value::as_i64)
                                        .unwrap_or(0);

                                    let linear_velocity_x = update.pointer("/value/rigid_body_state/linear_velocity/x")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);
                                    let linear_velocity_y = update.pointer("/value/rigid_body_state/linear_velocity/y")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);
                                    let linear_velocity_z = update.pointer("/value/rigid_body_state/linear_velocity/z")
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0);

                                    lines.push(format!(
                                         "{},{},\"{}\",{},{},{},{},{},{},{},{},{},{},{},{},{}",
                                        time, tname, pname,
                                        location_x, location_y, location_z,
                                        rotation_x, rotation_y, rotation_z, rotation_w,
                                        angular_velocity_x, angular_velocity_y, angular_velocity_z,
                                        linear_velocity_x, linear_velocity_y, linear_velocity_z
                                     ));
                                }

                            } 
                        }
                    }
                }
            }
        }
    }

    let mut writer = BufWriter::new(file);
    for line in &lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}


// // Parses the frames data from the replay JSON.
// pub fn parse_frames(data: &Value) -> Vec<Value> {
//     let empty_frames = Value::Array(vec![]); 
//     let frames = data.pointer("/content/body/frames").unwrap_or(&empty_frames);
//     let empty_vec = vec![];
//     let mut actors = serde_json::Map::new();
//     let mut unhandled_keys: HashSet<String> = HashSet::new(); 

//     frames
//         .as_array()
//         .unwrap_or(&empty_vec) 
//         .iter()
//         .map(|frame| {
//             let mut frame_map = serde_json::Map::new();

//             if let Some(delta) = frame.get("delta") {
//                 frame_map.insert("delta".to_string(), delta.clone());
//             }

//             let replications = frame
//                 .get("replications")
//                 .and_then(|r| r.as_array())
//                 .unwrap_or(&empty_vec)
//                 .iter()
//                 .map(|replication| {
//                     let mut replication_map = serde_json::Map::new();

//                     if let Some(actor_id) = replication.pointer("/actor_id/value") {
//                         let actor_id_str = actor_id.as_str().unwrap_or_default().to_string();
//                         replication_map.insert("actor_id".to_string(), actor_id.clone());

//                         if let Some(object_name) = replication.pointer("/value/spawned/class_name").and_then(|v| v.as_str()) {
//                             // println!("matched spawn {}", object_name);
//                             match object_name {
//                                 // Boost pickup
//                                 "TAGame.VehiclePickup_Boost_TA" => {
//                                     if let Some(location) = replication.pointer("/value/location") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("pickup_location".to_string(), location.clone());
//                                     }
//                                 }
//                                 // Camera settings
//                                 "TAGame.CameraSettingsActor_TA" => {
//                                     if let Some(camera_settings) = replication.pointer("/value/settings") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("camera_settings".to_string(), camera_settings.clone());
//                                     }
//                                 }
//                                 // Player replication info
//                                 "TAGame.PRI_TA" => {
//                                     if let Some(player_data) = replication.pointer("/value/player_data") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("player_data".to_string(), player_data.clone());
//                                     }
//                                 }
//                                 // Game rules
//                                 "TAGame.GRI_TA" => {
//                                     if let Some(rules) = replication.pointer("/value/rules") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("game_rules".to_string(), rules.clone());
//                                     }
//                                 }
//                                 // NetModeReplicator
//                                 "ProjectX.NetModeReplicator" => {
//                                     if let Some(net_mode) = replication.pointer("/value/net_mode") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("net_mode".to_string(), net_mode.clone());
//                                     }
//                                 }
//                                 // Car
//                                 "TAGame.Car_TA" => {
//                                     if let Some(car_data) = replication.pointer("/value/car_data") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("car_data".to_string(), car_data.clone());
//                                     }
//                                 }
//                                 // GodBall Game Event
//                                 "TAGame.GameEvent_GodBall_TA" => {
//                                     if let Some(event_data) = replication.pointer("/value/event_data") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("event_data".to_string(), event_data.clone());
//                                     }
//                                 }
//                                 // Rumble pickups
//                                 "TAGame.RumblePickups_TA" => {
//                                     if let Some(pickups) = replication.pointer("/value/pickups") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("rumble_pickups".to_string(), pickups.clone());
//                                     }
//                                 }
//                                 // Car components
//                                 "TAGame.CarComponent_DoubleJump_TA"
//                                 | "TAGame.CarComponent_Dodge_TA"
//                                 | "TAGame.CarComponent_Jump_TA"
//                                 | "TAGame.CarComponent_FlipCar_TA"
//                                 | "TAGame.CarComponent_Boost_TA" => {
//                                     if let Some(component_data) = replication.pointer("/value/component_data") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("car_component".to_string(), component_data.clone());
//                                     }
//                                 }
//                                 "TAGame.Team_Soccar_TA" => {
//                                     if let Some(team_data) = replication.pointer("/value/team_data") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("team_data".to_string(), team_data.clone());
//                                     }
//                                     if let Some(team_score) = replication.pointer("/value/team_score") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("team_score".to_string(), team_score.clone());
//                                     }
//                                 }
//                                 // Ball (God Mode)
//                                 "TAGame.Ball_God_TA" => {
//                                     if let Some(ball_data) = replication.pointer("/value/ball_data") {
//                                         actors
//                                             .entry(actor_id_str.clone())
//                                             .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             .as_object_mut()
//                                             .unwrap()
//                                             .insert("ball_data".to_string(), ball_data.clone());
//                                     }
//                                 }
//                                 // Default case for unhandled object names
//                                 _ => {
//                                     if unhandled_keys.insert(object_name.to_string()) {
//                                         println!("Unhandled spawned key: {}", object_name); // Only print new unhandled keys
//                                     }
//                                 }
//                             }
//                         }

//                         if let Some(updated) = replication.pointer("/value/updated").and_then(|v|v.as_array()) {
//                             for update in updated {
//                                 if let Some(object_name) = update.get("name").and_then(|v|v.as_str()) {
//                                     match object_name {

//                                         // Example of adding the new keys
//                                         "TAGame.VehiclePickup_TA:bNoPickup" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("bNoPickup".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.VehiclePickup_TA:NewReplicatedPickupData" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("NewReplicatedPickupData".to_string(), attribute_value.clone());
//                                         }
//                                         "Engine.Actor:RemoteRole" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("RemoteRole".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.CameraSettingsActor_TA:CameraYaw" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("CameraYaw".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.CameraSettingsActor_TA:CameraPitch" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("CameraPitch".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.CameraSettingsActor_TA:bUsingSecondaryCamera" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("bUsingSecondaryCamera".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.CameraSettingsActor_TA:ProfileSettings" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("ProfileSettings".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.CameraSettingsActor_TA:PRI" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("PRI".to_string(), attribute_value.clone());
//                                         }
//                                         "Engine.PlayerReplicationInfo:Ping" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("Ping".to_string(), attribute_value.clone());
//                                         }
//                                         "Engine.PlayerReplicationInfo:PlayerName" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("PlayerName".to_string(), attribute_value.clone());
//                                         }
//                                         "Engine.PlayerReplicationInfo:Team" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("Team".to_string(), attribute_value.clone());
//                                         }
//                                         "Engine.PlayerReplicationInfo:PlayerID" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("UniqueId".to_string(), attribute_value.clone());
//                                         }
//                                         "Engine.PlayerReplicationInfo:UniqueId" => {
//                                             // actors
//                                             //     .entry(actor_id_str.clone())
//                                             //     .or_insert_with(|| Value::Object(serde_json::Map::new()))
//                                             //     .as_object_mut()
//                                             //     .unwrap()
//                                             //     .insert("UniqueId".to_string(), attribute_value.clone());
//                                         }
//                                         "TAGame.PRI_TA:SpectatorShortcut" => {}
//                                         "TAGame.PRI_TA:SteeringSensitivity" => {}
//                                         "TAGame.PRI_TA:Title" => {}
//                                         "TAGame.PRI_TA:PersistentCamera" => {}
//                                         "TAGame.PRI_TA:ClientLoadoutsOnline" => {}
//                                         "TAGame.PRI_TA:ClientLoadouts" => {}
//                                         "TAGame.PRI_TA:ReplicatedGameEvent" => {}
//                                         "TAGame.PRI_TA:PlayerHistoryValid" => {}
//                                         "Engine.GameReplicationInfo:GameClass" => {}
//                                         "Engine.GameReplicationInfo:ServerName" => {}
//                                         "ProjectX.GRI_X:MatchGuid" => {}
//                                         "ProjectX.GRI_X:bGameStarted" => {}
//                                         "ProjectX.GRI_X:GameServerID" => {}
//                                         "ProjectX.GRI_X:Reservations" => {}
//                                         "ProjectX.GRI_X:ReplicatedServerRegion" => {}
//                                         "ProjectX.GRI_X:ReplicatedGamePlaylist" => {}
//                                         "TAGame.PRI_TA:CurrentVoiceRoom" => {}
//                                         "TAGame.PRI_TA:ClubID" => {}
//                                         "TAGame.PRI_TA:PlayerHistoryKey" => {}
//                                         "Engine.Pawn:PlayerReplicationInfo" => {}
//                                         "TAGame.RBActor_TA:ReplicatedRBState" => {}
//                                         "TAGame.Vehicle_TA:ReplicatedThrottle" => {}
//                                         "TAGame.Car_TA:RumblePickups" => {}
//                                         "TAGame.Car_TA:TeamPaint" => {}
//                                         "TAGame.GameEvent_TA:MatchStartEpoch" => {}
//                                         "TAGame.GameEvent_TA:ReplicatedStateName" => {}
//                                         "TAGame.GameEvent_TA:BotSkill" => {}
//                                         "TAGame.GameEvent_TA:bHasLeaveMatchPenalty" => {}
//                                         "TAGame.GameEvent_TA:MatchTypeClass" => {}
//                                         "TAGame.GameEvent_Team_TA:MaxTeamSize" => {}
//                                         "TAGame.GameEvent_Soccar_TA:SecondsRemaining" => {}
//                                         "TAGame.GameEvent_Soccar_TA:MaxScore" => {}
//                                         "TAGame.Team_TA:GameEvent" => {}
//                                         "TAGame.CarComponent_TA:Vehicle" => {}
//                                         "TAGame.CarComponent_Boost_TA:UnlimitedBoostRefCount" => {}
//                                         "TAGame.CarComponent_Boost_TA:ReplicatedBoost" => {}
//                                         "TAGame.Ball_TA:GameEvent" => {}
//                                         "Engine.PlayerReplicationInfo:RemoteUserData" => {}
//                                         "TAGame.PRI_TA:PartyLeader" => {}
//                                         "TAGame.GameEvent_TA:ReplicatedRoundCountDownNumber" => {}
//                                         "TAGame.GameEvent_TA:ReplicatedGameStateTimeRemaining" => {}
//                                         "TAGame.Vehicle_TA:ReplicatedSteer" => {}
//                                         "TAGame.Vehicle_TA:bDriving" => {}
//                                         "TAGame.GameEvent_Soccar_TA:RoundNum" => {}
//                                         "TAGame.CarComponent_TA:ReplicatedActive" => {}
//                                         "TAGame.Vehicle_TA:bReplicatedHandbrake" => {}
//                                         "TAGame.GameEvent_Soccar_TA:bBallHasBeenHit" => {}
//                                         "TAGame.Ball_TA:HitTeamNum" => {}
//                                         "TAGame.Ball_God_TA:TargetSpeed" => {}
//                                         "TAGame.CarComponent_Dodge_TA:DodgeTorque" => {}
//                                         "TAGame.CarComponent_DoubleJump_TA:DoubleJumpImpulse" => {}
//                                         "TAGame.PRI_TA:MatchScore" => {}
//                                         "TAGame.GameEvent_Soccar_TA:ReplicatedStatEvent" => {}
//                                         "TAGame.PRI_TA:MatchSaves" => {}
//                                         "TAGame.PRI_TA:MatchShots" => {}
//                                         "Engine.Actor:bCollideActors" => {}
//                                         "Engine.Actor:bHidden" => {}
//                                         "Engine.Actor:bBlockActors" => {}
//                                         "TAGame.Car_TA:ReplicatedDemolishGoalExplosion" => {}
//                                         "Engine.TeamInfo:Score" => {}
//                                         "TAGame.Ball_TA:ReplicatedExplosionDataExtended" => {}
//                                         "Engine.PlayerReplicationInfo:Score" => {}
//                                         "TAGame.PRI_TA:MatchGoals" => {}
//                                         "TAGame.GameEvent_Soccar_TA:ReplicatedScoredOnTeam" => {}
//                                         "TAGame.PRI_TA:MatchAssists" => {}
//                                         "TAGame.PRI_TA:bReady" => {}
//                                         "TAGame.GameEvent_TA:bCanVoteToForfeit" => {}
//                                         "TAGame.CarComponent_FlipCar_TA:FlipCarTime" => {}
//                                         "TAGame.CarComponent_FlipCar_TA:bFlipRight" => {}
//                                         "TAGame.CameraSettingsActor_TA:bUsingBehindView" => {}
//                                         "TAGame.RBActor_TA:bFrozen" => {}
//                                         "TAGame.PRI_TA:SecondaryTitle" => {}
//                                         "TAGame.PRI_TA:PrimaryTitle" => {}
//                                         "TAGame.Team_Soccar_TA:GameScore" => {}
//                                          // Default case for unhandled object names
//                                         _ => {
//                                             if unhandled_keys.insert(object_name.to_string()) {
//                                                 println!("Unhandled updated key: {}", object_name); // Only print new unhandled keys
//                                             }
//                                         }
//                                     }
//                                 }

//                             }
                            
//                         }
//                     }

//                     Value::Object(replication_map)
//                 })
//                 .collect::<Vec<Value>>();

//             frame_map.insert("replications".to_string(), Value::Array(replications));
//             Value::Object(frame_map)
//         })
//         .filter(|frame| !frame.is_null())
//         .collect()
// }