use super::value::DefinedValue;

pub struct DefinedArgument {
    pub short: String,
    pub long: String,
    pub description: String,
    pub value: DefinedValue,
    pub callback: fn(i32) -> std::io::Result<()>
}

impl DefinedArgument {
    pub fn new(short: &str, long: &str, description: &str, value: DefinedValue, callback: fn(i32) -> std::io::Result<()>) -> Self {
        Self {
            short: short.to_owned(),
            long: long.to_owned(),
            description: description.to_owned(),
            value,
            callback
        }
    }
}