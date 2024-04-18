use std::path::PathBuf;

use editor::LineEditor;
use file::FileReader;
pub use rustyline::error::ReadlineError;

mod editor;
mod file;

pub type Result<T, E = ReadlineError> = std::result::Result<T, E>;

enum Input {
    File(FileReader),
    Tty(LineEditor),
}

impl Iterator for Input {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Input::File(file) => file.next(),
            Input::Tty(tty) => tty.next(),
        }
    }
}

pub fn lines(
    prompt_name: &str,
    exit_terms: Vec<&'static str>,
    filename: Option<&String>,
    history_filename: Option<&PathBuf>,
) -> Result<(bool, impl Iterator<Item = Result<String>>), ReadlineError> {
    match filename {
        Some(file_name) => FileReader::try_new(file_name)
            .map(Input::File)
            .map(|r| (false, r)),
        None => LineEditor::try_new(prompt_name, exit_terms, history_filename)
            .map(Input::Tty)
            .map(|r| (true, r)),
    }
}
