# sudoku-rs

Simple cli to solve (or just validate) sudokus.

The alogrithm is just backtracking (brute forcing).

The measured time is only the time the algorithm took, opening the file etc is not measured.

## Usage

View help:
- `sudoku-rs -h` for short help
- `sudoku-rs --help` for long help

Solve a sudoku:
- `sudoku-rs /path/to/sudoku.txt`

Only validate a sudoku:
- `sudoku-rs -v /path/to/sudoku.txt`

## Valid input

The input files should contain 9 lines with 9 digits each.
There should be no spaces.
Empty fields should be marked with a 0.

Example inputs can be found in `sudokus/`.
