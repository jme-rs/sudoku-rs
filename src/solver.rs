use crate::error::SudokuError;

const SIZE: usize = 9;

/// # struct Sudoku
///
/// This contains numbers field and functions.
#[derive(Debug)]
pub struct Sudoku {
    pub field: [[Option<u8>; SIZE]; SIZE],
    pub options: [[Vec<u8>; SIZE]; SIZE],
}

impl Sudoku {
    /// Create a new Sudoku instance.
    pub fn new(initial_field: &[[u8; SIZE]; SIZE]) -> Result<Self, SudokuError> {
        let mut field = [[None; SIZE]; SIZE];

        for y in 0..SIZE {
            for x in 0..SIZE {
                match initial_field[y][x] {
                    0 => field[y][x] = None,
                    1..=9 => field[y][x] = Some(initial_field[y][x]),
                    _ => {
                        return Err(SudokuError::InvalidInputValue {
                            y,
                            x,
                            value: initial_field[y][x],
                        })
                    }
                }
            }
        }

        Ok(Self {
            field,
            options: Default::default(),
        })
    }

    /// Solve the Sudoku.
    pub fn solve(&mut self) -> Result<(), SudokuError> {
        loop {
            match self.step()? {
                0 => break,
                _ => continue,
            }
        }

        Ok(())
    }

    /// Solve the Sudoku step by step.
    /// Return the number of cells that have been determined.
    pub fn step(&mut self) -> Result<usize, SudokuError> {
        if self.is_solved() {
            return Ok(0);
        }

        let start = self.field.iter().flatten().filter(|&n| n.is_none()).count();
        self.update_options();
        self.apply_sole_candidate();
        self.apply_row_reduction();
        self.apply_column_reduction();
        self.apply_box_reduction();
        let end = self.field.iter().flatten().filter(|&n| n.is_none()).count();

        if start == end {
            return Err(SudokuError::UnsolvableSudoku);
        }

        Ok(start - end)
    }

