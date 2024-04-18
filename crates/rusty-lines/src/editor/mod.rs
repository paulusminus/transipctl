use std::path::Path;

use rustyline::{
    highlight::Highlighter, hint::HistoryHinter, history::FileHistory, Completer, CompletionType,
    Config, EditMode, Editor, Helper, Hinter, Validator,
};

use crate::{ReadlineError, Result};
use prompt::prompt;

mod prompt;

#[derive(Default, Completer, Helper, Hinter, Validator)]
pub struct MyHelper {
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for MyHelper {}

pub struct LineEditor<P: AsRef<Path>> {
    editor: Editor<MyHelper, FileHistory>,
    prompt: String,
    exit_terms: &'static [&'static str],
    history_filename: Option<P>,
}

impl<P: AsRef<Path>> Iterator for LineEditor<P> {
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

impl<P: AsRef<Path>> LineEditor<P> {
    pub fn try_new(
        name: &str,
        exit_terms: &'static [&'static str],
        history_filename: Option<P>,
    ) -> Result<Self> {
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
                history_filename,
            })
        })
    }
}

impl<P: AsRef<Path>> Drop for LineEditor<P> {
    fn drop(&mut self) {
        if let Some(filename) = self.history_filename.as_ref() {
            if let Err(error) = self.editor.save_history(&filename) {
                tracing::error!("Error saving {:?}: {}", filename.as_ref(), error);
            }
        }
    }
}
