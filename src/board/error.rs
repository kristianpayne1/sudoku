use std::fmt;

pub struct AlreadyFilledError;
pub struct InvalidValueError;

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Value is invalid") }
}
