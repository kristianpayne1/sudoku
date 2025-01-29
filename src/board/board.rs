use crate::game::{Sudoku};
use super::grid::{TOP_BORDER, BOTTOM_BORDER, MID_BORDER};

impl Sudoku {
    pub fn display(&self) {
        let num_rows: usize = self.grid.num_rows() * 2 + 1;
        let num_cols: usize = self.grid.num_cols() * 6 + 1;

        for row in 0..num_rows {
            match row {
                0 => println!("{}", TOP_BORDER),
                r if r == num_rows - 1 => println!("{}", BOTTOM_BORDER),
                r if r % 2 == 0 => println!("{}", MID_BORDER),
                _ => {
                    let mut row_str = String::new();
                    for col in 0..num_cols {
                        row_str.push(match col {
                            c if c % 18 == 0 => '║',
                            c if c % 6 == 0 => '│',
                            c if c % 3 == 0 => format!("{}", self.grid.get((row - 1) / 2, (col - 1) / 6)).parse().unwrap(),
                            _ => ' ',
                        });
                    }
                    println!("{}", row_str);
                }
            }
        }
    }
}
