use std::env;
use std::io::{self, Read};
use std::process::ExitCode;

use sudoku_format_converter::{format_block, format_line, parse_block, parse_line};

fn main() -> ExitCode {
    let mode = env::args().nth(1);

    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("error: failed to read stdin");
        return ExitCode::from(1);
    }

    let result = match mode.as_deref() {
        Some("to-line") => parse_block(&input).map(|board| format_line(&board)),
        Some("to-block") => parse_line(&input).map(|board| format_block(&board)),
        _ => {
            eprintln!("usage: sudoku-format-converter <to-line|to-block>");
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(output) => {
            println!("{}", output);
            ExitCode::from(0)
        }
        Err(e) => {
            eprintln!("error: {}", e);
            ExitCode::from(1)
        }
    }
}
