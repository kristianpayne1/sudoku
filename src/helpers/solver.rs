use crate::board::cell::Cell;
use crate::board::error::NoSolutionFoundError;
use crate::board::grid::Grid;
use crate::Sudoku;
use std::collections::HashSet;

fn get_available_values(cells: &Vec<&Cell>) -> HashSet<u8> {
    let mut available_values: HashSet<u8> = (1..=9).collect();
    for cell in cells {
        if let Cell::Value(value) = cell {
            available_values.remove(value);
        }
    }
    available_values
}

fn get_duplicates(cells: Vec<&Cell>) -> HashSet<u8> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for &cell in cells {
        if let Cell::Value(num) = cell {
            if !seen.insert(num) {
                duplicates.insert(num);
            }
        }
    }
    duplicates
}

impl Sudoku {
    pub fn get_all_available_values(grid: &Grid, row: usize, col: usize) -> HashSet<u8> {
        let subgrid = grid.get_subgrid(row / 3, col / 3);
        let row_list = grid.get_row(row);
        let column_list = grid.get_col(col);
        let cells: Vec<&Cell> = row_list
            .into_iter()
            .chain(column_list)
            .chain(subgrid.get_all())
            .collect();

        get_available_values(&cells)
    }

    pub fn solve_for(
        &self,
        grid: Grid,
        row: usize,
        col: usize,
    ) -> Result<u8, NoSolutionFoundError> {
        let row_list = grid.get_row(row);
        let col_list = grid.get_col(col);
        let sub_grid = grid.get_subgrid(row / 3, col / 3);

        // Check if only one value is available in row, column, or subgrid
        for list in [&row_list, &col_list, &sub_grid.get_all()] {
            let available_values = get_available_values(list);
            if available_values.len() == 1 {
                return Ok(available_values.iter().next().unwrap().clone());
            }
        }

        let all_available_values = Self::get_all_available_values(&grid, row, col);
        let (sub_row, sub_col) = (row % 3, col % 3);

        let row_neighbors = [
            grid.get_row((sub_row + 1) % 3),
            grid.get_row((sub_row + 2) % 3),
        ];
        let col_neighbors = [
            grid.get_col((sub_col + 1) % 3),
            grid.get_col((sub_col + 2) % 3),
        ];

        let row_neighbors_duplicates = get_duplicates(row_neighbors.concat());
        let col_neighbors_duplicates = get_duplicates(col_neighbors.concat());

        // Check for unique intersections
        if sub_grid.get_col(sub_col).len() == 2 {
            for &value in &all_available_values {
                if row_neighbors_duplicates.contains(&value) {
                    return Ok(value);
                }
            }
        }

        if sub_grid.get_row(sub_row).len() == 2 {
            for &value in &all_available_values {
                if col_neighbors_duplicates.contains(&value) {
                    return Ok(value);
                }
            }
        }

        for value in all_available_values {
            if col_neighbors_duplicates.contains(&value)
                && row_neighbors_duplicates.contains(&value)
            {
                return Ok(value);
            }
        }

        Err(NoSolutionFoundError)
    }
}
