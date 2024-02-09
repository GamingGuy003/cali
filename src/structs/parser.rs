use super::argument::Argument;

pub struct Parser {
    arguments: Vec<Argument>
}

impl Parser {
    pub fn new(arguments: Vec<Argument>) -> Self {
        Self { arguments }
    }

    pub fn parse(&mut self) {
        let unparsed = std::env::args();
    }
}