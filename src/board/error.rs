use std::fmt;

pub struct AlreadyFilledError;

impl fmt::Display for AlreadyFilledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Already filled")
    }
}

