use std::fmt::Display;

#[derive(Clone, Default, Debug)]
pub struct ParsedArgument {
    pub defined_argument: Option<RawArgument>,
    pub value: Option<String>,
}

impl ParsedArgument {
    /// Creates a new Argument from a Raw Argument and an optional value
    pub fn new(defined_argument: Option<RawArgument>, value: Option<String>) -> Self {
        Self {
            defined_argument,
            value,
        }
    }

    /// Returns true if the Argument is supposed to get a value
    pub fn has_value(&self) -> bool {
        match &self.defined_argument {
            Some(arg) => arg.has_value,
            None => false,
        }
    }

    /// Clears the Arguments fields and sets them to none
    pub fn clear(&mut self) {
        self.defined_argument = None;
        self.value = None;
    }

    /// Returns true if the argument matches, or false if not matching or not present. Leading minus' are ignored
    pub fn long_matches(&self, long: String) -> bool {
        self.defined_argument
            .clone()
            .and_then(|argument| Some(argument.long_matches(long)))
            .unwrap_or(false)
    }

    /// Returns true if the argument matches, or false if not matching or not present. Leading minus' are ignored
    pub fn short_matches(&self, short: String) -> bool {
        self.defined_argument
            .clone()
            .and_then(|argument| Some(argument.short_matches(short)))
            .unwrap_or(false)
    }
}

#[derive(Clone, Default, Debug)]
pub struct RawArgument {
    pub short: String,
    pub long: String,
    pub description: String,
    pub has_value: bool,
    _default: Option<String>,
}

impl RawArgument {
    pub fn new(
        short: &str,
        long: &str,
        description: &str,
        has_value: bool,
        _default: Option<String>,
    ) -> RawArgument {
        Self {
            short: short.to_owned(),
            long: long.to_owned(),
            description: description.to_owned(),
            has_value,
            _default,
        }
    }

    /// Returns true if the argument matches, or false if not. Leading minus' are ignored
    pub fn long_matches(&self, long: String) -> bool {
        long.trim_start_matches("--") == self.long
    }

    /// Returns true if the argument matches, or false if not. Leading minus' are ignored
    pub fn short_matches(&self, short: String) -> bool {
        short.trim_start_matches('-') == self.short
    }

    /// Returns the default value for the argument
    pub fn get_default(&self) -> Option<String> {
        self._default.clone()
    }
}

impl Display for RawArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_value {
            write!(
                f,
                "-{} \t --{} <value> \t {}",
                self.short, self.long, self.description
            )
        } else {
            write!(
                f,
                "-{} \t --{} \t \t {}",
                self.short, self.long, self.description
            )
        }
    }
}
