use rand::Rng;

use crate::board::cell::Cell;
use crate::board::grid::Grid;
use crate::game::sudoku::Difficulty;
use crate::Sudoku;

impl Sudoku {
    pub fn generate_puzzle(&mut self, difficulty: Difficulty) -> Grid {
        let mut solution = Grid::new();
        solution
            .fill(0)
            .expect("Failed to create the solution to the puzzle");

        let mut grid = solution.clone();
        let mut i = difficulty.value();
        while i > 0 {
            let grid_size = grid.size();
            let (random_row, random_col) = (
                rand::rng().random_range(0..grid_size),
                rand::rng().random_range(0..grid_size),
            );
            if let Cell::Value(solution_value) = grid.get(random_row, random_col) {
                let mut grid_copy = grid.clone();
                grid_copy.set(random_row, random_col, Cell::Empty);
                grid_copy.display();
                if let Ok(value) = grid_copy.solve_for(random_row, random_col) {
                    if &value == solution_value {
                        grid.set(random_row, random_col, Cell::Empty);
                    }
                }
                i -= 1;
            }
        }

        self.grid = grid;
        solution
    }
}
