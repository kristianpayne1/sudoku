use crate::board::cell::Cell;
use crate::board::error::SetValueError;
use crate::board::grid::Grid;

pub struct Sudoku {
    pub grid: Grid,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn value(&self) -> u32 {
        match self {
            Difficulty::Easy => 30,
            Difficulty::Medium => 40,
            Difficulty::Hard => 50,
        }
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self { grid: Grid::new() }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: u8) -> Result<(), SetValueError> {
        if value < 1 || value > 9 {
            return Err(SetValueError::InvalidValue);
        }
        match self.grid.get(row, col) {
            Cell::Empty => Ok(self.grid.set(row, col, Cell::Value(value))),
            Cell::Value(_val) => Err(SetValueError::AlreadyFilledError),
        }
    }

    pub fn display(&self) {
        self.grid.display();
    }
}
