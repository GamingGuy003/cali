use super::error::ParseError::{self, ParseBooleanError, ParseNumberError, ParseStringError};

// This is the parsed value for the argument
#[derive(Debug, PartialEq)]
pub enum ParsedValue {
    None,
    Number(i32),
    Boolean(bool),
    String(String),
}

impl ParsedValue {
    pub fn try_parse(defined: DefinedValue, string: String) -> Result<Self, ParseError> {
        match defined {
            // If defined as none ignores input and always returns non
            DefinedValue::None => Ok(Self::None),
            // if defined as number, tries parsing to integer, otherwise errors
            DefinedValue::Number => string
                .parse::<i32>()
                .map(|number| Self::Number(number))
                .map_err(|err| ParseNumberError(err.to_string())),
            // if defined as boolean, tries parsing to boolean, otherwise errors
            DefinedValue::Boolean => string
                .parse::<bool>()
                .map(|boolean| Self::Boolean(boolean))
                .map_err(|err| ParseBooleanError(err.to_string())),
            // if defined as string, checks if string is provided, otherwise errors
            DefinedValue::String => {
                if string.is_empty() {
                    Err(ParseStringError("Empty String".to_owned()))
                } else {
                    Ok(Self::String(string))
                }
            }
            // if defined as optional number, tries parsing to number, otherwise returns none
            DefinedValue::OptionalNumber => Ok(string
                .parse::<i32>()
                .map(|number| Self::Number(number))
                .unwrap_or(Self::None)),
            // if defined as optional boolean, tries parsing to boolean, otherwise returns none
            DefinedValue::OptionalBoolean => Ok(string
                .parse::<bool>()
                .map(|boolean| Self::Boolean(boolean))
                .unwrap_or(Self::None)),
            // if defined as optional string, checks if string is provided, otherwise returns none
            DefinedValue::OptionalString => Ok(if string.is_empty() {
                Self::None
            } else {
                Self::String(string)
            }),
        }
    }
}

// This is the defined type for each cli arguments value
#[derive(Debug, PartialEq)]
pub enum DefinedValue {
    None,
    Number,
    Boolean,
    String,
    OptionalNumber,
    OptionalBoolean,
    OptionalString,
}
