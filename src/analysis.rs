use serde_json::Value;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;

// Main function to analyze replay data
pub fn analyze_replay(data: Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;

    if filename.ends_with(".header.json") {
        handle_header(&data, filename)?;
    }

    if filename.ends_with(".frames.json") {
        handle_frames(&data, filename)?;
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

    Ok(())
}

// Handle header.json files
fn handle_header(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}_header.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    if let Some(header_data) = data.pointer("/content/header") {
        write!(file, "key,value\n")?;
        for (key, value) in header_data.as_object().unwrap_or(&serde_json::Map::new()) {
            writeln!(file, "{},{}", key, value)?;
        }
    }
    println!("Processed header: {}", filename);
    Ok(())
}

fn handle_frames(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}_frames.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    writeln!(file, "delta,replication_count\n")?;

    // Define a stable default value
    let empty_frames = Value::Array(vec![]);
    let frames = data.pointer("/content/body/frames").unwrap_or(&empty_frames);

    let empty_vec: Vec<Value> = vec![];
    for frame in frames.as_array().unwrap_or(&empty_vec) {
        let delta = frame.get("delta").unwrap_or(&Value::Null).to_string();
        let replication_count = frame
            .get("replications")
            .and_then(|r| r.as_array())
            .map_or(0, |r| r.len());
        writeln!(file, "{},{}", delta, replication_count)?;
    }

    println!("Processed frames: {}", filename);
    Ok(())
}


// Handle goals.json files
fn handle_goals(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}_goals.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;
    let empty_array = Value::Array(vec![]); // Define a constant default value
    writeln!(file, "goal_id,scorer,assist,timestamp\n")?;
    let goals = data.pointer("/content/goals").unwrap_or(&empty_array);
    for goal in goals.as_array().unwrap_or(&vec![]) {
        let goal_id = goal.get("id").unwrap_or(&Value::Null).to_string();
        let scorer = goal.pointer("/scorer/name").unwrap_or(&Value::Null).to_string();
        let assist = goal.pointer("/assist/name").unwrap_or(&Value::Null).to_string();
        let timestamp = goal.get("timestamp").unwrap_or(&Value::Null).to_string();
        writeln!(file, "{},{},{},{}", goal_id, scorer, assist, timestamp)?;
    }
    println!("Processed goals: {}", filename);
    Ok(())
}

// Handle highlights.json files
fn handle_highlights(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}_highlights.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;
    let empty_array = Value::Array(vec![]); // Define a constant default value
    writeln!(file, "highlight_id,player,event,timestamp\n")?;
    let highlights = data.pointer("/content/highlights").unwrap_or(&empty_array);
    for highlight in highlights.as_array().unwrap_or(&vec![]) {
        let highlight_id = highlight.get("id").unwrap_or(&Value::Null).to_string();
        let player = highlight.pointer("/player/name").unwrap_or(&Value::Null).to_string();
        let event = highlight.get("event").unwrap_or(&Value::Null).to_string();
        let timestamp = highlight.get("timestamp").unwrap_or(&Value::Null).to_string();
        writeln!(file, "{},{},{},{}", highlight_id, player, event, timestamp)?;
    }
    println!("Processed highlights: {}", filename);
    Ok(())
}

fn handle_player_stats(data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = format!("output/{}_player_stats.csv", sanitize_filename(filename));
    let mut file = File::create(output_path)?;

    writeln!(file, "player_id,name,goals,assists,saves,shots\n")?;
    let empty_array = Value::Array(vec![]); // Define a constant default value
    let stats = data.pointer("/content/player_stats").unwrap_or(&empty_array); // Use it here
    for player in stats.as_array().unwrap_or(&vec![]) {
        let player_id = player.get("id").unwrap_or(&Value::Null).to_string();
        let name = player.get("name").unwrap_or(&Value::Null).to_string();
        let goals = player.get("goals").unwrap_or(&Value::Null).to_string();
        let assists = player.get("assists").unwrap_or(&Value::Null).to_string();
        let saves = player.get("saves").unwrap_or(&Value::Null).to_string();
        let shots = player.get("shots").unwrap_or(&Value::Null).to_string();
        writeln!(file, "{},{},{},{},{},{}", player_id, name, goals, assists, saves, shots)?;
    }
    println!("Processed player stats: {}", filename);
    Ok(())
}


// Helper to sanitize file names
fn sanitize_filename(filename: &str) -> String {
    filename.replace("/", "_").replace("\\", "_").replace(".", "_")
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