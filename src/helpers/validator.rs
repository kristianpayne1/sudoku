use std::collections::HashSet;
use crate::board::cell::Cell;
use crate::Sudoku;

fn is_list_valid(list: Vec<&Cell>) -> bool {
    let mut seen: HashSet<&u8> = HashSet::new();
    let mut has_duplicates = false;

    for &cell in list.iter() {
        match cell {
            Cell::Empty => (), // Do nothing
            Cell::Value(num) => {
                if seen.contains(&num) {
                    has_duplicates = true;
                }
                seen.insert(&num);
            }
        }
    }

    !has_duplicates
}

impl Sudoku {
    pub fn is_row_valid(&self, row: usize) -> bool {
        is_list_valid(self.grid.get_row(row))
    }

   pub fn is_column_valid(&self, col: usize) -> bool {
        is_list_valid(self.grid.get_col(col))
   }

   pub fn is_subgrid_valid(&self, sub_row: usize, sub_col: usize) -> bool {
       is_list_valid(self.grid.get_subgrid(sub_row, sub_col).get_all())
   }

    pub fn is_valid(&self) -> bool {
        let mut is_valid = true;
        let mut i = 0;
        while is_valid && i < 9 {
            let (sub_row, sub_col) = (i % 3, (i / 9) * 10);
            is_valid = self.is_row_valid(i) && self.is_column_valid(i) && self.is_subgrid_valid(sub_row, sub_col);
            i += 1;
        }
        is_valid
    }

    pub fn is_value_valid(&self, row: usize, col: usize) -> bool {
        let (sub_row, sub_col) = (row % 3, col % 3);
        self.is_row_correct(row) && self.is_column_correct(col) && self.is_subgrid_correct(sub_row, sub_col)
    }
}