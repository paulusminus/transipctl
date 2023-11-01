use crate::{error::Error, Rule};
use pest::iterators::Pair;

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
                let name = inner.into_inner().next().unwrap().as_str().to_owned();
                Ok(DnsCommand::List(name))
            }
            Rule::dns_record_delete_acme_challenge => {
                let name = inner.into_inner().next().unwrap().as_str().to_owned();
                Ok(DnsCommand::DeleteAcmeChallenge(name))
            }
            Rule::dns_record_set_acme_challenge => {
                let mut inner = inner.into_inner();
                let name = inner.next().unwrap().as_str().to_owned();
                let value = inner.next().unwrap().as_str().to_owned();
                Ok(DnsCommand::SetAcmeChallenge(name, value))
            }
            _ => Err(Error::ParseDnsCommand(commandline)),
        }
    }
}
