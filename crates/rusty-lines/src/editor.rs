use std::path::{Path, PathBuf};

use rustyline::{
    highlight::Highlighter, hint::HistoryHinter, history::FileHistory, Completer, CompletionType,
    Config, EditMode, Editor, Helper, Hinter, Validator,
};

use crate::{Error, ReadlineError, Result};
use prompt::prompt;

#[derive(Default, Completer, Helper, Hinter, Validator)]
pub struct MyHelper {
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for MyHelper {}

pub struct LineEditor {
    editor: Editor<MyHelper, FileHistory>,
    prompt: String,
    exit_terms: &'static [&'static str],
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
            e @ Err(_) => Some(e.map_err(Error)),
        }
    }
}

impl LineEditor {
    pub fn try_new<P: AsRef<Path>>(
        name: &str,
        exit_terms: &'static [&'static str],
        history_filename: Option<P>,
    ) -> Result<Self, ReadlineError> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();
        rustyline::Editor::<MyHelper, FileHistory>::with_config(config).and_then(|mut editor| {
            editor.set_helper(Some(MyHelper::default()));
            if let Some(filename) = history_filename.as_ref() {
                if filename.as_ref().exists() {
                    editor.load_history(filename.as_ref())?;
                }
            };

            Ok(Self {
                exit_terms,
                prompt: prompt(&mut editor, name),
                editor,
                history_filename: history_filename.map(|f| PathBuf::from(f.as_ref())),
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

mod prompt {
    use std::io::{stdout, IsTerminal};

    use rustyline::{config::Configurer, history::FileHistory, ColorMode, Editor};

    use super::MyHelper;

    const ANSI_PREFIX: &str = "\x1b[1;32m";
    const ANSI_POSTFIX: &str = "\x1b[0m";

    pub fn prompt(editor: &mut Editor<MyHelper, FileHistory>, name: &str) -> String {
        if stdout().is_terminal() && editor.config_mut().color_mode() != ColorMode::Disabled {
            format!("{}{}: {}", ANSI_PREFIX, name, ANSI_POSTFIX)
        } else {
            format!("{}: ", name)
        }
    }
}
