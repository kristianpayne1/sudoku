use super::cell::Cell;

pub const TOP_BORDER: &str = "╔═ ═ ═╤═ ═ ═╤═ ═ ═╦═ ═ ═╤═ ═ ═╤═ ═ ═╦═ ═ ═╤═ ═ ═╤═ ═ ═╗";
pub const BOTTOM_BORDER: &str = "╚═ ═ ═╧═ ═ ═╧═ ═ ═╩═ ═ ═╧═ ═ ═╧═ ═ ═╩═ ═ ═╧═ ═ ═╧═ ═ ═╝";
pub const MID_BORDER: &str = "╟─ ─ ─┼─ ─ ─┼─ ─ ─╫─ ─ ─┼─ ─ ─┼─ ─ ─╫─ ─ ─┼─ ─ ─┼─ ─ ─╢";

pub struct Grid([[Cell; 9]; 9]);

impl Grid {
    pub fn new() -> Self {
        Self([[Cell::Empty; 9]; 9])
    }

    pub fn num_rows(&self) -> usize {
        self.0.len()
    }

    pub fn num_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn get(&self, row: usize, col: usize) -> &Cell {
       &self.0[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.0[row][col] = cell;
    }

}
