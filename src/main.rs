use pest_derive::Parser;
use pest::{Parser, iterators::Pair};
use std::{io::{BufReader, BufRead}, fmt::Debug};

mod error;

#[derive(Parser)]
#[grammar = "transip.pest"]
struct TransipCommandParser;

#[derive(Debug)]
struct VpsListCommand;

#[derive(Debug)]
struct VpsStartCommand(String);

#[derive(Debug)]
struct VpsStopCommand(String);

#[derive(Debug)]
struct VpsResetCommand(String);

#[derive(Debug)]
struct VpsItemCommand(String);

#[derive(Debug)]
struct VpsLockCommand(String);

#[derive(Debug)]
struct VpsUnlockCommand(String);

#[derive(Debug)]
struct UnknownCommand;

fn parse_vps_command(pair: Pair<'_, Rule>) -> Box<dyn Debug> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::vps_list => {
            Box::new(VpsListCommand)
        }
        Rule::vps_item_action => {
            let mut inner = inner.into_inner();
            let action = inner.next().unwrap().as_str().trim();
            let name = inner.next().unwrap().as_str().trim().to_owned();
            if action == "item" {
                Box::new(VpsItemCommand(name))
            }
            else if action == "reset" {
                Box::new(VpsResetCommand(name))
            }
            else if action == "start" {
                Box::new(VpsStartCommand(name))
            }
            else if action == "stop" {
                Box::new(VpsStopCommand(name))
            }
            else if action == "lock" {
                Box::new(VpsLockCommand(name))
            }
            else if action == "unlock" {
                Box::new(VpsUnlockCommand(name))
            }
            else {
                Box::new(UnknownCommand)
            }
        }
        _ => Box::new(UnknownCommand)
    }
}

fn main() {
    let mut lines = BufReader::new(std::io::stdin()).lines();
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
                            println!("{:#?}", parse_vps_command(inner));
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
}
