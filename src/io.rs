pub enum Error {
    IOError,
    InvalidSymbolError { row: usize, col: usize, char: char },
}

pub fn read_sudoku_from_file(path: &str) -> Result<[[usize; 9]; 9], Error> {
    // Open the file
    let file = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => return Err(Error::IOError),
    };

    // Empty sudoku grid
    let mut grid: [[usize; 9]; 9] = [[0; 9]; 9];

    // For each char in the 9x9 grid in the file
    for (row_i, row) in file.lines().take(9).enumerate() {
        for (col_i, cell) in row.chars().take(9).enumerate() {
            // Try to turn the char into a number
            let digit = match cell.to_digit(10) {
                Some(digit) => digit as usize,
                None => {
                    return Err(Error::InvalidSymbolError {
                        row: row_i,
                        col: col_i,
                        char: cell,
                    })
                }
            };

            // Add the number to the empty grid
            grid[row_i][col_i] = digit;
        }
    }

    // Return the created grid
    Ok(grid)
}

pub fn print_sudoku(board: &[[usize; 9]; 9]) {
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print!("{} ", cell);
            if (j + 1) % 3 == 0 && j + 1 != 9 {
                print!("| ");
            }
        }
        if (i + 1) % 3 == 0 && i + 1 != 9 {
            println!();
            println!("------+-------+------");
        } else {
            println!();
        }
    }
}
