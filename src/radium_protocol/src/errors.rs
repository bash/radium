use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TryFromError {
    InvalidValue,
}

impl fmt::Display for TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for TryFromError {
    fn description(&self) -> &str {
        "Invalid value"
    }
}