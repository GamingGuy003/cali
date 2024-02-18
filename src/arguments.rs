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
    pub fn long_matches(&self, long: &str) -> bool {
        self.defined_argument
            .clone().map(|argument| argument.long_matches(long))
            .unwrap_or(false)
    }

    /// Returns true if the argument matches, or false if not matching or not present. Leading minus' are ignored
    pub fn short_matches(&self, short: &str) -> bool {
        self.defined_argument
            .clone().map(|argument| argument.short_matches(short))
            .unwrap_or(false)
    }

    /// Returns true if the arguments value is optional, otherwise false
    pub fn is_optional(&self) -> bool {
        self.defined_argument
            .clone()
            .unwrap_or_default()
            .is_optional
    }
}

#[derive(Clone, Default, Debug)]
pub struct RawArgument {
    pub short: String,
    pub long: String,
    pub description: String,
    pub has_value: bool,
    pub is_optional: bool
}

impl RawArgument {
    pub fn new(
        short: &str,
        long: &str,
        description: &str,
        has_value: bool,
        is_optional: bool
    ) -> RawArgument {
        Self {
            short: short.to_owned(),
            long: long.to_owned(),
            description: description.to_owned(),
            has_value,
            is_optional
        }
    }

    /// Returns true if the argument matches, or false if not. Leading minus' are ignored
    pub fn long_matches(&self, long: &str) -> bool {
        long.trim_start_matches("--") == self.long
    }

    /// Returns true if the argument matches, or false if not. Leading minus' are ignored
    pub fn short_matches(&self, short: &str) -> bool {
        short.trim_start_matches('-') == self.short
    }
}

impl Display for RawArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let short = format!("-{:<5}", self.short);
        let long = format!("--{:<15}", self.long);
        let spacer = self
            .has_value
            .then(|| format!("{:<15}", "<value>"))
            .unwrap_or(format!("{:<15}", ""));
        write!(f, "{} {} {} {}", short, long, spacer, self.description)
    }
}
