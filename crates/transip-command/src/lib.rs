#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub use dns::DnsCommand;
pub use domain::DomainCommand;
use error::ErrorExt;
pub use invoice::{InvoiceAction, InvoiceCommand};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
pub use product::ProductCommand;
use std::str::FromStr;
pub use vps::{VpsAction, VpsCommand};

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

mod dns;
pub mod domain;
mod error;
mod invoice;
mod product;
mod vps;

#[derive(Parser)]
#[grammar = "transip.pest"]
struct TransipCommandParser;

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

#[cfg(test)]
mod test {
    use crate::{
        dns::DnsCommand,
        domain::DomainCommand,
        invoice::{InvoiceAction, InvoiceCommand},
        product::ProductCommand,
        vps::{VpsAction, VpsCommand},
        TransipCommand,
    };

    #[test]
    fn comment() {
        let commandline = "# lkasjkfiekf";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Comment(commandline.to_owned()),
        );
    }

    #[test]
    fn domain_command_list() {
        let commandline = "domain list";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Domain(DomainCommand::List),
        )
    }

    #[test]
    fn domain_command_item() {
        let commandline = "domain item lkdkf";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Domain(DomainCommand::Item("lkdkf".to_owned()))
        );
    }

    #[test]
    fn dns_command_list() {
        let commandline = "dns list lkadjf ";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Dns(DnsCommand::List("lkadjf".to_owned())),
        );
    }

    #[test]
    fn dns_acme_challenge_delete() {
        let commandline = "dns acme-challenge-delete lkdfjf";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Dns(DnsCommand::DeleteAcmeChallenge("lkdfjf".to_owned())),
        );
    }

    #[test]
    fn dns_acme_challenge_set() {
        let commandline = "dns acme-challenge-set paulmin.nl (83jgljfg";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Dns(DnsCommand::SetAcmeChallenge(
                "paulmin.nl".to_owned(),
                "(83jgljfg".to_owned()
            ))
        );
    }

    #[test]
    fn vps_command_list() {
        let commandline = "vps list";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Vps(VpsCommand::List,)
        );
    }

    #[test]
    fn vps_command_item() {
        let commandline = "vps item iuerit";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Vps(VpsCommand::Action("iuerit".to_owned(), VpsAction::Item))
        );
    }

    #[test]
    fn invoice_command_list() {
        let commandline = "invoice list";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Invoice(InvoiceCommand::List,)
        );
    }

    #[test]
    fn invoice_command_item() {
        let commandline = "invoice item 38374";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Invoice(InvoiceCommand::Action(
                "38374".to_owned(),
                InvoiceAction::Item
            ))
        );
    }

    #[test]
    fn product_command_list() {
        let commandline = "product list";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Product(ProductCommand::List),
        );
    }

    #[test]
    fn product_elements() {
        let commandline = "product elements 37465";
        assert_eq!(
            commandline.parse::<TransipCommand>().unwrap(),
            TransipCommand::Product(ProductCommand::Elements("37465".to_owned())),
        );
    }
}
