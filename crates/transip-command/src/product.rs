use std::fmt::Display;

use super::parameter;
use crate::{error::Error, parse::Rule, Result};
use pest::iterators::Pair;

pub type ProductName = String;

#[derive(Debug, PartialEq)]
pub enum ProductCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::{ProductCommand, TransipCommand};
    ///
    /// let commandline = "product list";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Product(ProductCommand::List),
    /// );
    /// ```
    List,

    /// # Example
    ///
    /// ```
    /// use transip_command::{ProductCommand, TransipCommand};
    ///
    /// let commandline = "product elements haip-basic-contract";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Product(
    ///         ProductCommand::Elements(
    ///             "haip-basic-contract".to_owned(),
    ///         )
    ///     ),
    /// );
    /// ```
    Elements(ProductName),
}

impl Display for ProductCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductCommand::Elements(name) => write!(f, "elements {}", name),
            ProductCommand::List => write!(f, "list"),
        }
    }
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
