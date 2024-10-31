use crate::check;

pub fn solve(sudoku: &mut [[usize; 9]; 9]) {
    solve_recursive(sudoku, 0, 0);
}

fn solve_recursive(sudoku: &mut [[usize; 9]; 9], row: usize, col: usize) -> bool {
    if row > 8 || col > 8 {
        return true;
    }

    let (next_row, next_col) = if col == 8 {
        (row + 1, 0)
    } else {
        (row, col + 1)
    };

    if sudoku[row][col] == 0 {
        for num in 1..10 {
            sudoku[row][col] = num;

            if check::is_valid(sudoku, false) && solve_recursive(sudoku, next_row, next_col) {
                return true;
            }
        }

        sudoku[row][col] = 0;
        return false;
    }

    solve_recursive(sudoku, next_row, next_col)
}
