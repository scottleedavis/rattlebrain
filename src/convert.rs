use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;
use std::io::BufWriter;

// Main function to analyze replay data
pub fn convert_replay(data: Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn parse_frames(data: &Value, file: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
    let empty_array: Vec<Value> = vec![];
    let frames = data.as_array().unwrap_or(&empty_array);
    let mut player_map: HashMap<String, String> = HashMap::new();
    let mut team_map: HashMap<String, String> = HashMap::new();
    let mut car_map: HashMap<String, String> = HashMap::new();
    let mut lines: Vec<String> = Vec::new();
    lines.push("Time,Team,PlayerName,Location_X,Location_Y,Location_Z,Rotation_X,Rotation_Y,Rotation_Z,Rotations_W,AngularVelocity_X,AngularVelocity_Y,AngularVelocity_Z,LinearVelocity_X,LinearVelocity_Y,LinearVelocity_Z".to_string());
    
    for frame in frames {
        // let delta = frame.get("delta").unwrap_or(&Value::Null).to_string();
        let time = frame.get("time").unwrap_or(&Value::Null).to_string();

        if let Some(replications) = frame.get("replications").and_then(|r| r.as_array()) {
            for replication in replications {
                let actor_id = replication
                    .pointer("/actor_id/value")
                    .unwrap_or(&Value::Null)
                    .to_string();

                if let Some(updated) = replication.pointer("/value/updated") {
                    for update in updated.as_array().unwrap_or(&empty_array) {
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
                        if name == "Engine.Pawn:PlayerReplicationInfo" || 
                            name == "TAGame.CarComponent_TA:Vehicle" ||
                            name == "Engine.Pawn:PlayerReplicationInfo"  {
                            if let Some(value_int) = update
                                .get("value")
                                .and_then(|value| value.get("flagged_int"))
                                .and_then(|value| value.get("int"))
                                .and_then(|int_value| int_value.as_i64())

                            {
                                if let Some(_name) = player_map.get(&value_int.to_string()) {
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

                if let Some(spawned) = replication.pointer("/value/spawned") {
                    let obj_name = spawned.get("object_name").unwrap_or(&Value::Null).to_string();
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
                    } else {
                        //look for the ball
                    }
                }

                if let Some(updated) = replication.pointer("/value/updated") {
                    for update in updated.as_array().unwrap_or(&empty_array) {
                        let name = update.get("name").unwrap_or(&Value::Null).to_string();

                        if name == "\"TAGame.RBActor_TA:ReplicatedRBState\"" {

                            // let value = serde_json::to_string(update.get("value").unwrap_or(&Value::Null))
                            //                 .unwrap_or_else(|_| "{}".to_string())
                            //                 .replace("\"", "\\\"");

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
                            } else {
                                // look for the ball
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
