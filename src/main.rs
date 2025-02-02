use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    sudoku.set_value(1, 8, 8);
    sudoku.display();
}
