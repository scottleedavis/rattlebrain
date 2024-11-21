use boxcars::{Replay, HeaderProp, NetworkFrames};

// Struct definitions
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
pub struct ActorStats {
    pub total_updates: u32,
    pub unique_actors: u32,
}

impl ActorStats {
    pub fn new() -> Self {
        ActorStats {
            total_updates: 0,
            unique_actors: 0,
        }
    }

    pub fn process_frame(&mut self, frame: &boxcars::Frame) {
        self.total_updates += frame.updated_actors.len() as u32;
        // You can add more sophisticated actor tracking here
    }
}

pub fn analyze_replay(replay: &Replay) -> Result<GameAnalysis, Box<dyn Error>> {
    // Get properties from the replay
    let properties = match &replay.properties {
        Some(props) => props,
        None => return Err("No properties found in replay".into()),
    };

    // Extract match info
    let game_score = extract_game_score(properties)?;
    let primary_player = extract_primary_player(properties)?;
    let team_sizes = extract_team_sizes(properties)?;
    let match_type = extract_match_type(properties)?;
    let arena = extract_arena(properties)?;
    let platform = extract_platform(properties)?;
    let date = extract_date(properties)?;

    // Get frames from network data
    let network_frames = match &replay.network_frames {
        Some(frames) => frames,
        None => return Err("No network frames found in replay".into()),
    };

    // Process actor data
    let mut actor_stats = ActorStats::new();
    for frame in &network_frames.frames {
        actor_stats.process_frame(frame);
    }

    Ok(GameAnalysis {
        engine_version: format!("{}.{}", replay.major_version, replay.minor_version),
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

// Helper functions for property extraction
fn extract_game_score(properties: &[(String, HeaderProp)]) -> Result<GameScore, Box<dyn Error>> {
    let team_0_score = properties
        .iter()
        .find(|(name, _)| name == "Team0Score")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Int(score) => Some(*score),
            _ => None,
        })
        .ok_or("Team 0 score not found or invalid")?;

    let team_1_score = properties
        .iter()
        .find(|(name, _)| name == "Team1Score")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Int(score) => Some(*score),
            _ => None,
        })
        .ok_or("Team 1 score not found or invalid")?;

    Ok(GameScore {
        team_0_score,
        team_1_score,
    })
}

fn extract_primary_player(properties: &[(String, HeaderProp)]) -> Result<String, Box<dyn Error>> {
    properties
        .iter()
        .find(|(name, _)| name == "PlayerName")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Str(name) => Some(name.clone()),
            _ => None,
        })
        .ok_or_else(|| "Primary player name not found or invalid".into())
}

fn extract_team_sizes(properties: &[(String, HeaderProp)]) -> Result<TeamSizes, Box<dyn Error>> {
    let team_size = properties
        .iter()
        .find(|(name, _)| name == "TeamSize")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Int(size) => Some(*size),
            _ => None,
        })
        .ok_or("Team size not found or invalid")?;

    Ok(TeamSizes {
        blue: team_size,
        orange: team_size,
    })
}

fn extract_match_type(properties: &[(String, HeaderProp)]) -> Result<String, Box<dyn Error>> {
    properties
        .iter()
        .find(|(name, _)| name == "MatchType")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Str(match_type) => Some(match_type.clone()),
            _ => None,
        })
        .ok_or_else(|| "Match type not found or invalid".into())
}

fn extract_arena(properties: &[(String, HeaderProp)]) -> Result<String, Box<dyn Error>> {
    properties
        .iter()
        .find(|(name, _)| name == "MapName")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Str(arena) => Some(arena.clone()),
            _ => None,
        })
        .ok_or_else(|| "Arena name not found or invalid".into())
}

fn extract_platform(properties: &[(String, HeaderProp)]) -> Result<String, Box<dyn Error>> {
    properties
        .iter()
        .find(|(name, _)| name == "Platform")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Str(platform) => Some(platform.clone()),
            _ => None,
        })
        .ok_or_else(|| "Platform not found or invalid".into())
}

fn extract_date(properties: &[(String, HeaderProp)]) -> Result<String, Box<dyn Error>> {
    properties
        .iter()
        .find(|(name, _)| name == "Date")
        .and_then(|(_, prop)| match prop {
            HeaderProp::Str(date) => Some(date.clone()),
            _ => None,
        })
        .ok_or_else(|| "Date not found or invalid".into())
}