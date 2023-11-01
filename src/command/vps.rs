use crate::{
    error::{Error, ErrorExt},
    to_json, unit_to_string, Result, Rule,
};
use pest::iterators::Pair;
use strum::EnumString;
use transip::{api::vps::VpsApi, Client};

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

    fn try_from(pair: Pair<'a, Rule>) -> std::result::Result<Self, Self::Error> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::vps_list => Ok(VpsCommand::List),
            Rule::vps_item_action => {
                let mut inner = inner.into_inner();
                let action = inner.next().unwrap().as_str().trim();
                let name = inner.next().unwrap().as_str().trim();
                action.parse::<VpsAction>()
                .map_err(|_| Error::ParseVpsCommand(commandline))
                .map(|action| VpsCommand::Action(name.to_owned(), action))
            }
            _ => Err(Error::ParseVpsCommand(commandline)),
        }
    }
}

pub fn execute(pair: Pair<'_, Rule>, client: &mut Client) -> Result<String> {
    let commandline = pair.as_str().to_owned();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::vps_list => client.vps_list().err_into().and_then(to_json),
        Rule::vps_item_action => {
            let mut inner = inner.into_inner();
            let action = inner.next().unwrap().as_str().trim();
            let name = inner.next().unwrap().as_str().trim();
            if action == "item" {
                client.vps(name).err_into().and_then(to_json)
            } else if action == "reset" {
                client.vps_reset(name).err_into().map(unit_to_string)
            } else if action == "start" {
                client.vps_start(name).err_into().map(unit_to_string)
            } else if action == "stop" {
                client.vps_stop(name).err_into().map(unit_to_string)
            } else if action == "lock" {
                client
                    .vps_set_is_locked(name, true)
                    .err_into()
                    .map(unit_to_string)
            } else if action == "unlock" {
                client
                    .vps_set_is_locked(name, false)
                    .err_into()
                    .map(unit_to_string)
            } else {
                Err(Error::ParseVpsCommand(commandline))
            }
        }
        _ => Err(Error::ParseVpsCommand(commandline)),
    }
}
