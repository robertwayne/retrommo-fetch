#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub mod entry;
pub mod error;
pub mod leaderboard;
pub mod player;
pub mod prelude;

use reqwest::StatusCode;

use crate::{entry::LeaderboardEntry, error::Error, leaderboard::Leaderboard, player::Player};

pub(crate) const API_URL: &str = "https://play.retro-mmo.com";

pub type OnlineList = Vec<String>;
pub type LeaderboardPage = Vec<LeaderboardEntry>;

/// Returns a specific player by their username.
pub async fn get_player(username: &str) -> Result<Player, Error> {
    let response = reqwest::get(format!("{API_URL}/users/{username}.json")).await;

    if let Ok(player) = response {
        let player = player.json::<Player>().await;

        match player {
            Ok(player) => Ok(player),
            Err(e) => Err(Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("Failed to parse player: {e}")),
            )),
        }
    } else {
        Err(Error::new(StatusCode::INTERNAL_SERVER_ERROR, Some("Failed to get player".to_string())))
    }
}

/// Returns a list of online players on the server. Only includes their
/// username.
///
/// If you wish to get all the online players as `Player` structs, use
/// `get_online_players_full`.
pub async fn get_online_players() -> Result<OnlineList, Error> {
    let response = reqwest::get(format!("{API_URL}/players.json")).await;

    if let Ok(players) = response {
        let players = players.json::<OnlineList>().await;

        match players {
            Ok(players) => Ok(players),
            Err(e) => Err(Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("Failed to parse online players: {e}")),
            )),
        }
    } else {
        Err(Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Failed to get online players".to_string()),
        ))
    }
}

/// Returns a list of Player structs for all the online players on the server.
/// Note that this requires a separate API request for each player, due to the
/// official API options available.
///
/// Use with caution so you don't slam the servers with unnessecary requests (or
/// risk your IP being banned).
pub async fn get_online_players_full() -> Result<Vec<Player>, Error> {
    let mut players = Vec::new();

    for player in get_online_players().await? {
        players.push(get_player(&player).await?);
    }

    Ok(players)
}

/// Returns the total amount of registered accounts.
pub async fn get_registered_player_count() -> Result<u64, Error> {
    let response = reqwest::get(format!("{API_URL}/registered-users.json")).await;

    if let Ok(count) = response {
        let count = count.json::<u64>().await;

        match count {
            Ok(count) => Ok(count),
            Err(e) => Err(Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("Failed to parse registered user count: {e}")),
            )),
        }
    } else {
        Err(Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Failed to get registered user count".to_string()),
        ))
    }
}

/// Returns a specific page of the leaderboard.
///
/// Currently there is no way to get more entries than what is given per page,
/// so you will have to iterate over a range of pages to get full / partial
/// leaderboard results. Be careful of slamming the servers with too many
/// requests.
pub async fn get_leaderboard_page(page: Option<u32>) -> Result<LeaderboardPage, Error> {
    let response = if let Some(page) = page {
        reqwest::get(format!("{API_URL}/leaderboards.json?page={page}")).await
    } else {
        reqwest::get(format!("{API_URL}/leaderboards.json")).await
    };

    if let Ok(leaderboard) = response {
        let leaderboard = leaderboard.json::<LeaderboardPage>().await;

        match leaderboard {
            Ok(leaderboard) => Ok(leaderboard),
            Err(e) => Err(Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(format!("Failed to parse leaderboard: {e}")),
            )),
        }
    } else {
        Err(Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some("Failed to get leaderboard".to_string()),
        ))
    }
}

/// Helper method for getting the "front page" of the leaderboard -- or the top
/// 100 -- to be exact.
///
/// This is no different than calling `get_leaderboard_page(Some(1))` yourself.
pub async fn get_top_players() -> Result<LeaderboardPage, Error> {
    let results = get_leaderboard_page(Some(1)).await?;

    Ok(results)
}

/// Returns an iterator over the leaderboard pages, starting at 0. You can
/// advance the iterator by calling `try_next()` or `next()` on the iterator
/// returned from this function.
///
/// See `Leaderboard` for more information.
pub fn get_leaderboard() -> Leaderboard {
    Leaderboard::new()
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;

    #[tokio::test]
    async fn test_get_player() {
        let player = get_player("Gliss").await;
        assert!(player.is_ok());
    }

    #[tokio::test]
    async fn test_get_player_not_found() {
        let player = get_player("a4iujtoisdjugfoiasjuhroighasoidg").await;
        assert!(player.is_err());
    }

    #[tokio::test]
    async fn test_get_player_eq() {
        let player = get_player("Gliss").await;

        if let Ok(player) = player {
            assert_eq!(player.username, "Gliss");
            assert_eq!(
                player.registered_at,
                DateTime::parse_from_rfc3339("2020-11-10T05:05:02.000Z").unwrap()
            );
        }
    }

    #[tokio::test]
    async fn test_get_players_online() {
        let players = get_online_players().await;
        assert!(players.is_ok());
    }

    #[tokio::test]
    async fn test_registered_users() {
        let count = get_registered_player_count().await;
        assert!(count.is_ok());
        assert!(count.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_leaderboard() {
        let leaderboard = get_leaderboard_page(None).await;
        assert!(leaderboard.is_ok());
        assert!(!leaderboard.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_leaderboard_page() {
        let leaderboard = get_leaderboard_page(Some(1)).await;
        assert!(leaderboard.is_ok());
        assert!(!leaderboard.unwrap().is_empty());
    }
}
