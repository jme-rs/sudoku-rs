use std::fmt;

#[derive(Debug)]
pub enum SudokuError {
    InvalidInputValue { y: usize, x: usize, value: u8 },
    UnsolvableSudoku,
}

impl fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SudokuError::InvalidInputValue { y, x, value } => {
                write!(f, "Invalid input value at ({}, {}): {}", y, x, value)
            }
            SudokuError::UnsolvableSudoku => write!(f, "Unsolvable Sudoku"),
        }
    }
}
