# retrommo-fetch

Small Rust wrapper around the [RetroMMO](https://retro-mmo.com) API.

## Usage

The library provides a prelude which exports all relevant types and functions.

```rust
use retrommo_fetch::prelude::*;
```

### Available Functions

### `get_leaderboard() -> Leaderboard`

Returns an iterator over the leaderboard pages.

*Note: This is a non-async function. If you are calling it within an async context, you must use a blocking thread, like `tokio::task::block_in_place`.*

```rust
let mut leaderboard = get_leaderboard();

let page1 = leaderboard.try_next()?;
println!("Page 1: {:?}", page1);

let page2 = leaderboard.try_next()?;
println!("Page 2: {:?}", page2);
```

### `get_player(username: String)` -> `Result<Player, Error>`

Returns a `Player` struct with the given username.

```rust
let player = get_player("Gliss").await?;
println!("{:#?}", player);

// Player {
//     rank: 25,
//     registered_at: 2020-11-10T05:05:02Z,
//     username: "Gliss",
//     time_played: 1009554.2688390692,
//     permissions: 0,
//     lifetime_experience: 313320,
// }
```

### `get_leaderboard_page(page: u32)` -> `Result<Leaderboard, Error>`

Returns a `Leaderboard` struct with all the player entries on the given page. If no page is given *(None)*, the first page is returned.

This is 100 entries per page maximum.

```rust
let page = get_leaderboard_page(Some(4)).await?;
for entry in page {
    println!("{:?}", entry);
}
```

### `get_top_players() -> Result<LeaderboardPage, Error>`

Returns a `LeaderboardPage` struct with the top 100 players. This is just an alias for `get_leaderboard_page(None)` or `get_leaderboard_page(Some(1))`.

### `get_online_players()` -> `Result<OnlineList, Error>`

Returns a `Vec` of players that are currently online. Note that this is only their usernames *(String)*.

### `get_online_players_full()` -> `Result<Vec<Player>, Error>`

Returns a `Vec` of `Player` structs that are currently online. Be careful of using this if many players are online, as it may require many requests to the API due to limitations.

### `get_registered_player_count()` -> `Result<u64, Error>`

Returns the total number of registered players.

## License

retrommo-fetch source code is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
