use itertools::Itertools;
use rusty_lines::{ReadlineError, TTYLinesBuilder};

const PROMPT: &str = "tip";
const EXIT_ON: &[&str] = &["exit", "quit"];

fn process<I>(f: impl FnMut(String) + Copy) -> impl Fn(I) -> Result<(), ReadlineError>
where
    I: Iterator<Item = Result<String, ReadlineError>>,
{
    move |lines| {
        lines
            .filter_ok(|line| !line.trim().is_empty())
            .map_ok(f)
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }
}

fn print(s: String) {
    println!("{}", s)
}

fn main() -> Result<(), ReadlineError> {
    TTYLinesBuilder::<&str>::prompt(PROMPT)
        .exit_on(EXIT_ON)
        .build().and_then(process(print))
}
