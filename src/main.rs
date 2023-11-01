use command::TransipCommand;
use execution::Execution;
use input::Input;
use pest_derive::Parser;
use transip::{configuration_from_environment, Client};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

mod command;
mod error;
mod execution;
mod input;

#[derive(Parser)]
#[grammar = "transip.pest"]
struct TransipCommandParser;

fn main() -> Result<()> {
    let input: Input = std::env::args().try_into()?;
    let mut client = configuration_from_environment().and_then(Client::try_from)?;
    for (_line_number, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            break;
        }
        match line.parse::<TransipCommand>() {
            Ok(command) => command.execute(&mut client),
            Err(error) => eprintln!("Error: {}", error)
        }
    }
    Ok(())
}
