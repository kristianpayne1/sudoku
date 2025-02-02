use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    sudoku.set_value(1, 0, 0);
    println!("{}", sudoku.is_valid());
    sudoku.display();
}
