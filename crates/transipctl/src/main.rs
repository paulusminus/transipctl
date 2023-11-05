use input::Input;
use std::process::exit;
use transip_command::TransipCommand;
use transip_execute::{configuration_from_environment, Client};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

mod error;
mod input;

fn arg_version() {
    if std::env::args()
        .enumerate()
        .any(|(i, s)| i > 0 && ["--version", "-v"].contains(&s.as_str()))
    {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        exit(0);
    }
}

fn main() -> Result<()> {
    arg_version();
    let input: Input = std::env::args().try_into()?;
    let mut client = configuration_from_environment().and_then(Client::try_from)?;
    for (line_number, line) in input.lines().enumerate() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut s = serde_yaml::Serializer::new(&mut buffer);
        if !line.trim().is_empty() {
            match line.parse::<TransipCommand>() {
                Ok(command) => match client.execute(&command, &mut s) {
                    Ok(_) => {
                        let s = String::from_utf8(buffer).unwrap();
                        if s.len() > 0 {
                            print!("{}", s);
                        }
                    }
                    Err(error) => {
                        eprintln!("Error executing command {:#?}: {}", command, error);
                    }
                },
                Err(error) => eprintln!("Error {} parsing line {}", error, line_number + 1),
            }
        }
    }
    Ok(())
}
