#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub use dns::DnsCommand;
pub use domain::DomainCommand;
use error::ErrorExt;
pub use invoice::{InvoiceAction, InvoiceCommand};
use parse::{Rule, TransipCommandParser};
use pest::{iterators::Pair, Parser};
pub use product::ProductCommand;
use std::{str::FromStr, fmt::Display};
pub use vps::{VpsAction, VpsCommand};

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

mod dns;
mod domain;
mod error;
mod invoice;
mod parse;
mod product;
mod vps;

#[derive(Debug, PartialEq)]
pub enum TransipCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::TransipCommand;
    ///
    /// let commandline = "# lkasjkfiekf";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Comment(commandline.to_owned()),
    /// );
    /// ```
    Comment(String),

    Domain(domain::DomainCommand),

    Dns(dns::DnsCommand),

    Invoice(invoice::InvoiceCommand),

    Product(product::ProductCommand),

    Vps(vps::VpsCommand),
}

impl FromStr for TransipCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut pairs = TransipCommandParser::parse(Rule::transip, s).map_err(Box::new)?;
        let pair = pairs
            .nth(0)
            .ok_or(Error::ParseTransipCommand(s.to_owned()))?;
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::comment => Ok(TransipCommand::Comment(inner.as_str().to_owned())),
            Rule::domain_command => DomainCommand::try_from(inner).map(TransipCommand::Domain),
            Rule::dns_command => DnsCommand::try_from(inner).map(TransipCommand::Dns),
            Rule::vps_command => VpsCommand::try_from(inner).map(TransipCommand::Vps),
            Rule::invoice_command => InvoiceCommand::try_from(inner).map(TransipCommand::Invoice),
            Rule::product_command => ProductCommand::try_from(inner).map(TransipCommand::Product),
            _ => Err(Error::ParseTransipCommand(s.to_owned())),
        }
    }
}

impl Display for TransipCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransipCommand::Comment(comment) => write!(f, "{}", comment),
            TransipCommand::Dns(command) => write!(f, "dns {}", command),
            TransipCommand::Domain(command) => write!(f, "domain {}", command),
            TransipCommand::Invoice(command) => write!(f, "invoice {}", command),
            TransipCommand::Product(command) => write!(f, "product {}", command),
            TransipCommand::Vps(command) => write!(f, "vps {}", command),
        }
    }
}

fn parameter(pair: Pair<'_, Rule>) -> Result<String> {
    match pair.as_rule() {
        Rule::env => {
            let name = pair
                .as_str()
                .strip_prefix("${")
                .unwrap()
                .strip_suffix('}')
                .unwrap();

            std::env::var(name).err_into()
        }
        Rule::value => Ok(pair.as_str().to_owned()),
        _ => Err(Error::ParseVpsCommand(pair.as_str().to_owned())),
    }
}
