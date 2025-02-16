use rand::Rng;

use crate::board::cell::Cell;
use crate::board::error::NoAvailableValidValuesError;
use crate::board::grid::Grid;
use crate::game::sudoku::Difficulty;
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
        if let Cell::Value(_) = grid.get(row, col) {
            return self.fill_board(grid, index + 1);
        }
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

    pub fn generate_puzzle(
        &mut self,
        difficulty: Difficulty,
    ) -> Result<Grid, NoAvailableValidValuesError> {
        let mut solution = Grid::new();
        self.fill_board(&mut solution, 0)?;
        solution.display();
        let mut grid = solution.clone();

        let mut i = difficulty.value();
        while i > 0 {
            let grid_size = grid.size();
            let (random_row, random_col) = (
                rand::rng().random_range(0..grid_size),
                rand::rng().random_range(0..grid_size),
            );
            if let Cell::Value(_) = grid.get(random_row, random_col) {
                let mut grid_copy = grid.clone();
                grid_copy.set(random_row, random_col, Cell::Empty);
                self.fill_board(&mut grid_copy, 0)?;
                if grid_copy == solution {
                    grid.set(random_row, random_col, Cell::Empty);
                    i -= 1;
                }
            }
        }
        Ok(grid)
    }
}
