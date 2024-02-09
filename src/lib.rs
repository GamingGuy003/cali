pub mod parser;
pub mod arguments;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn run() {
        let mut parser = Parser::new();
        parser.parse();
    }
}
