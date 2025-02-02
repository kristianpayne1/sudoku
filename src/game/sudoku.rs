use crate::board::cell::Cell;
use crate::board::error::{SetValueError};
use crate::board::grid::Grid;

pub struct Sudoku {
    pub grid: Grid,
}

impl Sudoku {
    pub fn new() -> Self {
        Self { grid: Grid::new() }
    }

    pub fn set_value(&mut self, value: u8, row: usize, col: usize) -> Result<(), SetValueError> {
        if (value < 1 || value > 9) {
            return Err(SetValueError::InvalidValue);
        }
        match self.grid.get(row, col) {
            Cell::Empty => Ok(self.grid.set(row, col, Cell::Value(value))),
            Cell::Value(_val) => Err(SetValueError::AlreadyFilledError)
        }
    }
}