use reqwest::StatusCode;

#[derive(Debug)]
pub struct Error {
    pub code: StatusCode,
    pub message: Option<String>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}: {message}", self.code),
            None => write!(f, "{}", self.code),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.message {
            Some(ref message) => message,
            None => "",
        }
    }
}

impl Error {
    #[must_use]
    pub fn new(code: StatusCode, message: Option<String>) -> Self {
        Self { code, message }
    }
}
