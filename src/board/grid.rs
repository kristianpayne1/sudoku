use crate::board::subgrid::SubGrid;
use super::cell::Cell;

pub struct Grid([[SubGrid; 3]; 3]);

impl Grid {
    pub fn new() -> Self {
        Self([[SubGrid::new(); 3]; 3])
    }

    pub fn get(&self, row: usize, col: usize) -> &Cell {
        let (sub_row, cell_row) = (row / 3, row % 3);
        let (sub_col, cell_col) = (col / 3, col % 3);
        self.0[sub_row][sub_col].get(cell_row, cell_col)
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        let (sub_row, cell_row) = (row / 3, row % 3);
        let (sub_col, cell_col) = (col / 3, col % 3);
        self.0[sub_row][sub_col].set(cell_row, cell_col, cell);
    }

    pub fn get_row(&self, row: usize) -> Vec<&Cell> {
        let (sub_row, cell_row) = (row / 3, row % 3);
        self.0[sub_row]
            .iter()
            .flat_map(|sub_grid| sub_grid.get_row(cell_row))
            .collect()
    }

    pub fn get_col(&self, col: usize) -> Vec<&Cell> {
        let (sub_col, cell_col) = (col / 3, col % 3);
        self.0
            .iter()
            .flat_map(|sub_row| sub_row[sub_col].get_col(cell_col))
            .collect()
    }
}