    /// Return true if the Sudoku is solved.
    fn is_solved(&self) -> bool {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.field[y][x].is_none() {
                    return false;
                }
            }
        }

        true
    }

    /// If there is only one option in a cell, determine the cell number.
    fn apply_sole_candidate(&mut self) {
        dbg!("apply_sole_candidate");

        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.options[y][x].len() == 1 {
                    self.determine(y, x, self.options[y][x][0]);
                }
            }
        }
    }

    /// If there is only one cell in a row that can be filled with a number, determine the cell number.
    fn apply_row_reduction(&mut self) {
        dbg!("apply_row_reduction");

        for y in 0..SIZE {
            let row_options = self.get_row_options(y);

            for x in 0..SIZE {
                if self.field[y][x].is_some() {
                    continue;
                }

                for option in self.options[y][x].iter() {
                    let count = row_options.iter().filter(|&n| *n == *option).count();
                    if count == 1 {
                        self.determine(y, x, *option);
                        break;
                    }
                }
            }
        }
    }

    /// If there is only one cell in a column that can be filled with a number, determine the cell number.
    fn apply_column_reduction(&mut self) {
        dbg!();

        for x in 0..SIZE {
            let column_options = self.get_column_options(x);

            for y in 0..SIZE {
                if self.field[y][x].is_some() {
                    continue;
                }

                for option in self.options[y][x].iter() {
                    let count = column_options.iter().filter(|&n| *n == *option).count();
                    if count == 1 {
                        self.determine(y, x, *option);
                        break;
                    }
                }
            }
        }
    }

    /// If there is only one cell in a box that can be filled with a number, determine the cell number.
    fn apply_box_reduction(&mut self) {
        dbg!("apply_box_reduction");

        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.field[y][x].is_some() {
                    continue;
                }

                let box_options = self.get_box_options(y, x);

                for option in self.options[y][x].iter() {
                    let count = box_options.iter().filter(|&n| *n == *option).count();
                    if count == 1 {
                        self.determine(y, x, *option);
                        break;
                    }
                }
            }
        }
    }

    /// Update options in all cells.
    fn update_options(&mut self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                self.options[y][x].clear();
                if self.field[y][x].is_some() {
                    continue;
                }
                for option in 1..=SIZE as u8 {
                    if !self.get_row_values(y).contains(&option)
                        && !self.get_column_values(x).contains(&option)
                        && !self.get_box_values(y, x).contains(&option)
                    {
                        self.options[y][x].push(option);
                    }
                }
            }
        }
    }

    /// Determine cell number and clear cell options.
    fn determine(&mut self, y: usize, x: usize, value: u8) {
        self.field[y][x] = Some(value);
        println!(
            "Determined ({}, {}) = {} {:?}",
            y, x, value, self.options[y][x]
        );
        self.options[y][x].clear();
    }

    /// Return available options in a row.
    fn get_row_options(&self, y: usize) -> Vec<u8> {
        let mut row_options = vec![];

        for x in 0..SIZE {
            if self.field[y][x].is_none() {
                row_options.extend(self.options[y][x].iter());
            }
        }

        row_options
    }

    /// Return available options in a column.
    fn get_column_options(&self, x: usize) -> Vec<u8> {
        let mut column_options = vec![];

        for y in 0..SIZE {
            if self.field[y][x].is_none() {
                column_options.extend(self.options[y][x].iter());
            }
        }

        column_options
    }

    /// Return available options in a box.
    fn get_box_options(&self, y: usize, x: usize) -> Vec<u8> {
        let mut box_options = vec![];

        let y_start = y / 3 * 3;
        let x_start = x / 3 * 3;

        for y in y_start..y_start + 3 {
            for x in x_start..x_start + 3 {
                if self.field[y][x].is_none() {
                    box_options.extend(self.options[y][x].iter());
                }
            }
        }

        box_options
    }

    pub fn print(&self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if let Some(value) = self.field[y][x] {
                    print!("{} ", value);
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }

    fn get_row_values(&self, y: usize) -> Vec<u8> {
        let mut row_values = vec![];

        for x in 0..SIZE {
            if let Some(value) = self.field[y][x] {
                row_values.push(value);
            }
        }

        row_values
    }

    fn get_column_values(&self, x: usize) -> Vec<u8> {
        let mut column_values = vec![];

        for y in 0..SIZE {
            if let Some(value) = self.field[y][x] {
                column_values.push(value);
            }
        }

        column_values
    }

    fn get_box_values(&self, y: usize, x: usize) -> Vec<u8> {
        let mut box_values = vec![];

        let y_start = y / 3 * 3;
        let x_start = x / 3 * 3;

        for y in y_start..y_start + 3 {
            for x in x_start..x_start + 3 {
                if let Some(value) = self.field[y][x] {
                    box_values.push(value);
                }
            }
        }

        box_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIELD_1: [[u8; SIZE]; SIZE] = [
        [0, 0, 0, 0, 6, 0, 5, 0, 0],
        [0, 0, 2, 0, 0, 0, 0, 0, 4],
        [0, 1, 0, 3, 0, 0, 0, 9, 0],
        [0, 3, 4, 5, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 0, 0, 0, 0, 9, 8, 1, 0],
        [0, 5, 0, 0, 0, 8, 0, 3, 0],
        [3, 0, 0, 0, 0, 0, 9, 0, 0],
        [0, 0, 6, 0, 1, 0, 0, 0, 0],
    ];

    #[test]
    fn test_new() {
        let sudoku = Sudoku::new(&FIELD_1).unwrap();
        assert_eq!(sudoku.field[0][0], None);
        assert_eq!(sudoku.field[0][4], Some(6));
        assert_eq!(sudoku.field[3][1], Some(3));
        assert_eq!(sudoku.field[8][8], None);
    }

    #[test]
    fn test_determine() {
        let mut sudoku = Sudoku::new(&FIELD_1).unwrap();
        sudoku.determine(0, 0, 1);

        assert_eq!(sudoku.field[0][0], Some(1));
        assert_eq!(sudoku.options[0][0], vec![]);
    }

    #[test]
    fn test_is_solved() {
        let field = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [4, 5, 6, 7, 8, 9, 1, 2, 3],
            [7, 8, 9, 1, 2, 3, 4, 5, 6],
            [2, 3, 4, 5, 6, 7, 8, 9, 1],
            [5, 6, 7, 8, 9, 1, 2, 3, 4],
            [8, 9, 1, 2, 3, 4, 5, 6, 7],
            [3, 4, 5, 6, 7, 8, 9, 1, 2],
            [6, 7, 8, 9, 1, 2, 3, 4, 5],
            [9, 1, 2, 3, 4, 5, 6, 7, 8],
        ];

        let sudoku = Sudoku::new(&field).unwrap();

        assert!(sudoku.is_solved());
    }

    #[test]
    fn test_update_options() {
        let mut sudoku = Sudoku::new(&FIELD_1).unwrap();
        sudoku.update_options();

        assert_eq!(sudoku.options[1][6], vec![1, 3, 6, 7]);
        assert_eq!(sudoku.options[4][8], vec![2, 3, 5, 7, 9]);
        assert_eq!(sudoku.options[6][1], vec![]);
    }

    #[test]
    fn test_apply_row_reduction() {
        let mut sudoku = Sudoku::new(&FIELD_1).unwrap();
        sudoku.print();
        sudoku.update_options();
        sudoku.apply_row_reduction();

        sudoku.print();

        assert_eq!(sudoku.field[1][6], Some(3));
        assert_eq!(sudoku.field[3][0], Some(9));
        assert_eq!(sudoku.field[4][4], None);
    }

    #[test]
    fn test_apply_column_reduction() {
        let mut sudoku = Sudoku::new(&FIELD_1).unwrap();
        sudoku.update_options();
        sudoku.apply_row_reduction();

        sudoku.update_options();
        sudoku.apply_column_reduction();

        assert_eq!(sudoku.field[5][8], Some(3));
        assert_eq!(sudoku.field[6][2], Some(9));
        assert_eq!(sudoku.field[0][5], None);
    }

    #[test]
    fn test_apply_box_reduction() {
        let mut sudoku = Sudoku::new(&FIELD_1).unwrap();
        sudoku.update_options();
        sudoku.apply_row_reduction();

        sudoku.update_options();
        sudoku.apply_column_reduction();

        sudoku.update_options();
        sudoku.apply_box_reduction();

        assert_eq!(sudoku.field[0][8], Some(1));
        assert_eq!(sudoku.field[4][7], Some(5));
        assert_eq!(sudoku.field[3][6], None);
    }

    #[test]
    fn test_solve() {
        let mut sudoku = Sudoku::new(&FIELD_1).unwrap();
        sudoku.print();
        sudoku.step().unwrap();
        sudoku.print();
    }
}
