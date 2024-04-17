#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub use dns::DnsCommand;
pub use domain::DomainCommand;
use error::ErrorExt;
pub use invoice::{InvoiceAction, InvoiceCommand};
pub use onerror::OnError;
pub use product::ProductCommand;
use std::{env::VarError, fmt::Display, str::FromStr};
pub use vps::{VpsAction, VpsCommand};

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

mod dns;
mod domain;
mod email;
mod error;
mod invoice;
mod onerror;
mod product;
mod str_extension;
mod vps;

const AVAILABILITY_ZONES: &str = "availibility-zones";
const COMMENT: &str = "#";
const PING: &str = "ping";
const DNS_COMMAND: &str = "dns ";
const DOMAIN_COMMAND: &str = "domain ";
const EMAIL_FORWARD: &str = "email-forward";
const EMAIL_LIST: &str = "email-list";
const EMAIL_BOX: &str = "email-box";
const INVOICE_COMMAND: &str = "invoice ";
const ONERROR_COMMAND: &str = "onerror ";
const PRODUCT_COMMAND: &str = "product ";
const SLEEP_COMMAND: &str = "sleep ";
const VPS_COMMAND: &str = "vps ";

#[derive(Debug, PartialEq)]
pub enum TransipCommand {
    AvailibilityZones,

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

    EmailBox(email::EmailCommand<String>),

    EmailForward(email::EmailCommand<String>),

    EmailList(email::EmailCommand<u64>),

    Invoice(invoice::InvoiceCommand),

    OnError(OnError),

    Product(product::ProductCommand),

    Vps(vps::VpsCommand),

    Ping,

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
            ($trimmed:expr, $command:expr, $sub_command_type:path, $map:path) => {
                if let Some(sub_command) = $trimmed.strip_prefix($command) {
                    return sub_command
                        .trim()
                        .parse::<$sub_command_type>()
                        .err_into()
                        .map($map);
                }
            };
        }

        if s.starts_with(COMMENT) {
            return Ok(TransipCommand::Comment(s.to_owned()));
        }
        let trimmed = s.trim();
        if trimmed == PING {
            return Ok(TransipCommand::Ping);
        }

        if trimmed == AVAILABILITY_ZONES {
            return Ok(TransipCommand::AvailibilityZones);
        }

        parse!(trimmed, SLEEP_COMMAND, u64, TransipCommand::Sleep);
        parse!(trimmed, ONERROR_COMMAND, OnError, TransipCommand::OnError);

        parse!(trimmed, DNS_COMMAND, DnsCommand, TransipCommand::Dns);
        parse!(
            trimmed,
            DOMAIN_COMMAND,
            DomainCommand,
            TransipCommand::Domain
        );
        parse!(
            trimmed,
            INVOICE_COMMAND,
            InvoiceCommand,
            TransipCommand::Invoice
        );
        parse!(
            trimmed,
            PRODUCT_COMMAND,
            ProductCommand,
            TransipCommand::Product
        );
        parse!(trimmed, VPS_COMMAND, VpsCommand, TransipCommand::Vps);

        Err(Error::ParseTransipCommand(s.to_owned()))
    }
}

impl Display for TransipCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransipCommand::AvailibilityZones => write!(f, "{}", AVAILABILITY_ZONES),
            TransipCommand::Comment(comment) => write!(f, "{}", comment),
            TransipCommand::Dns(command) => write!(f, "{}{}", DNS_COMMAND, command),
            TransipCommand::Domain(command) => write!(f, "{}{}", DOMAIN_COMMAND, command),
            TransipCommand::EmailBox(command) => write!(f, "{}", EMAIL_BOX),
            TransipCommand::EmailForward(command) => write!(f, "{}{}", EMAIL_FORWARD, command),
            TransipCommand::EmailList(command) => write!(f, "{}", EMAIL_LIST),
            TransipCommand::Invoice(command) => write!(f, "{}{}", INVOICE_COMMAND, command),
            TransipCommand::OnError(onerror) => write!(f, "{}{}", ONERROR_COMMAND, onerror),
            TransipCommand::Product(command) => write!(f, "{}{}", PRODUCT_COMMAND, command),
            TransipCommand::Sleep(timeout) => write!(f, "{}{}", SLEEP_COMMAND, timeout),
            TransipCommand::Vps(command) => write!(f, "{}{}", VPS_COMMAND, command),
            TransipCommand::Ping => write!(f, "{}", PING),
        }
    }
}

fn check_environment(name: &str) -> std::result::Result<String, VarError> {
    if name.starts_with("${") && name.ends_with('}') {
        let s = name[2..name.len() - 1].trim();
        std::env::var(s)
    } else {
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
            TransipCommand::OnError(crate::OnError::Exit).to_string(),
            "onerror exit".to_owned(),
        );

        assert_eq!(
            TransipCommand::OnError(crate::OnError::Print).to_string(),
            "onerror print".to_owned(),
        );

        assert_eq!(TransipCommand::Sleep(45).to_string(), "sleep 45".to_owned(),);

        assert_eq!(TransipCommand::Ping.to_string(), "ping".to_owned(),);

        assert_eq!(
            TransipCommand::AvailibilityZones.to_string(),
            "availibility-zones".to_owned()
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
            "vps \treset paulusminus-vps2"
                .parse::<TransipCommand>()
                .unwrap(),
            TransipCommand::Vps(crate::VpsCommand::Action(
                "paulusminus-vps2".to_owned(),
                crate::VpsAction::Reset,
            ))
        );

        assert_eq!(
            "sleep 3984".parse::<TransipCommand>().unwrap(),
            TransipCommand::Sleep(3984),
        );

        assert_eq!(
            "onerror print  ".parse::<TransipCommand>().unwrap(),
            TransipCommand::OnError(crate::OnError::Print),
        );

        assert_eq!(
            "onerror   exit".parse::<TransipCommand>().unwrap(),
            TransipCommand::OnError(crate::OnError::Exit),
        );

        assert_eq!(
            " ping ".parse::<TransipCommand>().unwrap(),
            TransipCommand::Ping,
        );

        assert_eq!(
            "availibility-zones ".parse::<TransipCommand>().unwrap(),
            TransipCommand::AvailibilityZones,
        );
    }
}
