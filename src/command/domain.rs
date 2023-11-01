use crate::{
    error::{Error, ErrorExt},
    to_json, Result, Rule,
};
use pest::iterators::Pair;
use transip::{api::domain::DomainApi, Client};

pub type DomainName = String;

#[derive(Debug, PartialEq)]
pub enum DomainCommand {
    List,
    Item(DomainName),
}

impl<'a> TryFrom<Pair<'a, Rule>> for DomainCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> std::result::Result<Self, Self::Error> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::domain_list => Ok(DomainCommand::List),
            Rule::domain_item => {
                let mut inner = inner.into_inner();
                let name = inner.next().unwrap().as_str().trim();
                Ok(DomainCommand::Item(name.to_owned()))
            }
            _ => Err(Error::ParseDomainCommand(commandline)),
        }            
    }
}

pub fn execute(pair: Pair<'_, Rule>, client: &mut Client) -> Result<String> {
    let commandline = pair.as_str().to_owned();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::domain_list => client.domain_list().err_into().and_then(to_json),
        Rule::domain_item => {
            let mut inner = inner.into_inner();
            let name = inner.next().unwrap().as_str().trim();
            client.domain_item(name).err_into().and_then(to_json)
        }
        _ => Err(Error::ParseDomainCommand(commandline)),
    }
}
