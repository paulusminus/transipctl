![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![docs.rs](https://img.shields.io/docsrs/rusty-lines)
[![Crates.io](https://img.shields.io/crates/v/rusty-lines)](https://crates.io/crates/rusty-lines)

# rusty-lines

Rusty-lines is a library that abstracts the way lines are read from a tty or from a file.
When using the file lines reader you can optionally replace environment variables names with their values.

It uses [rustyline](https://crates.io/crates/rustyline) for reading from tty.

## Example using tty

```no_run
use rusty_lines::TTYLinesBuilder;

let lines = TTYLinesBuilder::prompt("tip")
    .exit_on(&["exit"])
    .history("history.txt")
    .build()
    .unwrap();
```

## Example using file

```
use rusty_lines::FileLinesBuilder;

let lines = FileLinesBuilder::file("Cargo.toml")
    .build()
    .unwrap();
```
