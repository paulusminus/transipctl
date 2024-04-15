use std::path::PathBuf;

use rustyline::{
    highlight::Highlighter, hint::HistoryHinter, history::FileHistory, Cmd, Completer, CompletionType, Config, EditMode, Editor, Helper, Hinter, KeyEvent, Validator
};

use crate::{ReadlineError, Result};
use prompt::prompt;

mod prompt;

pub struct LineEditor {
    editor: Editor<MyHelper, FileHistory>,
    prompt: String,
    exit_terms: Vec<&'static str>,
    history_filename: Option<PathBuf>,
}

impl Iterator for LineEditor {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.editor.readline(&self.prompt) {
            Ok(line) => {
                let _ = self.editor.add_history_entry(line.as_str());
                if self.exit_terms.contains(&line.trim()) {
                    None
                } else {
                    Some(Ok(line))
                }
            }
            Err(ReadlineError::Eof) => None,
            e @ Err(_) => Some(e),
        }
    }
}

#[derive(Completer, Helper, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for MyHelper {}

impl Default for MyHelper {
    fn default() -> Self {
        Self {
            hinter: HistoryHinter::new(),
        }
    }
}

impl LineEditor {
    pub fn try_new(
        name: &str,
        exit_terms: Vec<&'static str>,
        history_filename: Option<&PathBuf>,
    ) -> Result<Self> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();
        rustyline::Editor::<MyHelper, FileHistory>::with_config(config).and_then(|mut editor| {
            editor.set_helper(Some(MyHelper::default()));
            editor.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
            editor.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
            if let Some(filename) = history_filename {
                if filename.exists() {
                    editor.load_history(filename)?;
                }
            };

            Ok(Self {
                exit_terms,
                prompt: prompt(&mut editor, name),
                editor,
                history_filename: history_filename.cloned(),
            })
        })
    }
}

impl Drop for LineEditor {
    fn drop(&mut self) {
        if let Some(filename) = self.history_filename.as_ref() {
            if let Err(error) = self.editor.save_history(&filename) {
                tracing::error!("Error saving {:?}: {}", &filename, error);
            }
        }
    }
}
