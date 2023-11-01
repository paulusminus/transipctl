use crate::{
    error::{Error, ErrorExt},
    to_json, Result, Rule, unit_to_string,
};
use pest::iterators::Pair;
use transip::{api::dns::{DnsApi, DnsEntry}, Client};

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

pub fn execute(pair: Pair<'_, Rule>, client: &mut Client) -> Result<String> {
    let commandline = pair.to_string();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::dns_list => {
            let name = inner.into_inner().next().unwrap().as_str();
            client.dns_entry_list(name).err_into().and_then(to_json)
        }
        Rule::dns_record_delete_acme_challenge => {
            let name = inner.into_inner().next().unwrap().as_str();
            client.dns_entry_delete_all(name, DnsEntry::is_acme_challenge)
            .err_into()
            .map(unit_to_string)
        }
        Rule::dns_record_set_acme_challenge => {
            let mut inner = inner.into_inner();
            let name = inner.next().unwrap().as_str();
            let value = inner.next().unwrap().as_str();
            client.dns_entry_delete_all(name, DnsEntry::is_acme_challenge)
            .and_then(|_| client.dns_entry_insert(name, DnsEntry::new_acme_challenge(60, value)))
            .err_into()
            .map(unit_to_string)
        }
        _ => Err(Error::ParseDnsCommand(commandline)),
    }
}
