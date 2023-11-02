use crate::{error::Error, Result, Rule};
use pest::iterators::Pair;

use super::parameter;

pub type DomainName = String;

#[derive(Debug, PartialEq)]
pub enum DomainCommand {
    List,
    Item(DomainName),
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
                Ok(DomainCommand::Item(name.to_owned()))
            }
            _ => Err(Error::ParseDomainCommand(commandline)),
        }
    }
}
