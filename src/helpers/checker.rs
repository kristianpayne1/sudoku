use crate::board::cell::Cell;
use crate::Sudoku;
use std::collections::HashSet;

fn is_list_correct(list: Vec<&Cell>) -> bool {
    let mut seen: HashSet<&u8> = HashSet::new();
    let mut required_numbers = (1..=9).collect::<HashSet<_>>();
    let mut has_duplicates = false;

    for &cell in list.iter() {
        match cell {
            Cell::Empty => return false,
            Cell::Value(num) => {
                if seen.contains(&num) {
                    has_duplicates = true;
                }
                seen.insert(&num);
                required_numbers.remove(&num);
            }
        }
    }

    !has_duplicates && required_numbers.is_empty()
}

impl Sudoku {
    pub fn is_row_correct(&self, row: usize) -> bool {
        is_list_correct(self.grid.get_row(row))
    }

    pub fn is_column_correct(&self, col: usize) -> bool {
        is_list_correct(self.grid.get_col(col))
    }

    pub fn is_subgrid_correct(&self, sub_row: usize, sub_col: usize) -> bool {
        is_list_correct(self.grid.get_subgrid(sub_row, sub_col).get_all())
    }

    pub fn is_correct(&self) -> bool {
        let mut is_correct = true;
        let mut i = 0;
        while is_correct && i < 9 {
            let (sub_row, sub_col) = (i % 3, (i / 9) * 10);
            is_correct = self.is_row_correct(i)
                && self.is_column_correct(i)
                && self.is_subgrid_correct(sub_row, sub_col);
            i += 1;
        }
        is_correct
    }
}
