use std::error::Error;
use boxcars::{Replay, HeaderProp};

// Type alias to simplify error handling
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct GameScore {
    pub team_0_score: i32,
    pub team_1_score: i32,
}

#[derive(Debug)]
pub struct TeamSizes {
    pub blue: i32,
    pub orange: i32,
}

#[derive(Debug)]
pub struct GameAnalysis {
    pub engine_version: String,
    pub game_score: GameScore,
    pub primary_player: String,
    pub team_sizes: TeamSizes,
    pub match_type: String,
    pub arena: String,
    pub platform: String,
    pub date: String,
    pub actor_stats: ActorStats,
}

#[derive(Debug)]
pub struct ActorStats {
    pub total_updates: i32,
}

pub fn analyze_replay(replay: &Replay) -> Result<GameAnalysis> {
    let properties = &replay.properties;

    let game_score = extract_game_score(properties)?;
    let primary_player = extract_primary_player(properties)?;
    let team_sizes = extract_team_sizes(properties)?;
    let match_type = extract_match_type(properties)?;
    let arena = extract_arena(properties)?;
    let platform = extract_platform(properties)?;
    let date = extract_date(properties)?;
    
    // Extract engine version using major_version and minor_version
    let engine_version = format!("{}.{}", replay.major_version, replay.minor_version);
    
    // Create actor stats using the frames length
    let actor_stats = ActorStats {
        total_updates: replay.network_frames.as_ref()
            .map(|frames| frames.frames.len() as i32)
            .unwrap_or(0),
    };

    Ok(GameAnalysis {
        engine_version,
        game_score,
        primary_player,
        team_sizes,
        match_type,
        arena,
        platform,
        date,
        actor_stats,
    })
}

fn extract_game_score(properties: &[(String, HeaderProp)]) -> Result<GameScore> {
    let mut team_0_score = 0;
    let mut team_1_score = 0;

    for (key, prop) in properties {
        match (key.as_str(), prop) {
            ("Team0Score", HeaderProp::Int(score)) => team_0_score = *score,
            ("Team1Score", HeaderProp::Int(score)) => team_1_score = *score,
            _ => continue,
        }
    }

    Ok(GameScore {
        team_0_score,
        team_1_score,
    })
}

fn extract_primary_player(properties: &[(String, HeaderProp)]) -> Result<String> {
    for (key, prop) in properties {
        if key == "PlayerName" {
            if let HeaderProp::Str(name) = prop {
                return Ok(name.clone());
            }
        }
    }
    Ok("Unknown".to_string())
}

fn extract_team_sizes(properties: &[(String, HeaderProp)]) -> Result<TeamSizes> {
    let mut blue_size = 0;
    let mut orange_size = 0;

    for (key, prop) in properties {
        match (key.as_str(), prop) {
            ("TeamSize", HeaderProp::Int(size)) => {
                blue_size = *size;
                orange_size = *size;
            }
            _ => continue,
        }
    }

    Ok(TeamSizes {
        blue: blue_size,
        orange: orange_size,
    })
}

fn extract_match_type(properties: &[(String, HeaderProp)]) -> Result<String> {
    for (key, prop) in properties {
        if key == "MatchType" {
            if let HeaderProp::Str(match_type) = prop {
                return Ok(match_type.clone());
            }
        }
    }
    Ok("Unknown".to_string())
}

fn extract_arena(properties: &[(String, HeaderProp)]) -> Result<String> {
    for (key, prop) in properties {
        if key == "MapName" {
            if let HeaderProp::Str(arena) = prop {
                return Ok(arena.clone());
            }
        }
    }
    Ok("Unknown".to_string())
}

fn extract_platform(properties: &[(String, HeaderProp)]) -> Result<String> {
    for (key, prop) in properties {
        if key == "Platform" {
            if let HeaderProp::Str(platform) = prop {
                return Ok(platform.clone());
            }
        }
    }
    Ok("Unknown".to_string())
}

fn extract_date(properties: &[(String, HeaderProp)]) -> Result<String> {
    for (key, prop) in properties {
        if key == "Date" {
            if let HeaderProp::Str(date) = prop {
                return Ok(date.clone());
            }
        }
    }
    Ok("Unknown".to_string())
}