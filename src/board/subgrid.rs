use super::cell::Cell;

#[derive(Clone, Copy)]
pub struct SubGrid([[Cell; 3]; 3]);

impl SubGrid {
    pub fn new() -> Self {
        Self([[Cell::Empty; 3]; 3])
    }

    pub fn get(&self, row: usize, col: usize) -> &Cell {
        &self.0[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.0[row][col] = cell;
    }

    pub fn get_row(&self, row: usize) -> Vec<&Cell> {
        self.0[row].iter().collect()
    }

    pub fn get_col(&self, col: usize) -> Vec<&Cell> {
        self.0.iter().map(|row| &row[col]).collect()
    }
}