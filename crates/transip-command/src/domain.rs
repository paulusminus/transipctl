use crate::{check_environment, error::Error, str_extension::StrExtension};
use std::{fmt::Display, str::FromStr};

pub type DomainName = String;

const ITEM: &str = "item";
const LIST: &str = "list";

#[derive(Debug, PartialEq)]
pub enum DomainCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::{DomainCommand, TransipCommand};
    ///
    /// let commandline = "domain list";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Domain(DomainCommand::List),
    /// );
    /// ```
    List,

    /// # Example
    ///
    /// ```
    /// use transip_command::{DomainCommand, TransipCommand};
    ///
    /// let commandline = "domain item oiwerhy.nl";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Domain(
    ///         DomainCommand::Item(
    ///             "oiwerhy.nl".to_owned(),
    ///         )
    ///     ),
    /// );
    /// ```
    Item(DomainName),
}

impl Display for DomainCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainCommand::Item(item) => write!(f, "{} {}", ITEM, item),
            DomainCommand::List => write!(f, "{}", LIST),
        }
    }
}

impl FromStr for DomainCommand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim() == LIST {
            return Ok(DomainCommand::List);
        }
        if let Some(domain_name) = s.one_param(ITEM) {
            Ok(DomainCommand::Item(check_environment(domain_name)?))
        } else {
            Err(Error::ParseDomainCommand(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::DomainCommand;

    #[test]
    fn display() {
        assert_eq!(
            DomainCommand::Item("paulmin.nl".to_owned()).to_string(),
            "item paulmin.nl".to_owned(),
        );

        assert_eq!(DomainCommand::List.to_string(), "list".to_owned(),);
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "list".parse::<DomainCommand>().unwrap(),
            DomainCommand::List,
        );

        assert_eq!(
            "item paulmin.nl".parse::<DomainCommand>().unwrap(),
            DomainCommand::Item("paulmin.nl".to_owned()),
        );
    }
}
