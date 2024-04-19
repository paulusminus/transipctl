#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::path::Path;

use editor::LineEditor;
use file::FileReader;
pub use rustyline::error::ReadlineError;

mod editor;
mod file;

type Result<T, E = ReadlineError> = std::result::Result<T, E>;

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
        let reader = LineEditor::try_new(&self.prompt_name, self.exit_terms, self.history_filename)?;
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
    pub fn build(self) -> Result<Box< dyn Iterator<Item = Result<String, ReadlineError>>>> {
        let reader = FileReader::try_new(self.filename, self.replace_variables)?;
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
