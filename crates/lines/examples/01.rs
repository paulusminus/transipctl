use std::env::args;

use itertools::Itertools;
use lines::{lines, Error, Result};

fn process<I>(f: impl FnMut(String) + Copy) -> impl Fn((bool, I)) -> Result<(), Error>
where
    I: Iterator<Item = Result<String, Error>>,
{
    move |(_interactive, lines)| {
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

fn main() -> Result<()> {
    lines("tip", vec!["exit", "quit"], args().nth(1).as_ref()).and_then(process(print))
}
