use crate::{error::Error, Rule};
use pest::iterators::Pair;
use strum::EnumString;

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
                action
                    .parse::<VpsAction>()
                    .map_err(|_| Error::ParseVpsCommand(commandline))
                    .map(|action| VpsCommand::Action(name.to_owned(), action))
            }
            _ => Err(Error::ParseVpsCommand(commandline)),
        }
    }
}
