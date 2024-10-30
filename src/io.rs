pub fn read_sudoku_from_file(path: &str) -> Result<Vec<Vec<usize>>, std::io::Error> {
    let file = std::fs::read_to_string(path)?;

    Ok(file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect())
}

pub fn print_sudoku(board: &[Vec<usize>]) {
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
