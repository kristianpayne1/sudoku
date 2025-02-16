use crate::board::grid::Grid;

pub const TOP_BORDER: &str = "╔═ ═ ═╤═ ═ ═╤═ ═ ═╦═ ═ ═╤═ ═ ═╤═ ═ ═╦═ ═ ═╤═ ═ ═╤═ ═ ═╗";
pub const BOTTOM_BORDER: &str = "╚═ ═ ═╧═ ═ ═╧═ ═ ═╩═ ═ ═╧═ ═ ═╧═ ═ ═╩═ ═ ═╧═ ═ ═╧═ ═ ═╝";
pub const MID_BORDER: &str = "╟─ ─ ─┼─ ─ ─┼─ ─ ─╫─ ─ ─┼─ ─ ─┼─ ─ ─╫─ ─ ─┼─ ─ ─┼─ ─ ─╢";

impl Grid {
    pub fn display(&self) {
        let num_rows: usize = self.get_col(0).len() * 2 + 1;
        let num_cols: usize = self.get_row(0).len() * 6 + 1;

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
                            c if c % 3 == 0 => {
                                format!("{}", self.get((row - 1) / 2, (col - 1) / 6))
                                    .parse()
                                    .unwrap()
                            }
                            _ => ' ',
                        });
                    }
                    println!("{}", row_str);
                }
            }
        }
    }
}
