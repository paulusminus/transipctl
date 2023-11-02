use transip::{api::general::GeneralApi, Client};

use crate::{command::product::ProductCommand, error::Error};

use super::{Execution, ToJson};

impl Execution for ProductCommand {
    fn execute(&self, client: &mut Client) -> Result<String, Error> {
        match self {
            Self::Elements(elements) => client.product_elements(elements).and_then_json(),
            Self::List => client.products().and_then_json(),
        }
    }
}
