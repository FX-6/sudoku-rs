pub fn is_valid(board: &[[usize; 9]; 9], strict: bool) -> bool {
    // Check rows
    #[allow(clippy::needless_range_loop)]
    for row_i in 0..9 {
        if !part_is_valid(&board[row_i], strict) {
            if strict {
                eprintln!("Row {} invalid: {:?}", row_i, &board[row_i]);
            }
            return false;
        }
    }

    // Check columns
    for col_i in 0..9 {
        let mut col = [0; 9];
        for row_i in 0..9 {
            col[row_i] = board[row_i][col_i];
        }

        if !part_is_valid(&col, strict) {
            if strict {
                eprintln!("Column {} invalid: {:?}", col_i, col);
            }
            return false;
        }
    }

    // Check 3x3 squares
    for i in 0..3 {
        for j in 0..3 {
            let mut square = [0; 9];
            for k in 0..9 {
                square[k] = board[i * 3 + k / 3][j * 3 + k % 3];
            }

            if !part_is_valid(&square, strict) {
                if strict {
                    eprintln!("Square {}:{} invalid: {:?}", j, i, square);
                }
                return false;
            }
        }
    }

    true
}

fn part_is_valid(values: &[usize; 9], strict: bool) -> bool {
    let mut seen = [false; 9];

    for &value in values {
        if value == 0 {
            if strict {
                return false;
            } else {
                continue;
            }
        }

        if seen[value - 1] {
            return false;
        }

        seen[value - 1] = true;
    }

    true
}
