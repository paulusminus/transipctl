use error::Error;
use pest_derive::Parser;
use pest::{Parser, iterators::Pair};
use serde::Serialize;
use std::{io::{BufReader, BufRead}, fmt::Debug};
use transip::{configuration_from_environment, Client};

pub type Result<T> = std::result::Result<T, error::Error>;

mod error;
mod vps_command;

pub trait Execute {
    type ApiResult;
    fn execute(&self, client: &mut Client) -> Result<Self::ApiResult>;
}

#[derive(Parser)]
#[grammar = "transip.pest"]
struct TransipCommandParser;

#[derive(Debug)]
struct UnknownCommand;

fn to_json<T: Serialize>(t: T) -> Result<String> {
    serde_json::to_string_pretty(&t).map_err(Error::from)
}

fn unit_to_string(_: ()) -> String {
    String::new()
}

fn execute_vps_command(pair: Pair<'_, Rule>, client: &mut Client) -> Result<String> {
    let commandline = pair.as_str().to_owned();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::vps_list => {
            vps_command::VpsListCommand.execute(client)
            .and_then(to_json)
        }
        Rule::vps_item_action => {
            let mut inner = inner.into_inner();
            let action = inner.next().unwrap().as_str().trim();
            let name = inner.next().unwrap().as_str().trim().to_owned();
            if action == "item" {
                vps_command::VpsItemCommand(name).execute(client)
                .and_then(to_json)
            }
            else if action == "reset" {
                vps_command::VpsResetCommand(name).execute(client)
                .map(unit_to_string)
            }
            else if action == "start" {
                vps_command::VpsStartCommand(name).execute(client)
                .map(unit_to_string)
            }
            else if action == "stop" {
                vps_command::VpsStopCommand(name).execute(client)
                .map(unit_to_string)
            }
            else if action == "lock" {
                vps_command::VpsLockCommand(name).execute(client)
                .map(unit_to_string)
            }
            else if action == "unlock" {
                vps_command::VpsUnlockCommand(name).execute(client)
                .map(unit_to_string)
            }
            else {
                Err(Error::ParseVpsCommand(commandline))
            }
        }
        _ => Err(Error::ParseVpsCommand(commandline))

    }
}

fn main() -> Result<()> {
    let mut lines = BufReader::new(std::io::stdin()).lines();
    let mut client = configuration_from_environment().and_then(Client::try_from)?;
    while let Some(line_result) = lines.next() {
        if let Ok(line) = line_result {
            if let Ok(pairs) = TransipCommandParser::parse(Rule::transip, line.as_str()) {
                for pair in pairs {
                    let inner = pair.into_inner().next().unwrap();
                    match inner.as_rule() {
                        Rule::comment => {
                            println!("comment: {}", inner.as_str());
                        }
                        Rule::dns_command => {
                            println!("dns: {}", inner.as_str());
                        }
                        Rule::vps_command => {
                            let s = execute_vps_command(inner, &mut client)?;
                            println!("{}", s);
                        }
                        Rule::invoice_command => {
                            println!("invoice: {}", inner.as_str());
                        }
                        _ => {
                            println!("Does not match");
                        }
                    }
                }
            }
            else {
                println!("Cannot parse");
            }
        }
        
    }
    Ok(())
}
