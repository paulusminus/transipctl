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

#[derive(Clone, Debug, ValueEnum)]
pub enum OnError {
    Print,
    Exit,
}

#[derive(Debug, Parser)]
pub struct DnsEntry {
    domain_name: String,
    record_name: String,
    ttl: u64,
    record_type: RecordType,
    content: String,
}

#[derive(Debug, Subcommand)]
pub enum DnsCommand {
    AcmeValidationDelete {
        domain_name: String,
    },
    AcmeValidationSet {
        domain_name: String,
        challenge: String,
    },
    Delete(DnsEntry),
    Insert(DnsEntry),
    List,
}

#[derive(Debug, Subcommand)]
pub enum DomainCommand {
    List,
    Item { domain_name: String },
}

#[derive(Debug, Subcommand)]
pub enum InvoiceCommand {
    List,
    Item { invoice_number: String },
    Pdf { invoice_number: String },
}

#[derive(Debug, Subcommand)]
pub enum ProductCommand {
    List,
    Elements { product_name: String },
}

#[derive(Debug, Subcommand)]
pub enum EmailBoxCommand {
    List {
        domain_name: String,
    },
    Item {
        domain_name: String,
        id: String,
    },
    Delete {
        domain_name: String,
        id: String,
    },
    Insert {
        domain_name: String,
        username: String,
        password: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum EmailForwardCommand {
    List {
        domain_name: String,
    },
    Item {
        domain_name: String,
        id: String,
    },
    Delete {
        domain_name: String,
        id: String,
    },
    Insert {
        domain_name: String,
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
    Comment{
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
#[command(about, long_about, version)]
pub struct TransipCommand {
    #[command(subcommand)]
    command: SubCommand,
}

fn command_line<S: AsRef<str>>(line: S) -> Option<Vec<String>> {
    if line.as_ref().trim_start().starts_with("#") {
        None
    } else {
        let st = "transip ".to_owned() + line.as_ref();
        shlex::split(st.as_str())
    }
}

impl FromStr for TransipCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command_line(s) {
            Some(args) => {
                TransipCommand::try_parse_from(args)
            }
            None => Ok(TransipCommand {
                command: SubCommand::Comment { text: s.to_owned() }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::TransipCommand;
    use clap::Parser;

    const COMMANDS: &[u8] = include_bytes!("commands.txt");

    fn command_line<S: AsRef<str>>(line: S) -> Option<Vec<String>> {
        if line.as_ref().trim_start().starts_with("#") {
            None
        } else {
            let st = "transip ".to_owned() + line.as_ref();
            shlex::split(st.as_str())
        }
    }

    #[test]
    fn try_command_lines() {
        let lines = BufReader::new(COMMANDS).lines();
        for args_option in lines.map_while(Result::ok).map(command_line) {
            match args_option {
                Some(args) => {
                    let result = TransipCommand::try_parse_from(args).unwrap();
                    println!("{:?}", &result.command);
                    match result.command {
                        super::SubCommand::Sleep { number_of_seconds } => {
                            println!("Sleeping for {}", number_of_seconds);
                        }
                        _ => {}
                    }
                }
                None => println!("Comment received"),
            }
        }
    }
}
