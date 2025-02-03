use std::fmt;

#[derive(Debug)]
pub struct AlreadyFilledError;
#[derive(Debug)]
pub struct InvalidValueError;

#[derive(Debug)]
pub enum SetValueError {
    InvalidValue,
    AlreadyFilledError,
}

impl fmt::Display for AlreadyFilledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Already filled")
    }
}

impl fmt::Display for InvalidValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value is invalid")
    }
}
