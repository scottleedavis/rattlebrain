use rocket_league_replay_ai_analysis::analysis::analyze_replay;
use serde_json::Value;
use std::{fs, path::Path};

#[test]
fn test_analyze_replay_with_valid_json() {
    // Read valid.json file
    let valid_json_path = "tests/valid.json";
    let valid_json_content = fs::read_to_string(valid_json_path)
        .expect("Failed to read valid.json file");

    // Parse JSON content
    let example_json: Value = serde_json::from_str(&valid_json_content)
        .expect("Failed to parse valid.json content");

    // Run analyze_replay
    analyze_replay(example_json).expect("Failed to analyze replay");

    // Define output files
    let output_dir = "./output";
    let match_guid = "383F0B0411EFAC27082CAFA884251EFF";
    let header_file = format!("{}/{}.header.json", output_dir, match_guid);
    let goals_file = format!("{}/{}.goals.json", output_dir, match_guid);
    let player_stats_file = format!("{}/{}.player_stats.json", output_dir, match_guid);
    let highlights_file = format!("{}/{}.highlights.json", output_dir, match_guid);

    // Debug: Print file paths
    println!("Checking for header file at: {}", header_file);

    // Ensure files exist
    assert!(Path::new(&header_file).exists(), "Header file does not exist");
    assert!(Path::new(&goals_file).exists(), "Goals file does not exist");
    assert!(Path::new(&player_stats_file).exists(), "Player stats file does not exist");
    assert!(Path::new(&highlights_file).exists(), "Highlights file does not exist");

    // Debug: Read and validate the header file content
    let header_content = fs::read_to_string(&header_file)
        .expect("Failed to read header file");
    println!("Header content: {}", header_content);
}

// #[test]
// fn test_parse_frames_with_object_names() {
//     let example_json = json!({
//         "body": {
//             "frames": [
//                 {
//                     "delta": 0,
//                     "replications": [
//                         {
//                             "actor_id": { "limit": 2046, "value": 0 },
//                             "value": {
//                                 "spawned": {
//                                     "class_name": "TAGame.CarComponent_Boost_TA:ReplicatedBoost",
//                                     "flag": true,
//                                     "initialization": { "location": null, "rotation": null },
//                                     "name": "BoostComponent",
//                                     "object_id": 1,
//                                     "object_name": "BoostComponent_1"
//                                 },
//                                 "boost": { "boostAmount": 100 }
//                             }
//                         }
//                     ]
//                 }
//             ]
//         }
//     });

//     let frames = parse_frames(&example_json);
//     assert_eq!(frames.len(), 1);
//     assert_eq!(frames[0]["delta"], json!(0));
//     let replications = frames[0]["replications"].as_array().unwrap();
//     assert_eq!(replications.len(), 1);
//     assert_eq!(replications[0]["actor_id"], json!(0));
// }


