use rand::Rng;
use std::collections::HashSet;

use crate::board::cell::Cell;
use crate::board::error::NoAvailableValidValuesError;
use crate::board::grid::Grid;
use crate::Sudoku;

fn get_available_values(grid: &Grid, row: usize, col: usize) -> HashSet<u8> {
    let subgrid = grid.get_subgrid(row / 3, col / 3);
    let row_list = grid.get_row(row);
    let column_list = grid.get_col(col);
    let all_cells = row_list
        .into_iter()
        .chain(column_list)
        .chain(subgrid.get_all());
    let mut available_values = (1..=9).collect::<HashSet<_>>();
    for cell in all_cells {
        if let Cell::Value(value) = cell {
            if available_values.contains(value) {
                available_values.remove(value);
            }
        }
    }

    available_values
}

impl Sudoku {
    pub fn fill_grid(
        &self,
        mut grid: &mut Grid,
        index: usize,
    ) -> Result<(), NoAvailableValidValuesError> {
        let grid_size = grid.size();
        if index == grid_size * grid_size {
            return Ok(());
        }

        let (row, col) = (index / 9, index % 9);
        let mut available_values = get_available_values(&grid, row, col);

        let mut valid_value = false;
        while !valid_value {
            if available_values.is_empty() {
                grid.set(row, col, Cell::Empty);
                return Err(NoAvailableValidValuesError);
            }
            let available_values_list = available_values.iter().cloned().collect::<Vec<_>>();
            let random_index = rand::rng().random_range(0..available_values.len());
            let value = available_values_list[random_index];
            grid.set(row, col, Cell::Value(value));
            match self.fill_grid(&mut grid, index + 1) {
                Ok(_) => valid_value = true,
                Err(_) => {
                    available_values.remove(&value);
                }
            }
        }
        Ok(())
    }

    pub fn generate(&mut self) {
        let mut grid = Grid::new();
        match self.fill_grid(&mut grid, 0) {
            Ok(_) => self.grid = grid,
            Err(_) => panic!("Failed to generate a valid sudoku board"),
        }
    }
}
