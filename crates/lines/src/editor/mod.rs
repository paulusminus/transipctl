use rustyline::{history::FileHistory, Editor};

use crate::{Error, Result};
use prompt::prompt;

mod prompt;

pub struct LineEditor {
    editor: Editor<(), FileHistory>,
    prompt: String,
    exit_terms: Vec<&'static str>,
}

impl Iterator for LineEditor {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.editor.readline(&self.prompt) {
            Ok(line) => {
                if self.exit_terms.contains(&line.trim()) {
                    None
                } else {
                    Some(Ok(line))
                }
            }
            Err(Error::Eof) => None,
            e @ Err(_) => Some(e),
        }
    }
}

impl LineEditor {
    pub fn try_new(name: &str, exit_terms: Vec<&'static str>) -> Result<Self> {
        rustyline::DefaultEditor::new().map(|mut editor| Self {
            exit_terms,
            prompt: prompt(&mut editor, name),
            editor,
        })
    }
}
