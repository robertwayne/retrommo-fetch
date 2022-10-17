use serde::{Deserialize, Serialize};

/// Represents a single player entry from a `LeaderboardPage`.
#[derive(Debug, Deserialize, Serialize)]
pub struct LeaderboardEntry {
    pub experience: u64,
    pub permissions: u8,
    pub username: String,
}

impl std::fmt::Display for LeaderboardEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} experience", self.username, self.experience)
    }
}
