use std::fmt;

#[derive(Debug)]
pub struct AlreadyFilledError;
#[derive(Debug)]
pub struct InvalidValueError;
pub struct NoAvailableValidValuesError;
pub struct NoSolutionFoundError;

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

impl fmt::Display for NoAvailableValidValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No available valid values")
    }
}

impl fmt::Display for NoSolutionFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No solution found")
    }
}
