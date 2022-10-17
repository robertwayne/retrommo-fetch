use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single player. Note that this struct is NOT the same as the one
/// returned by leaderboard resulsts. This is explicitly when requesting a
/// specific player, and contains much more information.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub rank: u64,
    pub registered_at: DateTime<Utc>,
    pub username: String,
    pub time_played: f64,
    pub permissions: u8,
    pub lifetime_experience: u64,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Username: {}\nExperience{} ({})\nRegistered: {}, Time Played: {}, Permission Rank: {}",
            self.username,
            self.lifetime_experience,
            self.rank,
            self.registered_at,
            self.time_played,
            self.permissions
        )
    }
}
