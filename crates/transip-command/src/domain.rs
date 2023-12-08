use crate::{check_environment, error::DomainCommandError, str_extension::Words};
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
    type Err = DomainCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DomainCommand::try_from(Words::from(s))
    }
}

impl<'a> TryFrom<Words<'a>> for DomainCommand {
    type Error = DomainCommandError;

    fn try_from(mut words: Words<'a>) -> Result<Self, Self::Error> {
        let sub_command = words.next().ok_or(DomainCommandError::MissingSubCommand)?;

        if sub_command == LIST {
            if let Some(rest) = words.rest() {
                Err(DomainCommandError::TooManyParameters(rest.to_owned()))
            } else {
                Ok(DomainCommand::List)
            }
        } else if sub_command == ITEM {
            let domain_name = words.next().ok_or(DomainCommandError::MissingDomainName)?;
            if let Some(rest) = words.rest() {
                Err(DomainCommandError::TooManyParameters(rest.to_owned()))
            } else {
                Ok(DomainCommand::Item(check_environment(domain_name)?))
            }
        } else {
            Err(DomainCommandError::WrongSubCommand(sub_command.to_owned()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::str_extension::Words;

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
    fn try_from() {
        assert_eq!(
            DomainCommand::try_from(Words::from("  item   paulmin.nl   ")).unwrap(),
            DomainCommand::Item("paulmin.nl".to_owned())
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            DomainCommand::try_from(Words::from("list")).unwrap(),
            DomainCommand::List,
        );

        assert_eq!(
            DomainCommand::try_from(Words::from("item paulmin.nl")).unwrap(),
            DomainCommand::Item("paulmin.nl".to_owned()),
        );
    }
}
