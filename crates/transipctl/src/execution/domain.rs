use super::ToJson;
use crate::Result;
use transip::{api::domain::DomainApi, Client};
use transip_command::domain::DomainCommand;

pub fn execute(command: DomainCommand, client: &mut Client) -> Result<String> {
    match command {
        DomainCommand::Item(name) => client.domain_item(&name).and_then_json(),
        DomainCommand::List => client.domain_list().and_then_json(),
    }
}
