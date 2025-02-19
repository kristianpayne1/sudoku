use crate::board::cell::Cell;
use crate::board::error::NoAvailableValidValuesError;
use crate::board::grid::Grid;
use crate::Sudoku;
use rand::Rng;
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

impl Grid {
    pub fn get_all_available_values(self, row: usize, col: usize) -> HashSet<u8> {
        let subgrid = self.get_subgrid(row / 3, col / 3);
        let row_list = self.get_row(row);
        let column_list = self.get_col(col);
        let cells: Vec<&Cell> = row_list
            .into_iter()
            .chain(column_list)
            .chain(subgrid.get_all())
            .collect();

        get_available_values(&cells)
    }

    pub fn fill(&mut self, index: usize) -> Result<(), NoAvailableValidValuesError> {
        let grid_size = self.size();
        if index == grid_size * grid_size {
            return Ok(());
        }

        let (row, col) = (index / 9, index % 9);
        if let Cell::Value(_) = self.get(row, col) {
            return self.fill(index + 1);
        }
        let mut available_values = self.get_all_available_values(row, col);

        let mut valid_value = false;
        while !valid_value {
            if available_values.is_empty() {
                self.set(row, col, Cell::Empty);
                return Err(NoAvailableValidValuesError);
            }
            let available_values_list = available_values.iter().cloned().collect::<Vec<_>>();
            let random_index = rand::rng().random_range(0..available_values.len());
            let value = available_values_list[random_index];
            self.set(row, col, Cell::Value(value));
            match self.fill(index + 1) {
                Ok(_) => valid_value = true,
                Err(_) => {
                    available_values.remove(&value);
                }
            }
        }
        Ok(())
    }

    pub fn solve_for(&mut self, row: usize, col: usize) -> Result<u8, NoAvailableValidValuesError> {
        let cell = self.get(row, col);
        if let Cell::Value(value) = cell {
            return Ok(value.clone());
        }

        let all_available_values = self.get_all_available_values(row, col);
        if all_available_values.len() == 1 {
            return Ok(all_available_values.into_iter().next().unwrap());
        }

        let (sub_row, sub_col) = (row / 3, col / 3);
        let subgrid = self.get_subgrid(sub_row, sub_col);
        let mut possible_values: HashSet<u8> = HashSet::new();
        for available_value in all_available_values {
            'outer: for i in 0..subgrid.size() {
                for j in 0..subgrid.size() {
                    let cell = subgrid.get(i, j);
                    if let Cell::Empty = cell {
                        let all_available_subgrid_cell_values =
                            self.get_all_available_values(sub_row + i, sub_col + j);
                        if !all_available_subgrid_cell_values.contains(&available_value)
                            && !possible_values.contains(&available_value)
                        {
                            possible_values.insert(available_value);
                        } else if all_available_subgrid_cell_values.contains(&available_value) {
                            possible_values.remove(&available_value);
                            break 'outer;
                        }
                    }
                }
            }
        }
        if possible_values.len() == 1 {
            return Ok(possible_values.into_iter().next().unwrap());
        }
        Err(NoAvailableValidValuesError)
    }
}

impl Sudoku {
    pub fn solve(&mut self) {
        match self.grid.fill(0) {
            Err(NoAvailableValidValuesError) => {
                println!("No solutions found!");
            }
            Ok(_) => {
                self.grid.display();
                println!("Successfully solved!");
            }
        }
    }

    pub fn solve_for(&mut self, row: usize, col: usize) {
        match self.grid.solve_for(row, col) {
            Err(NoAvailableValidValuesError) => {
                println!("No solutions found!");
            }
            Ok(value) => {
                self.grid.set(row, col, Cell::Value(value));
                self.grid.display();
                println!("Found value for row {}, column {}: {}", row, col, value);
            }
        }
    }
}
