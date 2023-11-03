use super::{Execution, ToJson};
use crate::{command::domain::DomainCommand, Result};
use transip::{api::domain::DomainApi, Client};

impl Execution for DomainCommand {
    fn execute(&self, client: &mut Client) -> Result<String> {
        match self {
            Self::Item(name) => client.domain_item(name).and_then_json(),
            Self::List => client.domain_list().and_then_json(),
        }
    }
}
