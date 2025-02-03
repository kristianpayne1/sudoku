use crate::Sudoku;
use rand::{Rng};
use std::collections::{HashSet};
use crate::board::cell::Cell;

impl Sudoku {
    fn get_available_values(&self, row: usize, col: usize) -> Vec<u8> {
        let subgrid = self.grid.get_subgrid(row / 3, col / 3);
        let row_list = self.grid.get_row(row);
        let column_list = self.grid.get_col(col);
        let all_cells = row_list.into_iter().chain(column_list).chain(subgrid.get_all());
        let mut available_values = (1..=9).collect::<HashSet<_>>();
        for cell in all_cells {
            if let Cell::Value(value) = cell {
                if available_values.contains(value) { available_values.remove(value); }
            }
        }

       available_values.iter().cloned().collect()
    }
    
    pub fn generate(&mut self) {
        for i in 0..self.grid.get_col(0).len() {
            for j in 0..self.grid.get_row(0).len() {
                let available_values = self.get_available_values(i, j);
                let random_index = rand::rng().random_range(0..available_values.len());
                let value = available_values[random_index];
                self.set_value(i, j, value);
                self.display();
            }
        }
    }
}
