use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    sudoku.generate();
    println!("Is correct: {}", sudoku.is_correct());
    println!("Is valid: {}", sudoku.is_valid());
    sudoku.display();
}
