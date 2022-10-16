use crate::{LeaderboardEntry, LeaderboardPage, API_URL};

/// Leaderboard is an iterator over the leaderboard pages. Each time `try_next`
/// is called, it will make a request to the API and fetch the next 100 entries.
///
/// # Examples
///
/// ```rust
/// use retrommo_fetch::get_leaderboard;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut leaderboard = get_leaderboard();
///
///     for i in 0..2 {
///         let page = leaderboard.try_next()?;
///
///       if let Some(page) = page {
///             println!("Page: {}", i + 1);
///             for entry in page {
///                 println!("{:?}", entry);
///             }
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub struct Leaderboard {
    client: reqwest::blocking::Client,
    entries: <Vec<LeaderboardEntry> as IntoIterator>::IntoIter,
    page: u32,
}

impl Iterator for Leaderboard {
    type Item = reqwest::Result<LeaderboardPage>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(page)) => Some(Ok(page)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl Leaderboard {
    pub fn new() -> Self {
        Self { client: reqwest::blocking::Client::new(), entries: Vec::new().into_iter(), page: 0 }
    }

    pub fn try_next(&mut self) -> reqwest::Result<Option<LeaderboardPage>> {
        self.page += 1;
        let response = self
            .client
            .get(&format!("{API_URL}/leaderboards.json?page={}", self.page))
            .send()?
            .json::<LeaderboardPage>()?;

        if response.is_empty() {
            return Ok(None);
        }

        self.entries = response.into_iter();

        Ok(Some(self.entries.by_ref().collect()))
    }

    pub fn current_page(&self) -> u32 {
        self.page
    }
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::new()
    }
}
