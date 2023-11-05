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
    ///         DnsCommand::AcmeChallengeDelete(
    ///             "lkdfjf.nl".to_owned()
    ///         )
    ///     ),
    /// );
    /// ```
    AcmeChallengeDelete(DomainName),

    /// # Example
    ///
    /// ```
    /// use transip_command::{DnsCommand, TransipCommand};
    ///
    /// let commandline = "dns acme-challenge-set lkdfjf.nl oe8rtg";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Dns(
    ///         DnsCommand::AcmeChallengeSet(
    ///             "lkdfjf.nl".to_owned(),
    ///             "oe8rtg".to_owned(),
    ///         )
    ///     ),
    /// );
    /// ```
    AcmeChallengeSet(DomainName, String),
}

impl Display for DnsCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsCommand::AcmeChallengeDelete(name) => write!(f, "acme-challenge-delete {}", name),
            DnsCommand::List(name) => write!(f, "list {}", name),
            DnsCommand::AcmeChallengeSet(name, challenge) => write!(f, "acme-challenge-set {} {}", name, challenge),
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
            Rule::dns_acme_challenge_delete => {
                let name = parameter(inner.into_inner().next().unwrap())?;
                Ok(DnsCommand::AcmeChallengeDelete(name))
            }
            Rule::dns_acme_challenge_set => {
                let mut inner = inner.into_inner();
                let name = parameter(inner.next().unwrap())?;
                let value = parameter(inner.next().unwrap())?;
                Ok(DnsCommand::AcmeChallengeSet(name, value))
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
            DnsCommand::AcmeChallengeDelete("paulmin.nl".to_owned()).to_string(),
            "acme-challenge-delete paulmin.nl".to_owned(),
        );

        assert_eq!(
            DnsCommand::AcmeChallengeSet("paulmin.nl".to_owned(), "hallo".to_owned()).to_string(),
            "acme-challenge-set paulmin.nl hallo".to_owned(),
        );
    }
}