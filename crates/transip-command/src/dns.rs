use std::fmt::Display;

use super::parameter;
use crate::{error::Error, parse::Rule, Result};
use pest::iterators::Pair;

pub type DomainName = String;

#[derive(Debug, PartialEq)]
pub enum DnsCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns list lkdjfie.nl";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
    ///         DnsCommand::List(
    ///             "lkdjfie.nl".to_owned()
    ///         )
    ///     ),
    /// );
    /// ```
    List(DomainName),

    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns acme-challenge-delete lkdfjf.nl";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
    ///         DnsCommand::DeleteAcmeChallenge(
    ///             "lkdfjf.nl".to_owned()
    ///         )
    ///     ),
    /// );
    /// ```
    DeleteAcmeChallenge(DomainName),

    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns acme-challenge-set lkdfjf.nl oe8rtg";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
    ///         DnsCommand::SetAcmeChallenge(
    ///             "lkdfjf.nl".to_owned(),
    ///             "oe8rtg".to_owned(),
    ///         )
    ///     ),
    /// );
    /// ```
    SetAcmeChallenge(DomainName, String),
}

impl Display for DnsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsCommand::DeleteAcmeChallenge(name) => write!(f, "acme-challenge-delete {}", name),
            DnsCommand::List(name) => write!(f, "list {}", name),
            DnsCommand::SetAcmeChallenge(name, challenge) => write!(f, "acme-challenge-set {} {}", name, challenge),
        }
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for DnsCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::dns_list => {
                let name = parameter(inner.into_inner().next().unwrap())?;
                Ok(DnsCommand::List(name))
            }
            Rule::dns_delete_acme_challenge => {
                let name = parameter(inner.into_inner().next().unwrap())?;
                Ok(DnsCommand::DeleteAcmeChallenge(name))
            }
            Rule::dns_set_acme_challenge => {
                let mut inner = inner.into_inner();
                let name = parameter(inner.next().unwrap())?;
                let value = parameter(inner.next().unwrap())?;
                Ok(DnsCommand::SetAcmeChallenge(name, value))
            }
            _ => Err(Error::ParseDnsCommand(commandline)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::DnsCommand;

    #[test]
    fn display() {
        assert_eq!(
            DnsCommand::List("paulmin.nl".to_owned()).to_string(),
            "list paulmin.nl".to_owned(),
        );

        assert_eq!(
            DnsCommand::DeleteAcmeChallenge("paulmin.nl".to_owned()).to_string(),
            "acme-challenge-delete paulmin.nl".to_owned(),
        );

        assert_eq!(
            DnsCommand::SetAcmeChallenge("paulmin.nl".to_owned(), "hallo".to_owned()).to_string(),
            "acme-challenge-set paulmin.nl hallo".to_owned(),
        );
    }
}