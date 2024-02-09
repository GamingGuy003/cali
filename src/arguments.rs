pub mod arguments {
    use std::fmt::Display;

    #[derive(Clone, Default, Debug)]
    pub struct ParsedArgument {
        pub defined_argument: Option<RawArgument>,
        pub value: Option<String>,
    }

    impl ParsedArgument {
        pub fn new(defined_argument: Option<RawArgument>, value: Option<String>) -> Self {
            Self {
                defined_argument,
                value,
            }
        }

        pub fn has_value(&self) -> bool {
            match &self.defined_argument {
                Some(arg) => arg.has_value,
                None => false,
            }
        }

        pub fn clear(&mut self) {
            self.defined_argument = None;
            self.value = None;
        }
    }

    #[derive(Clone, Default, Debug)]
    pub struct RawArgument {
        short: String,
        long: String,
        description: String,
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

        pub fn long_matches(&self, long: String) -> bool {
            long.trim_start_matches("--") == self.long
        }

        pub fn short_matches(&self, short: String) -> bool {
            short.trim_start_matches("-") == self.short
        }
    }

    impl Display for RawArgument {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.has_value {
                write!(
                    f,
                    "-{} \t --{} <value>\n{}",
                    self.short, self.long, self.description
                )
            } else {
                write!(
                    f,
                    "-{} \t --{}\n{}",
                    self.short, self.long, self.description
                )
            }
        }
    }
}
