use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    match sudoku.generate_puzzle() {
        Ok(grid) => sudoku.grid = grid,
        Err(err) => println!("{}", err),
    }
    println!("Is correct: {}", sudoku.is_correct());
    println!("Is valid: {}", sudoku.is_valid());
    sudoku.display();
}
