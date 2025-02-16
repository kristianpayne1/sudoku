use crate::board::cell::Cell;
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
}
