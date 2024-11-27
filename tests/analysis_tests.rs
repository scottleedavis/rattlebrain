use rocket_league_replay_ai_analysis::analysis::analyze_replay;
use serde_json::{json, Value};
use std::{fs, path::Path};

#[test]
fn test_analyze_replay() {
    let example_json = json!({
        "header": {
            "body": {
                "engine_version": 868,
                "licensee_version": 32,
                "patch_version": 10,
                "properties": {
                    "MatchGuid": { "value": { "str": "test_match_guid" } },
                    "Goals": { "value": { "array": [] } },
                    "PlayerStats": { "value": { "array": [] } },
                    "HighLights": { "value": { "array": [] } },
                    "elements": [
                        ["TeamSize", { "value": { "int": 3 } }],
                        ["UnfairTeamSize", { "value": { "int": 1 } }],
                        ["Team1Score", { "value": { "int": 4 } }],
                        ["Team0Score", { "value": { "int": 5 } }],
                        ["PrimaryPlayerTeam", { "value": { "int": 0 } }]
                    ]
                }
            }
        }
    });

    analyze_replay(example_json).expect("Failed to analyze replay");

    let output_dir = "output";
    let header_file = format!("{}/test_match_guid.header.json", output_dir);
    let goals_file = format!("{}/test_match_guid.goals.json", output_dir);
    let player_stats_file = format!("{}/test_match_guid.player_stats.json", output_dir);
    let highlights_file = format!("{}/test_match_guid.highlights.json", output_dir);

    // Ensure files exist
    assert!(Path::new(&header_file).exists(), "Header file does not exist");
    assert!(Path::new(&goals_file).exists(), "Goals file does not exist");
    assert!(Path::new(&player_stats_file).exists(), "Player stats file does not exist");
    assert!(Path::new(&highlights_file).exists(), "Highlights file does not exist");

    // Read and validate header file content
    let header_content: Value = serde_json::from_str(&fs::read_to_string(&header_file).unwrap())
        .expect("Failed to parse header file");
    assert_eq!(header_content["engine_version"], 868);
    assert_eq!(header_content["licensee_version"], 32);
    assert_eq!(header_content["patch_version"], 10);
    assert_eq!(header_content["team_size"]["int"], 3);
    assert_eq!(header_content["unfair_team_size"]["int"], 1);
    assert_eq!(header_content["team_1_score"],4);
    assert_eq!(header_content["team_0_score"],5);
    assert_eq!(header_content["primary_player_team"],1);

    // Read and validate goals file content (should be empty array)
    let goals_content: Value =
        serde_json::from_str(&fs::read_to_string(&goals_file).unwrap()).expect("Failed to parse goals file");
    assert_eq!(goals_content.as_array().unwrap().len(), 0);

    // Read and validate player stats file content (should be empty array)
    let player_stats_content: Value =
        serde_json::from_str(&fs::read_to_string(&player_stats_file).unwrap())
            .expect("Failed to parse player stats file");
    assert_eq!(player_stats_content.as_array().unwrap().len(), 0);

    // Read and validate highlights file content (should be empty array)
    let highlights_content: Value =
        serde_json::from_str(&fs::read_to_string(&highlights_file).unwrap())
            .expect("Failed to parse highlights file");
    assert_eq!(highlights_content.as_array().unwrap().len(), 0);

    // Cleanup test files (uncomment if cleanup is desired)
    // fs::remove_file(&header_file).expect("Failed to delete header file");
    // fs::remove_file(&goals_file).expect("Failed to delete goals file");
    // fs::remove_file(&player_stats_file).expect("Failed to delete player stats file");
    // fs::remove_file(&highlights_file).expect("Failed to delete highlights file");
}
