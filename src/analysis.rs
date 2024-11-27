pub fn analyze_replay(input: &str) {
    println!("Analyzing replay: {}", input);

    // Placeholder: Add analysis logic
    let analysis_results = vec![
        vec!["Player".to_string(), "Shots".to_string(), "Saves".to_string()],
        vec!["JohnDoe".to_string(), "5".to_string(), "3".to_string()],
    ];

    println!("Analysis Complete!");
    crate::csv_export::export_to_csv(analysis_results, "output.csv");
}

use boxcars::{HeaderProp, Replay};
// use chrono::{DateTime, NaiveDateTime, Utc};

// pub fn get_property_value<'a>(
//     properties: &'a [(String, HeaderProp)],
//     key: &str,
// ) -> Option<&'a HeaderProp> {
//     properties.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
// }

// pub fn extract_primary_player(replay: &Replay) -> String {
//     if let Some(HeaderProp::Str(name)) = get_property_value(&replay.properties, "PrimaryPlayerName") {
//         return name.clone();
//     }
//     "Unknown".to_string()
// }

// pub fn extract_match_type(replay: &Replay) -> String {
//     if let Some(HeaderProp::Str(game_mode)) = get_property_value(&replay.properties, "GameMode") {
//         return game_mode.clone();
//     }
//     "Unknown".to_string()
// }

// pub fn extract_arena(replay: &Replay) -> String {
//     if let Some(HeaderProp::Str(map_name)) = get_property_value(&replay.properties, "MapName") {
//         return map_name.clone();
//     }
//     "Unknown".to_string()
// }

// pub fn extract_platform(replay: &Replay) -> String {
//     if let Some(HeaderProp::Str(platform)) = get_property_value(&replay.properties, "Platform") {
//         return platform.clone();
//     }
//     "Unknown".to_string()
// }

// pub fn extract_date(replay: &Replay) -> String {
//     if let Some(HeaderProp::QWord(timestamp)) = get_property_value(&replay.properties, "Date") {
//         if let Some(naive) = NaiveDateTime::from_timestamp_opt(*timestamp as i64, 0) {
//             let datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);
//             return datetime.format("%Y-%m-%d %H:%M:%S").to_string();
//         }
//     }
//     "Unknown".to_string()
// }
