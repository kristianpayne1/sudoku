use std::collections::HashSet;
use crate::board::cell::Cell;
use crate::Sudoku;

fn is_list_valid(list: Vec<&Cell>) -> bool {
    let seen: HashSet<&u8> = HashSet::new();
    let mut required_numbers = (1..=9).collect::<HashSet<_>>();
    let mut has_duplicates = false;

    for &cell in list.iter() {
        match cell {
            Cell::Empty => return false,
            Cell::Value(num) => {
                if !seen.contains(&num) {
                    has_duplicates = true;
                }
                required_numbers.remove(&num);
            }
        }
    }

    !has_duplicates && required_numbers.is_empty()
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
}