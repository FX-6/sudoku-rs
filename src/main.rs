use std::time::Instant;

use clap::Parser;

mod backtracking;
mod check;
mod io;

#[derive(Parser, Debug)]
struct Cli {
    /// The file containing the sudoku
    ///
    /// Should be 9 lines with 9 numbers each, no spaces, 0 for empty cells
    #[arg(value_name = "FILE")]
    filename: String,

    /// Only validate the sudoku instead of solving it
    #[arg(short, long)]
    validate: bool,
}

fn main() {
    let args = Cli::parse();

    let mut sudoku = match io::read_sudoku_from_file(&args.filename) {
        Ok(sudoku) => sudoku,
        Err(_) => {
            eprintln!("Failed to read sudoku from file '{}'", args.filename);
            return;
        }
    };

    if args.validate {
        validate(&sudoku);
    } else {
        solve(&mut sudoku);
    }
}

fn validate(sudoku: &[Vec<usize>]) {
    println!("Sudoku:");
    io::print_sudoku(sudoku);
    println!();

    if check::is_sudoku_valid(sudoku, true) {
        println!("Sudoku is valid :D");
    } else {
        println!("Sudoku is invalid :(");
    }
}

fn solve(sudoku: &mut Vec<Vec<usize>>) {
    println!("Pre-solve:");
    io::print_sudoku(sudoku);
    println!();

    let start = Instant::now();

    backtracking::solve(sudoku);

    let duration = start.elapsed();

    println!("Post-solve:");
    io::print_sudoku(sudoku);
    println!();

    if check::is_sudoku_valid(sudoku, true) {
        println!("Sudoku is valid :D");
    } else {
        println!("Sudoku is invalid :(");
    }

    println!("Took {:?}", duration);
}
