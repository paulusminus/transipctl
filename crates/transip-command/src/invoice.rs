use crate::{error::Error, str_extension::StrExtension};
use std::{fmt::Display, str::FromStr};
use strum::{Display, EnumString};

const LIST: &str = "list";

#[derive(Debug, PartialEq, EnumString, Display)]
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
    ///     TransipCommand::Invoice(
    ///         InvoiceCommand::List
    ///     ),
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

impl Display for InvoiceCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoiceCommand::Action(number, action) => write!(f, "{} {}", action, number),
            InvoiceCommand::List => write!(f, "{}", LIST),
        }
    }
}

impl FromStr for InvoiceCommand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim() == LIST {
            return Ok(InvoiceCommand::List);
        }

        for action in [InvoiceAction::Item, InvoiceAction::Pdf] {
            if let Some(invoice_number) = s.one_param(action.to_string().as_str()) {
                return Ok(InvoiceCommand::Action(invoice_number.to_owned(), action));
            }
        }

        Err(Error::ParseInvoiceCommand(s.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use super::{InvoiceAction, InvoiceCommand};

    #[test]
    fn display() {
        assert_eq!(
            InvoiceCommand::Action("98234".to_owned(), InvoiceAction::Item).to_string(),
            "item 98234".to_owned(),
        );

        assert_eq!(
            InvoiceCommand::Action("98234".to_owned(), InvoiceAction::Pdf).to_string(),
            "pdf 98234".to_owned(),
        );

        assert_eq!(InvoiceCommand::List.to_string(), "list".to_owned(),);
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "list".parse::<InvoiceCommand>().unwrap(),
            InvoiceCommand::List,
        );

        assert_eq!(
            "item 98874".parse::<InvoiceCommand>().unwrap(),
            InvoiceCommand::Action("98874".to_owned(), InvoiceAction::Item),
        );

        assert_eq!(
            "pdf 98874".parse::<InvoiceCommand>().unwrap(),
            InvoiceCommand::Action("98874".to_owned(), InvoiceAction::Pdf),
        );
    }
}
