use core::fmt;
use std::error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct FractionError {
    details: String,
}

impl FractionError {
    pub fn new(msg: &str) -> FractionError {
        FractionError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for FractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for FractionError {
    fn description(&self) -> &str {
        &self.details
    }
}