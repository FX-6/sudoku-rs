pub fn is_valid(board: &[[usize; 9]; 9], strict: bool) -> bool {
    // Check rows
    for (row_i, row) in board.iter().enumerate() {
        if !part_is_valid(row, strict) {
            if strict {
                eprintln!("Row {} invalid: {:?}", row_i, row);
            }
            return false;
        }
    }

    // Check columns
    for col_i in 0..9 {
        let col_vec: Vec<usize> = board.iter().map(|row| row[col_i]).collect();
        let col: [usize; 9] = col_vec.try_into().unwrap();

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
            let square_vec: Vec<usize> = (0..9)
                .map(|x| board[i * 3 + x / 3][j * 3 + x % 3])
                .collect();
            let square: [usize; 9] = square_vec.try_into().unwrap();

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
