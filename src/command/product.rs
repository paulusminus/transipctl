use pest::iterators::Pair;
use transip::{Client, api::general::GeneralApi};

use crate::{Result, Rule, error::{ErrorExt, Error}, to_json};

pub fn execute(pair: Pair<'_, Rule> , client: &mut Client) -> Result<String> {
    let commandline = pair.as_str().to_owned();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::product_list => client.products().err_into().and_then(to_json),
        Rule::product_elements => {
            let mut inner = inner.into_inner();
            let name = inner.next().unwrap().as_str().trim();
            client.product_elements(name).err_into().and_then(to_json)
        }
        _ => Err(Error::ParseInvoiceCommand(commandline)),
    }
}