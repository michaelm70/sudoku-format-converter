use std::env;
use std::io::{self, Read};
use std::process::ExitCode;

use sudoku_format_converter::{
    detect_format, format_block, format_line, parse_block, parse_line, validate, Format,
};

fn main() -> ExitCode {
    let mode = env::args().nth(1);

    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("error: failed to read stdin");
        return ExitCode::from(1);
    }

    // `source_format` is which format the *input* was in, so we know which
    // conversion to run once parsing succeeds.
    let source_format = match mode.as_deref() {
        Some("to-line") => Format::Block,
        Some("to-block") => Format::Line,
        Some(other) => {
            eprintln!("error: unrecognized mode '{}'", other);
            eprintln!("usage: sudoku-format-converter [to-line|to-block]");
            return ExitCode::from(2);
        }
        None => match detect_format(&input) {
            Some(format) => format,
            None => {
                eprintln!(
                    "error: could not tell whether input is line or block format; pass to-line or to-block explicitly"
                );
                return ExitCode::from(2);
            }
        },
    };

    let board = match source_format {
        Format::Line => parse_line(&input),
        Format::Block => parse_block(&input),
    };

    let board = match board {
        Ok(board) => board,
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::from(1);
        }
    };

    for conflict in validate(&board) {
        eprintln!("warning: {}", conflict);
    }

    let output = match source_format {
        Format::Line => format_block(&board),
        Format::Block => format_line(&board),
    };
    println!("{}", output);
    ExitCode::from(0)
}
