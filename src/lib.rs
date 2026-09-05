//! Converts sudoku boards between two plain-text formats:
//!
//! - "line": 81 characters on one line, `.` or `0` for a blank cell, `1`-`9`
//!   for a given digit. This is what most puzzle databases and solvers use.
//! - "block": 9 rows of 3 space-separated groups of 3 digits, with a blank
//!   line between each band of 3 rows. This is what people write by hand or
//!   paste into a forum post.
//!
//! Neither format encodes anything about whether the puzzle is valid or
//! solvable; this crate only moves cells between representations.

pub type Board = [u8; 81];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConvertError {
    WrongLength { expected: usize, found: usize },
    WrongRowCount { expected: usize, found: usize },
    InvalidChar { row: usize, col: usize, ch: char },
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::WrongLength { expected, found } => {
                write!(f, "expected {} characters, found {}", expected, found)
            }
            ConvertError::WrongRowCount { expected, found } => {
                write!(f, "expected {} rows, found {}", expected, found)
            }
            ConvertError::InvalidChar { row, col, ch } => {
                write!(f, "invalid character '{}' at row {}, column {}", ch, row, col)
            }
        }
    }
}

impl std::error::Error for ConvertError {}

fn cell_from_char(ch: char) -> Option<u8> {
    match ch {
        // '0' shows up in the wild almost as often as '.' for a blank cell,
        // so both are accepted on the way in.
        '.' | '0' => Some(0),
        '1'..='9' => Some(ch as u8 - b'0'),
        _ => None,
    }
}

fn char_from_cell(cell: u8) -> char {
    if cell == 0 {
        '.'
    } else {
        (b'0' + cell) as char
    }
}

/// Parses a board from the compact line format. Leading and trailing
/// whitespace (including a trailing newline) is ignored, but the remaining
/// content must be exactly 81 characters with no embedded whitespace.
pub fn parse_line(input: &str) -> Result<Board, ConvertError> {
    let trimmed = input.trim();
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() != 81 {
        return Err(ConvertError::WrongLength {
            expected: 81,
            found: chars.len(),
        });
    }
    let mut board: Board = [0; 81];
    for (i, ch) in chars.iter().enumerate() {
        board[i] = cell_from_char(*ch).ok_or(ConvertError::InvalidChar {
            row: i / 9,
            col: i % 9,
            ch: *ch,
        })?;
    }
    Ok(board)
}

/// Formats a board as the compact line format: 81 characters, no newline.
pub fn format_line(board: &Board) -> String {
    board.iter().map(|&c| char_from_cell(c)).collect()
}

/// Parses a board from the block format. Blank lines are ignored wherever
/// they appear, so band separators are optional and extra blank lines at the
/// start or end are harmless. Whitespace within a row (the gaps between the
/// three groups of three) is stripped before the row is checked, so it must
/// contain exactly 9 remaining characters.
pub fn parse_block(input: &str) -> Result<Board, ConvertError> {
    let rows: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    if rows.len() != 9 {
        return Err(ConvertError::WrongRowCount {
            expected: 9,
            found: rows.len(),
        });
    }
    let mut board: Board = [0; 81];
    for (row_idx, row) in rows.iter().enumerate() {
        let cells: Vec<char> = row.chars().filter(|c| !c.is_whitespace()).collect();
        if cells.len() != 9 {
            return Err(ConvertError::WrongLength {
                expected: 9,
                found: cells.len(),
            });
        }
        for (col_idx, ch) in cells.iter().enumerate() {
            let cell = cell_from_char(*ch).ok_or(ConvertError::InvalidChar {
                row: row_idx,
                col: col_idx,
                ch: *ch,
            })?;
            board[row_idx * 9 + col_idx] = cell;
        }
    }
    Ok(board)
}

