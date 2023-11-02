use super::parameter;
use crate::{error::Error, Result, Rule};
use pest::iterators::Pair;

pub type ProductName = String;

#[derive(Debug, PartialEq)]
pub enum ProductCommand {
    List,
    Elements(ProductName),
}

impl<'a> TryFrom<Pair<'a, Rule>> for ProductCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::product_list => Ok(ProductCommand::List),
            Rule::product_elements => {
                let name = parameter(inner.into_inner().next().unwrap())?;
                Ok(ProductCommand::Elements(name))
            }
            _ => Err(Error::ParseInvoiceCommand(commandline)),
        }
    }
}
