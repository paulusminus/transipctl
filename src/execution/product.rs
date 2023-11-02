use super::{Execution, ToJson};
use crate::{command::product::ProductCommand, Result};
use transip::{api::general::GeneralApi, Client};

impl Execution for ProductCommand {
    fn execute(&self, client: &mut Client) -> Result<String> {
        match self {
            Self::Elements(elements) => client.product_elements(elements).and_then_json(),
            Self::List => client.products().and_then_json(),
        }
    }
}
