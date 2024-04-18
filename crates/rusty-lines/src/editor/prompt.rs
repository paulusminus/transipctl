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
