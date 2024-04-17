use std::str::FromStr;

use clap::{Error, Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    TXT,
    SRV,
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum OnError {
    Print,
    Exit,
}

#[derive(Clone, Debug, Parser)]
pub struct DnsEntry {
    pub domain: String,
    pub name: String,
    pub ttl: u32,
    pub r#type: RecordType,
    pub content: String,
}

#[cfg(feature = "propagation")]
#[derive(Debug, Subcommand)]
pub enum DnsCommand {
    AcmeValidationDelete { domain: String },
    AcmeValidationSet { domain: String, challenge: String },
    AcmeValidationCheck { domain: String, challenge: String },
    Delete(DnsEntry),
    Insert(DnsEntry),
    List { domain: String },
}

#[cfg(not(feature = "propagation"))]
#[derive(Debug, Subcommand)]
pub enum DnsCommand {
    AcmeValidationDelete { domain: String },
    AcmeValidationSet { domain: String, challenge: String },
    Delete(DnsEntry),
    Insert(DnsEntry),
    List,
}

#[derive(Debug, Subcommand)]
pub enum DomainCommand {
    List,
    Item { domain: String },
}

#[derive(Debug, Subcommand)]
pub enum InvoiceCommand {
    List,
    Item { number: String },
    Pdf { number: String },
}

#[derive(Debug, Subcommand)]
pub enum ProductCommand {
    List,
    Elements { name: String },
}

#[derive(Debug, Subcommand)]
pub enum EmailBoxCommand {
    List {
        domain: String,
    },
    Item {
        domain: String,
        id: String,
    },
    Delete {
        domain: String,
        id: String,
    },
    Insert {
        domain: String,
        username: String,
        password: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum EmailForwardCommand {
    List {
        domain: String,
    },
    Item {
        domain: String,
        id: String,
    },
    Delete {
        domain: String,
        id: String,
    },
    Insert {
        domain: String,
        local_part: String,
        forward_to: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum VpsCommand {
    List,
    Item { name: String },
    Start { name: String },
    Stop { name: String },
    Reset { name: String },
    Lock { name: String },
    Unlock { name: String },
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    AvailibilityZones,
    Comment {
        text: String,
    },
    #[command(subcommand)]
    Dns(DnsCommand),
    #[command(subcommand)]
    Domain(DomainCommand),
    #[command(subcommand)]
    EmailBox(EmailBoxCommand),
    #[command(subcommand)]
    EmailForward(EmailForwardCommand),
    #[command(subcommand)]
    Invoice(InvoiceCommand),
    Onerror {
        on_error: OnError,
    },
    Ping,
    #[command(subcommand)]
    Product(ProductCommand),
    Sleep {
        number_of_seconds: u64,
    },
    #[command(subcommand)]
    Vps(VpsCommand),
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
pub struct TransipCommand {
    #[command(subcommand)]
    pub command: SubCommand,
}

fn command_line<S: AsRef<str>>(line: S) -> Result<Vec<String>, clap::Error> {
    if line.as_ref().trim_start().starts_with("#") {
        Ok(vec!["comment".to_owned(), line.as_ref().to_owned()])
    } else {
        shlex::split(line.as_ref()).ok_or(clap::Error::new(clap::error::ErrorKind::Format))
    }
}

impl FromStr for TransipCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        command_line(s).and_then(TransipCommand::try_parse_from)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::{command_line, TransipCommand};
    use clap::Parser;

    const COMMANDS: &[u8] = include_bytes!("commands.txt");

    #[test]
    fn try_command_lines() {
        let lines = BufReader::new(COMMANDS).lines();
        for args_option in lines.map_while(Result::ok).map(command_line) {
            match args_option {
                Ok(args) => {
                    let result = TransipCommand::try_parse_from(args).unwrap();
                    println!("{:?}", &result.command);
                }
                Err(error) => eprintln!("Error parsing line: {error}"),
            }
        }
    }
}
