use command::TransipCommand;
use error::ErrorExt;
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
    for (line_number, line) in input.lines().enumerate() {
        if !line.trim().is_empty() {
            match line
                .parse::<TransipCommand>()
                .and_then(|command| command.execute(&mut client).err_into())
            {
                Ok(result) => {
                    if result.as_str() != "null" {
                        println!("{}", result)
                    }
                }
                Err(error) => eprintln!("Error {} executing line {}", error, line_number + 1),
            }
        }
    }
    Ok(())
}
