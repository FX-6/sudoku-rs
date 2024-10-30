pub fn is_sudoku_valid(board: &[Vec<usize>], strict: bool) -> bool {
    // Check rows
    for (i, row) in board.iter().enumerate() {
        if !is_valid(row, strict) {
            if strict {
                eprintln!("Row {} invalid: {:?}", i, row);
            }
            return false;
        }
    }

    // Check columns
    for i in 0..9 {
        let column: Vec<usize> = board.iter().map(|row| row[i]).collect();

        if !is_valid(&column, strict) {
            if strict {
                eprintln!("Column {} invalid: {:?}", i, column);
            }
            return false;
        }
    }

    // Check 3x3 squares
    for i in 0..3 {
        for j in 0..3 {
            let square: Vec<usize> = (0..9)
                .map(|x| board[i * 3 + x / 3][j * 3 + x % 3])
                .collect();

            if !is_valid(&square, strict) {
                if strict {
                    eprintln!("Square {}:{} invalid: {:?}", j, i, square);
                }
                return false;
            }
        }
    }

    true
}

fn is_valid(values: &[usize], strict: bool) -> bool {
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
