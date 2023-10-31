use crate::{
    error::{Error, ErrorExt},
    to_json, unit_to_string, Result, Rule,
};
use pest::iterators::Pair;
use transip::{api::vps::VpsApi, Client};

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
