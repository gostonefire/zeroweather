use std::fmt;
use std::fmt::Formatter;


pub struct W1Error(pub String);

impl fmt::Display for W1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "W1Error: {}", self.0)
    }
}
impl From<std::io::Error> for W1Error {
    fn from(e: std::io::Error) -> Self { W1Error(e.to_string()) }
}
impl From<String> for W1Error {
    fn from(e: String) -> Self { W1Error(e) }
}
impl From<std::num::ParseFloatError> for W1Error {
    fn from(e: std::num::ParseFloatError) -> Self { W1Error(e.to_string()) }
}
