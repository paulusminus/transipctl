use crate::{error::Error, Rule};
use pest::iterators::Pair;

use super::parameter;

pub type DomainName = String;

#[derive(Debug, PartialEq)]
pub enum DnsCommand {
    List(DomainName),
    DeleteAcmeChallenge(DomainName),
    SetAcmeChallenge(DomainName, String),
}

impl<'a> TryFrom<Pair<'a, Rule>> for DnsCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> std::result::Result<Self, Self::Error> {
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
