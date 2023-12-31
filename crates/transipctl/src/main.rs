use input::Input;
use std::process::exit;
use transip_execute::{configuration_from_environment, Client, TransipCommand};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
const EXIT_COMMANDS: [&str; 2] = ["exit", "quit"];

mod error;
mod input;
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

enum Out {
    #[allow(dead_code)]
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
    fn execute(&self, client: &mut Client, command: &TransipCommand) {
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
    let input: Input = std::env::args().try_into()?;

    let (interactive, run_from) = input.run_from();
    tracing::info!("Running {} {}", VERSION, run_from);

    let output_format = Out::Yaml;
    let mut client = configuration_from_environment().and_then(Client::try_from)?;

    for (line_number, line) in input.lines().enumerate() {
        if EXIT_COMMANDS.contains(&line.trim().to_ascii_lowercase().as_str()) && interactive {
            break;
        }
        if !line.trim().is_empty() {
            match line.parse::<TransipCommand>() {
                Ok(command) => output_format.execute(&mut client, &command),
                Err(error) => handle_error(
                    format!("Error {} parsing line {}", error, line_number + 1),
                    client.exit_on_error(),
                ),
            }
        }
    }

    tracing::info!("Ending {} {}", VERSION, run_from);
    Ok(())
}
