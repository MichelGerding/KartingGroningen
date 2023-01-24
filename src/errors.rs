use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AlreadyExistsError {
    details: String,
}

impl AlreadyExistsError {
    pub fn new(msg: &str) -> AlreadyExistsError {
        AlreadyExistsError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AlreadyExistsError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct HeatInvalidError {
    details: String,
}

impl HeatInvalidError {
    pub fn new(msg: String) -> HeatInvalidError {
        HeatInvalidError { details: msg }
    }
}

impl fmt::Display for HeatInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl fmt::Debug for HeatInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
