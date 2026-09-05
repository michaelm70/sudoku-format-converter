# sudoku-format-converter

Sudoku puzzles get passed around in two incompatible plain-text shapes.
Puzzle databases and solvers want a single line of 81 characters. People
writing a puzzle by hand, or pasting one into a forum post, write a 9x9 block
grid instead. Converting between the two by hand is easy to get subtly wrong
(miscount a row, drop a blank cell) and there isn't much reason to write that
code twice, so this is a small command-line tool that does the conversion in
both directions.

## Formats

**line**: 81 characters, no whitespace. `1`-`9` for a given digit, `.` or `0`
for a blank cell.

```
53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..97
```

**block**: 9 rows, each row written as three groups of three digits separated
by a space, with a blank line after the 3rd and 6th row. `.` or `0` mark a
blank cell. Extra blank lines and `\r\n` line endings are tolerated on input.

```
53. .7. ...
6.. 195 ...
.98 ... .6.

8.. .6. ..3
4.. 8.3 ..1
7.. .2. ..6

.6. ... 28.
... 419 ..5
... .8. .97
```

Neither format says anything about whether the puzzle is valid; the converter
only reshapes cells, it doesn't check sudoku rules.

## Usage

The binary reads a puzzle from stdin and writes the converted form to stdout.

```
$ cat puzzle.txt
53. .7. ...
6.. 195 ...
.98 ... .6.

8.. .6. ..3
4.. 8.3 ..1
7.. .2. ..6

.6. ... 28.
... 419 ..5
... .8. .97

$ cargo run --quiet -- to-line < puzzle.txt
53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..97
```

And the other direction:

```
$ echo "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..97" \
    | cargo run --quiet -- to-block
53. .7. ...
6.. 195 ...
.98 ... .6.

8.. .6. ..3
4.. 8.3 ..1
7.. .2. ..6

.6. ... 28.
... 419 ..5
... .8. .97
```

Malformed input (wrong length, wrong row count, a stray letter) produces an
error message on stderr and a non-zero exit code instead of a guess.

## Status

Format conversion in both directions works and has a table-driven test suite
covering the awkward cases (CRLF line endings, `0` vs `.` for blanks, missing
or extra blank lines, malformed rows). It does not yet validate that a parsed
board is a legal sudoku, and the CLI only reads from stdin.

## License

MIT, see LICENSE.
