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
    // Parse the args
    let args = Cli::parse();

    // Read the sudoku from the file
    let sudoku = match io::read_sudoku_from_file(&args.filename) {
        Ok(sudoku) => sudoku,
        Err(io::Error::IOError) => {
            eprintln!("Failed to read sudoku from file '{}'", args.filename);
            return;
        }
        Err(io::Error::InvalidSymbolError { row, col, char }) => {
            eprintln!(
                "Invalid character '{}' in {}:{}:{}",
                char, args.filename, row, col
            );
            return;
        }
    };

    // Only validate or also solve?
    if args.validate {
        validate(sudoku);
    } else {
        solve(sudoku);
    }
}

fn validate(sudoku: [[usize; 9]; 9]) {
    println!("Sudoku:");
    io::print_sudoku(&sudoku);
    println!();

    if check::is_valid(&sudoku, true) {
        println!("Sudoku is valid :D");
    } else {
        println!("Sudoku is invalid :(");
    }
}

fn solve(mut sudoku: [[usize; 9]; 9]) {
    println!("Pre-solve:");
    io::print_sudoku(&sudoku);
    println!();

    let start = Instant::now();

    backtracking::solve(&mut sudoku);

    let duration = start.elapsed();

    println!("Post-solve:");
    io::print_sudoku(&sudoku);
    println!();

    if check::is_valid(&sudoku, true) {
        println!("Sudoku is valid :D");
    } else {
        println!("Sudoku is invalid :(");
    }

    println!("Took {:?}", duration);
}
