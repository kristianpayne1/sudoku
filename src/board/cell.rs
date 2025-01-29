use std::fmt;

#[derive(Copy, Clone)]
pub enum Cell {
    Empty,
    Value(u8)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Value(val) => write!(f, "{}", val)
        }
    }
}

