pub struct Argument {
    long: String,
    short: String,
    description: String,
    value: bool,
    default: Option<String>,
    handler: Box<dyn FnOnce()>
}