use error::ErrorExt;
use input::Input;
use pest::Parser;
use pest_derive::Parser;
use serde::Serialize;
use transip::{configuration_from_environment, Client};

pub type Result<T> = std::result::Result<T, error::Error>;

pub const VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

pub trait Report {
    fn report(self);
}

impl Report for Result<String> {
    fn report(self) {
        match self {
            Ok(s) => {
                if !s.is_empty() {
                    println!("{}", s);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }
}

mod error;
mod input;
pub mod command;

#[derive(Parser)]
#[grammar = "transip.pest"]
struct TransipCommandParser;

fn to_json<T: Serialize>(t: T) -> Result<String> {
    serde_json::to_string_pretty(&t).err_into()
}

fn unit_to_string(_: ()) -> String {
    String::new()
}

fn main() -> Result<()> {
    let input: Input = std::env::args().try_into()?;
    let mut client = configuration_from_environment().and_then(Client::try_from)?;
    for (_line_number, line) in input.lines().flatten().enumerate() {
        if line.trim().is_empty() {
            break;
        }
        if let Ok(pairs) = TransipCommandParser::parse(Rule::transip, line.as_str()) {
            for pair in pairs {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::comment => {
                        // println!("comment: {}", inner.as_str());
                    }
                    Rule::dns_command => {
                        println!("dns: {}", inner.as_str());
                    }
                    Rule::vps_command => {
                        command::vps::execute(inner, &mut client).report();
                    }
                    Rule::invoice_command => {
                        command::invoice::execute(inner, &mut client).report()
                    }
                    Rule::product_command => {
                        command::product::execute(inner, &mut client).report();
                    }
                    _ => {
                        println!("Does not match");
                    }
                }
            }
        } else {
            println!("Cannot parse");
        }
    }
    Ok(())
}
