use crate::{
    error::{Error, ErrorExt},
    Result, Rule,
};
use pest::iterators::Pair;
use strum::EnumString;

use super::parameter;

pub type VpsName = String;

#[derive(Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum VpsAction {
    Item,
    Lock,
    Unlock,
    Start,
    Stop,
    Reset,
}

#[derive(Debug, PartialEq)]
pub enum VpsCommand {
    List,
    Action(VpsName, VpsAction),
}

impl<'a> TryFrom<Pair<'a, Rule>> for VpsCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::vps_list => Ok(VpsCommand::List),
            Rule::vps_item_action => {
                let mut inner = inner.into_inner();
                let action = inner.next().unwrap().as_str().trim();
                let name = parameter(inner.next().unwrap())?;
                action
                    .parse::<VpsAction>()
                    .map_err(|_| Error::ParseVpsCommand(commandline))
                    .map(|action| VpsCommand::Action(name, action))
            }
            _ => Err(Error::ParseVpsCommand(commandline)),
        }
    }
}

pub struct Parameter(String);

impl<'a> TryFrom<Pair<'a, Rule>> for Parameter {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self> {
        match pair.as_rule() {
            Rule::value => Ok(Parameter(pair.as_str().to_owned())),
            Rule::env => std::env::var(pair.as_str()).err_into().map(Parameter),
            _ => Err(Error::ParseTransipCommand("Failure".to_owned())),
        }
    }
}
