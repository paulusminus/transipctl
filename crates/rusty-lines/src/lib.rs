#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::path::Path;

use editor::LineEditor;
use file::FileReader;
pub use rustyline::error::ReadlineError;

mod editor;
mod file;

type Result<T, E = ReadlineError> = std::result::Result<T, E>;

pub enum Input<P: AsRef<Path>> {
    #[doc(hidden)]
    File(FileReader),
    #[doc(hidden)]
    TTY(LineEditor<P>),
}

impl<P: AsRef<Path>> Iterator for Input<P> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Input::File(file) => file.next(),
            Input::TTY(tty) => tty.next(),
        }
    }
}

pub struct TTYLinesBuilder<P: AsRef<Path>> {
    exit_terms: &'static [&'static str],
    history_filename: Option<P>,
    prompt_name: String,
}

impl<P: AsRef<Path>> TTYLinesBuilder<P> {
    pub fn prompt(prompt: &str) -> Self {
        Self {
            prompt_name: prompt.to_owned(),
            exit_terms: &[],
            history_filename: None,
        }
    }

    pub fn exit_on(self, exit_terms: &'static [&'static str]) -> Self {
        Self {
            prompt_name: self.prompt_name,
            exit_terms,
            history_filename: self.history_filename,
        }
    }

    pub fn history(self, filename: P) -> Self {
        Self {
            prompt_name: self.prompt_name,
            exit_terms: self.exit_terms,
            history_filename: Some(filename),
        }
    }

    pub fn build(self) -> Result<Input<P>> {
        LineEditor::try_new(&self.prompt_name, self.exit_terms, self.history_filename)
            .map(Input::TTY)
    }
}

pub struct FileLinesBuilder<P: AsRef<Path>> {
    filename: P,
    #[allow(dead_code)]
    replace_variables: bool,
}

impl<P: AsRef<Path>> FileLinesBuilder<P> {
    pub fn file(path: P) -> Self {
        Self {
            filename: path,
            replace_variables: false,
        }
    }

    pub fn replace_variables(self) -> Self {
        Self {
            filename: self.filename,
            replace_variables: true,
        }
    }

    pub fn build(self) -> Result<Input<P>> {
        FileReader::try_new(self.filename, self.replace_variables).map(Input::File)
    }
}

#[cfg(test)]
mod test {
    use crate::{FileLinesBuilder, TTYLinesBuilder};

    #[test]
    fn options() {
        let mut input = FileLinesBuilder::file("Cargo.toml").build().unwrap();
        assert_eq!(input.next().unwrap().unwrap(), "[package]".to_owned());
    }

    #[test]
    fn file_options_builder() {
        let mut input = Some("Cargo.toml")
            .map(|f| FileLinesBuilder::file(f).replace_variables().build())
            .unwrap_or(
                TTYLinesBuilder::prompt("tipctl")
                    .exit_on(&["exit", "quit"])
                    .history("history.txt")
                    .build(),
            )
            .unwrap();
        assert_eq!(input.next().unwrap().unwrap(), "[package]".to_owned());
    }
}
