use rand::Rng;

use crate::board::cell::Cell;
use crate::board::error::NoAvailableValidValuesError;
use crate::board::grid::Grid;
use crate::Sudoku;

impl Sudoku {
    pub fn fill_board(
        &self,
        mut grid: &mut Grid,
        index: usize,
    ) -> Result<(), NoAvailableValidValuesError> {
        let grid_size = grid.size();
        if index == grid_size * grid_size {
            return Ok(());
        }

        let (row, col) = (index / 9, index % 9);
        let mut available_values = Sudoku::get_all_available_values(&grid, row, col);

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
            match self.fill_board(&mut grid, index + 1) {
                Ok(_) => valid_value = true,
                Err(_) => {
                    available_values.remove(&value);
                }
            }
        }
        Ok(())
    }

    pub fn generate_puzzle(&mut self) -> Result<Grid, NoAvailableValidValuesError> {
        let mut grid = Grid::new();
        self.fill_board(&mut grid, 0)?;
        Ok(grid)
    }
}
