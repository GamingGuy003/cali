pub mod arguments;
pub mod error;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn run() {
        let parser = Parser::new();
        assert!(parser.parse().is_ok());
    }
}
