# sudoku-rs

A sudoku solver written in Rust.

## Usage

```rust
use sudoku_rs::Solver;

fn main() {
    let field: [[u8; 9]; 9] = [
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

    let mut solver = Solver::new(&field).unwrap();

    // solver.step()
    // or
    solver.solve();
}
```
