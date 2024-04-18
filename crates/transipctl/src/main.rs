use rusty_lines::{FileLinesBuilder, TTYLinesBuilder};
use std::{env::args, path::PathBuf, process::exit};
use transip_execute::{
    configuration_from_environment, Client, ErrorKind, SubCommand, TransipCommand,
};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
const EXIT_ON: &[&str] = &["exit", "quit"];
const PROMPT: &str = "tipctl";

mod error;
mod log;

fn arg_version() {
    if std::env::args()
        .skip(1)
        .any(|s| ["--version", "-v"].contains(&s.as_str()))
    {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        exit(0);
    }
}

#[allow(dead_code)]
enum Out {
    Json,
    Yaml,
}

fn handle_error(msg: String, exit_on_error: bool) {
    eprintln!("{}", msg);
    if exit_on_error {
        exit(1);
    }
}

fn handle_ok(buffer: Vec<u8>, extra_newline: bool) {
    let s = String::from_utf8(buffer).unwrap();
    if !s.is_empty() {
        if extra_newline {
            println!("{}", s);
        } else {
            print!("{}", s);
        }
    }
}

macro_rules! execute_out {
    ($ser:path, $client:ident, $command:ident, $extra_newline:expr) => {
        let mut buffer: Vec<u8> = Vec::new();
        let mut ser = $ser(&mut buffer);

        match $client.execute($command, &mut ser) {
            Ok(_) => {
                handle_ok(buffer, $extra_newline);
            }
            Err(error) => {
                handle_error(format!("Error: {error}"), $client.exit_on_error());
            }
        }
    };
}

impl Out {
    fn execute(&self, client: &mut Client, command: &SubCommand) {
        match self {
            Out::Json => {
                execute_out!(serde_json::Serializer::pretty, client, command, true);
            }
            Out::Yaml => {
                execute_out!(serde_yaml::Serializer::new, client, command, false);
            }
        }
    }
}

fn main() -> Result<()> {
    arg_version();
    log::setup_logging();

    let lines = args()
        .nth(1)
        .map(PathBuf::from)
        .map(|f| FileLinesBuilder::file(f).replace_variables().build())
        .unwrap_or(
            TTYLinesBuilder::prompt(PROMPT)
                .exit_on(EXIT_ON)
                .history(log::log_dir().join("history.txt"))
                .build(),
        )?;

    tracing::info!(
        "Running {} {}",
        VERSION,
        args().nth(1).as_ref().unwrap_or(&"tty".to_owned())
    );

    let output_format = Out::Json;
    let mut client = configuration_from_environment().and_then(Client::try_from)?;

    for (line_number, line_result) in lines.enumerate() {
        let line = line_result?;
        if !line.trim().is_empty() {
            match line.parse::<TransipCommand>() {
                Ok(command) => output_format.execute(&mut client, &command.command),
                Err(error) => {
                    if error.kind() == ErrorKind::DisplayHelp {
                        handle_error(error.to_string(), client.exit_on_error())
                    } else {
                        handle_error(
                            format!("Error {} parsing line {}", error, line_number + 1),
                            client.exit_on_error(),
                        )
                    }
                }
            }
        }
    }

    tracing::info!(
        "Ending {} {}",
        VERSION,
        args().nth(1).as_ref().unwrap_or(&"tty".to_owned())
    );
    Ok(())
}
