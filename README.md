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
The direction (`to-line` or `to-block`) is optional; if you leave it off, the
CLI looks at the shape of the input (one non-blank line vs. nine) and
converts to the other format.

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

$ cargo run --quiet -- < puzzle.txt
53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..97
```

And the other direction:

```
$ echo "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..97" \
    | cargo run --quiet --
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

Pass `to-line` or `to-block` explicitly to pick the direction yourself; this
is also the only way to convert input whose shape is ambiguous (auto-detect
only recognizes exactly one non-blank line or exactly nine).

Malformed input (wrong length, wrong row count, a stray letter) produces an
error message on stderr and a non-zero exit code instead of a guess.

Once a board parses, it is checked for duplicate digits in any row, column,
or 3x3 box. Conversion still happens either way - the two formats don't
encode sudoku's rules, so this is a warning rather than a rejection - but
each duplicate is printed to stderr:

```
$ echo "55..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..97" \
    | cargo run --quiet --
warning: digit 5 repeats in row 0
warning: digit 5 repeats in box 0
55. .7. ...
6.. 195 ...
.98 ... .6.

8.. .6. ..3
4.. 8.3 ..1
7.. .2. ..6

.6. ... 28.
... 419 ..5
... .8. .97
```

## Status

Format conversion in both directions works and has a table-driven test suite
covering the awkward cases (CRLF line endings, `0` vs `.` for blanks, missing
or extra blank lines, malformed rows). The CLI can auto-detect which
direction to convert and warns about duplicate digits in a parsed board, but
it only reads from stdin.

## License

MIT, see LICENSE.
