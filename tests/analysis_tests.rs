use rocket_league_replay_ai_analysis::analysis::analyze_replay;
use serde_json::json;
use std::fs;
use std::path::Path;

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
                        ["UnfairTeamSize", { "value": { "int": 1 } }]
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

    assert!(Path::new(&header_file).exists(), "Header file does not exist");
    assert!(Path::new(&goals_file).exists(), "Goals file does not exist");
    assert!(Path::new(&player_stats_file).exists(), "Player stats file does not exist");
    assert!(Path::new(&highlights_file).exists(), "Highlights file does not exist");

//     fs::remove_file(&header_file).expect("Failed to delete header file");
//     fs::remove_file(&goals_file).expect("Failed to delete goals file");
//     fs::remove_file(&player_stats_file).expect("Failed to delete player stats file");
//     fs::remove_file(&highlights_file).expect("Failed to delete highlights file");
}
