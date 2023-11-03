use super::ToJson;
use crate::Result;
use transip::{api::general::GeneralApi, Client};
use transip_command::product::ProductCommand;

pub fn execute(command: ProductCommand, client: &mut Client) -> Result<String> {
    match command {
        ProductCommand::Elements(elements) => client.product_elements(&elements).and_then_json(),
        ProductCommand::List => client.products().and_then_json(),
    }
}
