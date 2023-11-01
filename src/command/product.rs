use pest::iterators::Pair;
use transip::{api::general::GeneralApi, Client};

use crate::{
    error::{Error, ErrorExt},
    to_json, Result, Rule,
};

pub type ProductName = String;

#[derive(Debug, PartialEq)]
pub enum ProductCommand {
    List,
    Elements(ProductName),
}

impl<'a> TryFrom<Pair<'a, Rule>> for ProductCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> std::result::Result<Self, Self::Error> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::product_list => Ok(ProductCommand::List),
            Rule::product_elements => {
                let mut inner = inner.into_inner();
                let name = inner.next().unwrap().as_str().trim();
                Ok(ProductCommand::Elements(name.to_owned()))
            }
            _ => Err(Error::ParseInvoiceCommand(commandline)),
        }            
    }
}

pub fn execute(pair: Pair<'_, Rule>, client: &mut Client) -> Result<String> {
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
