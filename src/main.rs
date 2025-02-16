use sudoku::game::sudoku::Difficulty;
use sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    let solution = sudoku.generate_puzzle(Difficulty::Hard);
    sudoku.display();
    solution.display();
}
