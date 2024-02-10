use crate::error::ParserError;

use log::trace;

use crate::arguments::{ParsedArgument, RawArgument};

#[derive(Default)]
pub struct Parser {
    _defined_arguments: Vec<RawArgument>,
    _parsed_arguments: Vec<ParsedArgument>,
}

impl Parser {
    /// Creates a new Parser
    pub fn new() -> Self {
        Self {
            _defined_arguments: Vec::new(),
            _parsed_arguments: Vec::new(),
        }
    }

    /// Adds an Argument
    /// ```rust
    /// use cali::parser::Parser;
    ///
    /// let parser = Parser::new()
    ///     .add_arg("h", "help", "Prints this help prompt", false, None)
    ///     .add_arg("t", "test", "None", true, Some("test_string".to_owned()));
    /// ```
    pub fn add_arg(
        mut self,
        short: &str,
        long: &str,
        description: &str,
        has_value: bool,
        default: Option<String>,
    ) -> Self {
        self._defined_arguments.push(RawArgument::new(
            short,
            long,
            description,
            has_value,
            default,
        ));
        self
    }

    /// Parses the supplied cli Arguments and tries matching them with the predefined Arguments
    /// ```rust
    /// use cali::parser::Parser;
    ///
    /// let mut parser = Parser::new().add_arg("t", "test", "A test Argument", true, Some("test_value".to_owned()));
    /// parser.parse();
    /// ```
    pub fn parse(&mut self) -> Result<(), ParserError> {
        let system_arguments = std::env::args().collect::<Vec<String>>();
        let mut idx = 1;

        while idx < system_arguments.len() {
            let current_system_argument = system_arguments
                .get(idx)
                .unwrap_or(&String::new())
                .to_owned();

            let mut current_parsed_argument = self.parse_string(&current_system_argument)?;
            idx += 1;
            // if no other steps are necessary add to parsed and start with next argument
            if !current_parsed_argument.has_value() {
                trace!(
                    "Pushing without value {}",
                    current_system_argument
                );
                self._parsed_arguments.push(current_parsed_argument);
                continue;
            }

            // if there is another argument, it will be used as value
            if let Some(next_value) = system_arguments.get(idx) {
                trace!(
                    "Pushing with value {}: {}",
                    current_system_argument,
                    next_value
                );
                current_parsed_argument.value = Some(next_value.clone());
                self._parsed_arguments.push(current_parsed_argument);
                idx += 1;
            } else {
                return Err(ParserError::new(&format!(
                    "Did not provide a value for {current_system_argument}"
                )));
            }
        }
        Ok(())
    }

    fn parse_string(&self, string: &str) -> Result<ParsedArgument, ParserError> {
        match string {
            s if s.starts_with("--") => match self.find_predefined_by_long(s) {
                Some(found) => {
                    trace!("Found long arg {s}");
                    Ok(ParsedArgument::new(Some(found), None))
                }
                None => {
                    Err(ParserError::new(&format!(
                        "{} is not a valid Argument!",
                        string
                    )))
                }
            },
            s if s.starts_with('-') => match self.find_predefined_by_short(s) {
                Some(found) => {
                    trace!("Found short arg {s}");
                    Ok(ParsedArgument::new(Some(found), None))
                }
                None => {
                    Err(ParserError::new(&format!(
                        "{} is not a valid Argument!",
                        string
                    )))
                }
            },
            _ => {
                Err(ParserError::new(&format!(
                    "{} is not a valid Argument!",
                    string
                )))
            }
        }
    }

    fn find_predefined_by_short(&self, short: &str) -> Option<RawArgument> {
        self._defined_arguments
            .iter()
            .find(|raw_argument| raw_argument.short_matches(short.to_owned()))
            .cloned()
    }

    fn find_predefined_by_long(&self, long: &str) -> Option<RawArgument> {
        self._defined_arguments
            .iter()
            .find(|raw_argument| raw_argument.long_matches(long.to_owned()))
            .cloned()
    }

    /// Returns the defined Arguments for printing help prompts or the likes
    /// ```rust
    /// use cali::parser::Parser;
    ///
    /// let parser = Parser::new().add_arg("t", "test", "A test Argument", true, Some("test_value".to_owned()));
    /// let arguments = parser.get_arguments();
    /// 
    /// for argument in arguments {
    ///     println!("{argument}");
    /// }
    /// ```
    pub fn get_arguments(&self) -> Vec<RawArgument> {
        self._defined_arguments.clone()
    }

    /// Returns either false, if argument has not been found, or true with the optional value of the argument
    pub fn get_parsed_argument_long(&self, long: &str) -> (bool, Option<String>) {
        let parsed_argument = self._parsed_arguments.iter().find(|parsed_argument| {
            parsed_argument
                .defined_argument
                .clone()
                .unwrap_or_default()
                .long_matches(long.to_owned())
        });
        match parsed_argument {
            Some(parsed_argument) => (true, parsed_argument.value.clone()),
            None => (false, None),
        }
    }

    /// Returns either false, if argument has not been found, or true with the optional value of the argument
    pub fn get_parsed_argument_short(&self, short: &str) -> (bool, Option<String>) {
        let parsed_argument = self._parsed_arguments.iter().find(|parsed_argument| {
            parsed_argument
                .defined_argument
                .clone()
                .unwrap_or_default()
                .short_matches(short.to_owned())
        });
        match parsed_argument {
            Some(parsed_argument) => (true, parsed_argument.value.clone()),
            None => (false, None),
        }
    }
}
