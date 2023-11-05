use std::fmt::Display;

use super::parameter;
use crate::{error::Error, parse::Rule, Result};
use pest::iterators::Pair;

pub type DomainName = String;

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
            DomainCommand::Item(item) => write!(f, "item {}", item),
            DomainCommand::List => write!(f, "list"),
        }
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for DomainCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::domain_list => Ok(DomainCommand::List),
            Rule::domain_item => {
                let mut inner = inner.into_inner();
                let name = parameter(inner.next().unwrap())?;
                Ok(DomainCommand::Item(name))
            }
            _ => Err(Error::ParseDomainCommand(commandline)),
        }
    }
}
