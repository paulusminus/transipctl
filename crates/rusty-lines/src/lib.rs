#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{
    fmt::{Debug, Display},
    fs::File,
    path::Path,
};

use editor::LineEditor;
use file::FileReader;
use rustyline::error::ReadlineError;

mod editor;
mod file;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Wrapper for rustlyline ReadlineError
pub struct Error(ReadlineError);

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

/// Builder for lines read from the tty or stdin
pub struct TTYLinesBuilder<P: AsRef<Path>> {
    exit_terms: &'static [&'static str],
    history_filename: Option<P>,
    prompt_name: String,
}

impl<P: AsRef<Path>> TTYLinesBuilder<P> {
    /// prompt to display is using the tty as input
    pub fn prompt(prompt: &str) -> Self {
        Self {
            prompt_name: prompt.to_owned(),
            exit_terms: &[],
            history_filename: None,
        }
    }

    /// set the words that stop the iteration
    pub fn exit_on(self, exit_terms: &'static [&'static str]) -> Self {
        Self {
            prompt_name: self.prompt_name,
            exit_terms,
            history_filename: self.history_filename,
        }
    }

    /// Set history on when using the tty. History is saved to a file with filenam
    pub fn history(self, filename: P) -> Self {
        Self {
            prompt_name: self.prompt_name,
            exit_terms: self.exit_terms,
            history_filename: Some(filename),
        }
    }

    /// Construct the line iterator
    pub fn build(self) -> Result<Box<dyn Iterator<Item = Result<String>>>> {
        let reader = LineEditor::try_new(&self.prompt_name, self.exit_terms, self.history_filename)
            .map_err(Error)?;
        Ok(Box::new(reader))
    }
}

/// Builder for lines read from a file
pub struct FileLinesBuilder<P: AsRef<Path>> {
    filename: P,
    replace_variables: bool,
}

impl<P: AsRef<Path>> FileLinesBuilder<P> {
    /// path sets the filename on the filesystem to read from
    pub fn file(path: P) -> Self {
        Self {
            filename: path,
            replace_variables: false,
        }
    }

    /// if used then environment variables will be captured and replaced by their value
    pub fn replace_variables(self) -> Self {
        Self {
            filename: self.filename,
            replace_variables: true,
        }
    }

    /// construct the line iterator
    pub fn build(self) -> Result<Box<dyn Iterator<Item = Result<String, Error>>>> {
        let reader = FileReader::<File>::try_new(self.filename, self.replace_variables)?;
        Ok(Box::new(reader))
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
