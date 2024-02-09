use log::{trace, debug};

use crate::arguments::arguments::{ParsedArgument, RawArgument};

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
    /// let mut parser = Parser::new();
    /// parser.add_arg("t", "test", "A test Argument", true, Some("test_value".to_owned()));
    /// ```
    pub fn add_arg(
        &mut self,
        short: &str,
        long: &str,
        description: &str,
        has_value: bool,
        default: Option<String>,
    ) {
        self._defined_arguments.push(RawArgument::new(
            short,
            long,
            description,
            has_value,
            default,
        ))
    }

    /// Parses the supplied cli Arguments and tries matching them with the predefined Arguments
    /// ```rust
    /// use cali::parser::Parser;
    /// 
    /// let mut parser = Parser::new();
    /// parser.add_arg("t", "test", "A test Argument", true, Some("test_value".to_owned()));
    /// parser.parse();
    /// ```
    pub fn parse(&mut self) {
        let mut current_arg = ParsedArgument::new(None, None);
        let mut searching_value = false;

        for raw_argument in std::env::args() {
            // If searching for value, take current string and add as value, push to parsed args and clear current arg
            if searching_value {
                current_arg.value = Some(raw_argument);
                self._parsed_arguments.push(current_arg.clone());
                current_arg.clear();
                searching_value = false;
                continue;
            }

            match &raw_argument {
                s if s.starts_with("--") => {
                    match self.find_by_long(&raw_argument) {
                        Some(found) => {
                            trace!("Found long arg {raw_argument}");
                            current_arg.set_defined(found)
                        },
                        None => {
                            debug!("Undefined long argument {raw_argument}");
                            continue;
                        }
                    }
                },
                s if s.starts_with("-") => {
                    match self.find_by_short(&raw_argument) {
                        Some(found) => {
                            trace!("Found short arg {raw_argument}");
                            current_arg.set_defined(found)
                        },
                        None => {
                            debug!("Undefined short argument {raw_argument}");
                            continue;
                        }
                    }
                },
                _ => {}
            }

            // if current arg needs a value but still doesnt have one presume next string is value
            if current_arg.has_value() && current_arg.value.is_none() {
                searching_value = true;
            }
        }
    }

    fn find_by_short(&self, short: &str) -> Option<RawArgument> {
        self._defined_arguments
            .iter()
            .find(|raw_argument| raw_argument.short_matches(short.to_owned()))
            .cloned()
    }

    fn find_by_long(&self, long: &str) -> Option<RawArgument> {
        self._defined_arguments
            .iter()
            .find(|raw_argument| raw_argument.long_matches(long.to_owned()))
            .cloned()
    }

    /// Prints the help prompt
    /// ```rust
    /// use cali::parser::Parser;
    /// 
    /// let mut parser = Parser::new();
    /// parser.add_arg("t", "test", "A test Argument", true, Some("test_value".to_owned()));
    /// parser.help();
    /// ```
    pub fn help(&self) {
        for defined_argument in &self._defined_arguments {
            println!("{defined_argument}");
        }
    }
}
