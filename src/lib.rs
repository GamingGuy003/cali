pub mod parsing;
pub mod structs;

#[cfg(test)]
mod tests {
    use crate::structs::error::ParseError::ParseStringError;
    use crate::structs::value::{DefinedValue, ParsedValue};

    #[test]
    fn parse_none() {
        assert_eq!(
            Ok(ParsedValue::None),
            ParsedValue::try_parse(DefinedValue::None, "".to_owned())
        )
    }

    #[test]
    fn parse_number() {
        assert_eq!(
            Ok(ParsedValue::Number(1)),
            ParsedValue::try_parse(DefinedValue::Number, "1".to_owned())
        )
    }

    #[test]
    fn parse_boolean_true() {
        assert_eq!(
            Ok(ParsedValue::Boolean(true)),
            ParsedValue::try_parse(DefinedValue::Boolean, "true".to_owned())
        )
    }

    #[test]
    fn parse_boolean_false() {
        assert_eq!(
            Ok(ParsedValue::Boolean(false)),
            ParsedValue::try_parse(DefinedValue::Boolean, "false".to_owned())
        )
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            Ok(ParsedValue::String("test".to_owned())),
            ParsedValue::try_parse(DefinedValue::String, "test".to_owned())
        )
    }

    #[test]
    fn parse_string_empty() {
        assert_eq!(
            Err(ParseStringError("Empty String".to_owned())),
            ParsedValue::try_parse(DefinedValue::String, "".to_owned())
        )
    }

    #[test]
    fn parse_optional_number() {
        assert_eq!(
            Ok(ParsedValue::String("test".to_owned())),
            ParsedValue::try_parse(DefinedValue::String, "test".to_owned())
        )
    }

    #[test]
    fn parse_optional_number_empty() {
        assert_eq!(
            Ok(ParsedValue::None),
            ParsedValue::try_parse(DefinedValue::OptionalNumber, "".to_owned())
        )
    }

    #[test]
    fn parse_optional_boolean() {
        assert_eq!(
            Ok(ParsedValue::Boolean(true)),
            ParsedValue::try_parse(DefinedValue::OptionalBoolean, "true".to_owned())
        )
    }

    #[test]
    fn parse_optional_boolean_empty() {
        assert_eq!(
            Ok(ParsedValue::None),
            ParsedValue::try_parse(DefinedValue::OptionalBoolean, "".to_owned())
        )
    }

    #[test]
    fn parse_optional_string() {
        assert_eq!(
            Ok(ParsedValue::String("test".to_owned())),
            ParsedValue::try_parse(DefinedValue::OptionalString, "test".to_owned())
        )
    }

    #[test]
    fn parse_optional_string_empty() {
        assert_eq!(
            Ok(ParsedValue::None),
            ParsedValue::try_parse(DefinedValue::OptionalString, "".to_owned())
        )
    }
}
