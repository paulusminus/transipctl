use pest::iterators::Pair;

use crate::{error::Error, Rule};

use super::parameter;

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
                let name = parameter(inner.next().unwrap())?;
                Ok(ProductCommand::Elements(name.to_owned()))
            }
            _ => Err(Error::ParseInvoiceCommand(commandline)),
        }
    }
}
