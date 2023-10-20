use error::Error;
use pest_derive::Parser;
use pest::{Parser, iterators::Pair};
use serde::Serialize;
use std::{io::{BufReader, BufRead, Read}, fmt::Debug, fs::OpenOptions, process::exit};
use transip::{configuration_from_environment, Client, api::vps::TransipApiVps};

pub type Result<T> = std::result::Result<T, error::Error>;

mod error;

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
            client.vps_list()
            .map_err(Error::from)
            .and_then(to_json)
        }
        Rule::vps_item_action => {
            let mut inner = inner.into_inner();
            let action = inner.next().unwrap().as_str().trim();
            let name = inner.next().unwrap().as_str().trim();
            if action == "item" {
                client.vps(name)
                .map_err(Error::from)
                .and_then(to_json)
            }
            else if action == "reset" {
                client.vps_reset(&name)
                .map_err(Error::from)
                .map(unit_to_string)
            }
            else if action == "start" {
                client.vps_start(name)
                .map_err(Error::from)
                .map(unit_to_string)
            }
            else if action == "stop" {
                client.vps_stop(name)
                .map_err(Error::from)
                .map(unit_to_string)
            }
            else if action == "lock" {
                client.vps_set_is_locked(name, true)
                .map_err(Error::from)
                .map(unit_to_string)
            }
            else if action == "unlock" {
                client.vps_set_is_locked(name, false)
                .map_err(Error::from)
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
    let input: Option<Box<dyn Read>> =
        if let Some(file_name) = std::env::args().nth(1) {
            if let Ok(file) = OpenOptions::new().read(true).open(file_name) {
                Some(Box::new(file))
            }
            else {
                None
            }
        }
        else {
            Some(Box::new(std::io::stdin()))
        };

    if input.is_none() {
        exit(1);
    }

    let mut lines = BufReader::new(input.unwrap()).lines();
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
