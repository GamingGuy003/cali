use std::fmt::Display;

#[derive(Debug)]
pub struct ParserError {
    details: String,
}

impl ParserError {
    pub fn new(details: &str) -> Self {
        Self {
            details: details.to_owned(),
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse: {}", self.details)
    }
}

impl std::error::Error for ParserError {}
