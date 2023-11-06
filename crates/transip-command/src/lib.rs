#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub use dns::DnsCommand;
pub use domain::DomainCommand;
use error::ErrorExt;
pub use invoice::{InvoiceAction, InvoiceCommand};
pub use product::ProductCommand;
use std::{fmt::Display, str::FromStr, env::VarError};
pub use vps::{VpsAction, VpsCommand};

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

mod dns;
mod domain;
mod error;
mod invoice;
mod product;
mod str_extension;
mod vps;

const COMMENT: &str = "#";
const DNS_COMMAND: &str = "dns ";
const DOMAIN_COMMAND: &str = "domain ";
const INVOICE_COMMAND: &str = "invoice ";
const PRODUCT_COMMAND: &str = "product ";
const SLEEP_COMMAND: &str = "sleep ";
const VPS_COMMAND: &str = "vps ";

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

    /// # Example
    ///
    /// ```
    /// use transip_command::TransipCommand;
    ///
    /// let commandline = "sleep 5";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Sleep(5),
    /// );
    /// ```
    Sleep(u64),
}

impl FromStr for TransipCommand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        macro_rules! parse {
            ($s:expr, $command:expr, $subcommand:path, $map:path) => {
                if s.starts_with($command) {
                    return s[$command.len()..].parse::<$subcommand>().err_into().map($map);
                }
            };
        }

        if s.starts_with(COMMENT) {
            return Ok(TransipCommand::Comment(s.to_owned()));
        }
        parse!(s, DNS_COMMAND, DnsCommand, TransipCommand::Dns);
        parse!(s, DOMAIN_COMMAND, DomainCommand, TransipCommand::Domain);
        parse!(s, INVOICE_COMMAND, InvoiceCommand, TransipCommand::Invoice);
        parse!(s, PRODUCT_COMMAND, ProductCommand, TransipCommand::Product);
        parse!(s, SLEEP_COMMAND, u64, TransipCommand::Sleep);
        parse!(s, VPS_COMMAND, VpsCommand, TransipCommand::Vps);

        Err(Error::ParseTransipCommand(s.to_owned()))
    }
}

impl Display for TransipCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransipCommand::Comment(comment) => write!(f, "{}", comment),
            TransipCommand::Dns(command) => write!(f, "{}{}", DNS_COMMAND, command),
            TransipCommand::Domain(command) => write!(f, "{}{}", DOMAIN_COMMAND, command),
            TransipCommand::Invoice(command) => write!(f, "{}{}", INVOICE_COMMAND, command),
            TransipCommand::Product(command) => write!(f, "{}{}", PRODUCT_COMMAND, command),
            TransipCommand::Sleep(timeout) => write!(f, "{}{}", SLEEP_COMMAND, timeout),
            TransipCommand::Vps(command) => write!(f, "{}{}", VPS_COMMAND, command),
        }
    }
}

fn check_environment(name: &str) -> std::result::Result<String, VarError> {
    if name.starts_with("${") && name.ends_with("}") {
        let s = name[2..name.len() - 1].trim();
        std::env::var(s)
    }
    else {
        Ok(name.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::TransipCommand;

    #[test]
    fn display() {
        assert_eq!(
            TransipCommand::Comment("# lksadjf".to_owned()).to_string(),
            "# lksadjf".to_owned(),
        );

        assert_eq!(
            TransipCommand::Dns(crate::DnsCommand::List("paulmin.nl".to_owned())).to_string(),
            "dns list paulmin.nl".to_owned(),
        );

        assert_eq!(
            TransipCommand::Domain(crate::DomainCommand::List).to_string(),
            "domain list".to_owned(),
        );

        assert_eq!(
            TransipCommand::Invoice(crate::InvoiceCommand::List).to_string(),
            "invoice list".to_owned(),
        );

        assert_eq!(
            TransipCommand::Product(crate::ProductCommand::List).to_string(),
            "product list".to_owned(),
        );

        assert_eq!(
            TransipCommand::Vps(crate::VpsCommand::List).to_string(),
            "vps list".to_owned(),
        );
    }

    #[test]
    fn transip_command_from_str() {
        assert_eq!(
            "# alsjff".parse::<TransipCommand>().unwrap(),
            TransipCommand::Comment("# alsjff".to_owned()),
        ); 
        assert_eq!(
            "dns list paulmin.nl ".parse::<TransipCommand>().unwrap(),
            TransipCommand::Dns(crate::DnsCommand::List("paulmin.nl".to_owned()))
        );
        assert_eq!(
            "vps reset paulusminus-vps2".parse::<TransipCommand>().unwrap(),
            TransipCommand::Vps(crate::VpsCommand::Action("paulusminus-vps2".to_owned(), crate::VpsAction::Reset))
        );
        assert_eq!(
            "sleep 3984".parse::<TransipCommand>().unwrap(),
            TransipCommand::Sleep(3984),
        );
    }
}
