use super::parameter;
use crate::{
    error::{Error, ErrorExt},
    parse::Rule,
    Result,
};
use pest::iterators::Pair;
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum InvoiceAction {
    Item,
    Pdf,
}

pub type InvoiceNumber = String;

#[derive(Debug, PartialEq)]
pub enum InvoiceCommand {
    /// # Example
    ///
    /// ```
    /// use transip_command::{InvoiceCommand, TransipCommand};
    ///
    /// let commandline = "invoice list";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Invoice(InvoiceCommand::List),
    /// );
    /// ```
    List,

    /// # Example
    ///
    /// ```
    /// use transip_command::{InvoiceCommand, InvoiceAction, TransipCommand};
    ///
    /// let commandline = "invoice item 938474";
    /// assert_eq!(
    ///     commandline.parse::<TransipCommand>().unwrap(),
    ///     TransipCommand::Invoice(
    ///         InvoiceCommand::Action(
    ///             "938474".to_owned(),
    ///             InvoiceAction::Item,
    ///         )
    ///     ),
    /// );
    /// ```
    Action(InvoiceNumber, InvoiceAction),
}

impl<'a> TryFrom<Pair<'a, Rule>> for InvoiceCommand {
    type Error = Error;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self> {
        let commandline = pair.as_str().to_owned();
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::invoice_list => Ok(Self::List),
            Rule::invoice_item_action => {
                let mut inner = inner.into_inner();
                let action = inner.next().unwrap().as_str().trim();
                let name = parameter(inner.next().unwrap())?;
                action
                    .parse::<InvoiceAction>()
                    .err_into()
                    .map(|action| Self::Action(name, action))
            }
            _ => Err(Error::ParseInvoiceCommand(commandline)),
        }
    }
}
