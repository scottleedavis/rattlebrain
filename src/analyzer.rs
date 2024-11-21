use boxcars::{HeaderProp, Replay};
use chrono::{DateTime, NaiveDateTime, Utc};

/// Public function to extract property values by key
pub fn get_property_value<'a>(
    properties: &'a [(String, HeaderProp)],
    key: &str,
) -> Option<&'a HeaderProp> {
    properties.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
}

/// Extract the primary player's name from the replay
pub fn extract_primary_player(replay: &Replay) -> String {
    if let Some(HeaderProp::Str(name)) = get_property_value(&replay.properties, "PlayerName") {
        return name.clone();
    }
    "Unknown".to_string()
}

/// Extract the match type from the replay
pub fn extract_match_type(replay: &Replay) -> String {
    if let Some(HeaderProp::Str(game_mode)) = get_property_value(&replay.properties, "GameMode") {
        return game_mode.clone();
    }
    "Unknown".to_string()
}

/// Extract the arena name from the replay
pub fn extract_arena(replay: &Replay) -> String {
    if let Some(HeaderProp::Str(map_name)) = get_property_value(&replay.properties, "MapName") {
        return map_name.clone();
    }
    "Unknown".to_string()
}

/// Extract the platform information from the replay (not always available)
pub fn extract_platform(_replay: &Replay) -> String {
    // Placeholder: Adjust this based on specific data in replay
    "Unknown".to_string()
}

/// Extract the match date from the replay
pub fn extract_date(replay: &Replay) -> String {
    if let Some(HeaderProp::QWord(time)) = get_property_value(&replay.properties, "Date") {
        // Use the new recommended method
        let naive = NaiveDateTime::from_timestamp_opt(*time as i64, 0).unwrap_or_else(|| {
            eprintln!("Invalid timestamp; defaulting to epoch.");
            NaiveDateTime::from_timestamp(0, 0)
        });
        let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive);
        return datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    }
    "Unknown".to_string()
}