/// Formats a board as the block format, with a blank line after the 3rd and
/// 6th rows and no trailing newline.
pub fn format_block(board: &Board) -> String {
    let mut out = String::new();
    for row in 0..9 {
        let mut groups = Vec::with_capacity(3);
        for group in 0..3 {
            let start = row * 9 + group * 3;
            let group_str: String = board[start..start + 3]
                .iter()
                .map(|&c| char_from_cell(c))
                .collect();
            groups.push(group_str);
        }
        out.push_str(&groups.join(" "));
        out.push('\n');
        if row % 3 == 2 && row != 8 {
            out.push('\n');
        }
    }
    out.pop(); // drop the final newline, to match format_line's style
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_board() -> Board {
        [0; 81]
    }

    fn cycling_board() -> Board {
        let mut b: Board = [0; 81];
        for (i, cell) in b.iter_mut().enumerate() {
            *cell = ((i % 9) + 1) as u8;
        }
        b
    }

    fn shuffled_board() -> Board {
        let mut b: Board = [0; 81];
        for (i, cell) in b.iter_mut().enumerate() {
            *cell = ((i * 7 % 9) + 1) as u8;
        }
        b
    }

    fn sparse_board() -> Board {
        let mut b: Board = [0; 81];
        for (i, cell) in b.iter_mut().enumerate() {
            *cell = if i % 3 == 0 { 0 } else { ((i % 9) + 1) as u8 };
        }
        b
    }

    struct RoundTripCase {
        name: &'static str,
        board: Board,
    }

    fn round_trip_cases() -> Vec<RoundTripCase> {
        vec![
            RoundTripCase {
                name: "all blank",
                board: blank_board(),
            },
            RoundTripCase {
                name: "mixed blanks and digits",
                board: sparse_board(),
            },
            RoundTripCase {
                name: "no blanks, repeating pattern",
                board: cycling_board(),
            },
            RoundTripCase {
                name: "no blanks, shuffled pattern",
                board: shuffled_board(),
            },
        ]
    }

    #[test]
    fn line_format_round_trips() {
        for case in round_trip_cases() {
            let line = format_line(&case.board);
            assert_eq!(line.len(), 81, "case '{}': line length", case.name);
            let parsed =
                parse_line(&line).unwrap_or_else(|e| panic!("case '{}': {}", case.name, e));
            assert_eq!(parsed, case.board, "case '{}'", case.name);
        }
    }

    #[test]
    fn block_format_round_trips() {
        for case in round_trip_cases() {
            let block = format_block(&case.board);
            let parsed =
                parse_block(&block).unwrap_or_else(|e| panic!("case '{}': {}", case.name, e));
            assert_eq!(parsed, case.board, "case '{}'", case.name);
        }
    }

    #[test]
    fn block_format_accepts_zero_as_blank_marker() {
        let board = sparse_board();
        let block = format_block(&board).replace('.', "0");
        assert_eq!(parse_block(&block).unwrap(), board);
    }

    #[test]
    fn block_format_ignores_carriage_returns() {
        let board = sparse_board();
        let block = format_block(&board).replace('\n', "\r\n");
        assert_eq!(parse_block(&block).unwrap(), board);
    }

    #[test]
    fn block_format_tolerates_extra_blank_lines() {
        let board = sparse_board();
        let block = format_block(&board);
        let noisy = format!("\n\n{}\n\n\n", block);
        assert_eq!(parse_block(&noisy).unwrap(), board);
    }

    #[test]
    fn block_format_tolerates_missing_band_separators() {
        let board = sparse_board();
        let block = format_block(&board).replace("\n\n", "\n");
        assert_eq!(parse_block(&block).unwrap(), board);
    }

    #[test]
    fn line_format_rejects_wrong_length() {
        assert_eq!(
            parse_line(&".".repeat(80)),
            Err(ConvertError::WrongLength {
                expected: 81,
                found: 80
            })
        );
        assert_eq!(
            parse_line(&".".repeat(82)),
            Err(ConvertError::WrongLength {
                expected: 81,
                found: 82
            })
        );
    }

    #[test]
    fn line_format_rejects_invalid_character() {
        let mut chars: Vec<char> = ".".repeat(81).chars().collect();
        chars[9] = 'x';
        let line: String = chars.into_iter().collect();
        assert_eq!(
            parse_line(&line),
            Err(ConvertError::InvalidChar {
                row: 1,
                col: 0,
                ch: 'x'
            })
        );
    }

    #[test]
    fn block_format_rejects_missing_row() {
        let block = format_block(&sparse_board());
        let short: String = block
            .lines()
            .filter(|line| !line.trim().is_empty())
            .take(8)
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(
            parse_block(&short),
            Err(ConvertError::WrongRowCount {
                expected: 9,
                found: 8
            })
        );
    }

    #[test]
    fn block_format_rejects_row_with_wrong_cell_count() {
        let block = format_block(&sparse_board());
        let mut lines: Vec<String> = block.lines().map(|line| line.to_string()).collect();
        lines[0].pop(); // drop the last cell of the first row
        let joined = lines.join("\n");
        assert_eq!(
            parse_block(&joined),
            Err(ConvertError::WrongLength {
                expected: 9,
                found: 8
            })
        );
    }

    #[test]
    fn block_format_rejects_invalid_character() {
        let block = format_block(&sparse_board()).replacen('.', "x", 1);
        assert!(matches!(
            parse_block(&block),
            Err(ConvertError::InvalidChar { .. })
        ));
    }
}
