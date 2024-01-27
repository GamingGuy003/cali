// an error to handle returning multiple different error variants from parsing
#[derive(Debug, PartialEq)]
pub enum ParseError {
    ParseNumberError(String),
    ParseBooleanError(String),
    ParseStringError(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParseNumberError(string) => write!(f, "Failed to parse integer: {string}"),
            ParseError::ParseBooleanError(string) => write!(f, "Failed to parse boolean: {string}"),
            ParseError::ParseStringError(string) => write!(f, "Failed to parse string: {string}"),
        }
    }
}

impl std::error::Error for ParseError {}
