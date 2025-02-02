use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    sudoku.set_value(0, 0, 1);
    println!("{}", sudoku.is_valid());
    sudoku.display();
}
