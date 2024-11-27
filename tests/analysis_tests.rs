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

    // Ensure files exist
    assert!(Path::new(&header_file).exists(), "Header file does not exist");

    // Read and validate header file content
    let header_content: Value = serde_json::from_str(&fs::read_to_string(&header_file).unwrap())
        .expect("Failed to parse header file");
    assert_eq!(
        header_content["team_size"].as_i64(),
        Some(3),
        "team_size is incorrect"
    );
    assert_eq!(
        header_content["unfair_team_size"].as_i64(),
        Some(1),
        "unfair_team_size is incorrect"
    );
    assert_eq!(
        header_content["team_1_score"].as_i64(),
        Some(4),
        "team_1_score is incorrect"
    );
    assert_eq!(
        header_content["team_0_score"].as_i64(),
        Some(5),
        "team_0_score is incorrect"
    );
    assert_eq!(
        header_content["primary_player_team"].as_i64(),
        Some(0),
        "primary_player_team is incorrect"
    );
}
