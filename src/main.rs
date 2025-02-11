use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    match sudoku.generate_puzzle() {
        Ok(grid) => sudoku.grid = grid,
        Err(err) => println!("{}", err),
    }
    sudoku.display();
    println!("Valid {}", sudoku.is_valid());
    println!("Correct {}", sudoku.is_correct());
}
