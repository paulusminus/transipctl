use crate::{error::InvoiceCommandError, str_extension::Words};
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
    type Err = InvoiceCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InvoiceCommand::try_from(Words::from(s))
    }
}

impl<'a> TryFrom<Words<'a>> for InvoiceCommand {
    type Error = InvoiceCommandError;

    fn try_from(mut words: Words<'a>) -> Result<Self, Self::Error> {
        let sub_command = words.next().ok_or(InvoiceCommandError::MissingSubCommand)?;

        if sub_command == LIST {
            if let Some(rest) = words.rest() {
                return Err(InvoiceCommandError::TooManyParameters(rest.to_owned()));
            } else {
                return Ok(InvoiceCommand::List);
            }
        }

        let action = sub_command.parse::<InvoiceAction>()?;
        let invoice_number = words
            .next()
            .ok_or(InvoiceCommandError::MissingInvoiceNumber)?;

        if let Some(rest) = words.rest() {
            Err(InvoiceCommandError::TooManyParameters(rest.to_owned()))
        } else {
            Ok(InvoiceCommand::Action(invoice_number.to_owned(), action))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::str_extension::Words;

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
            InvoiceCommand::try_from(Words::from("list")).unwrap(),
            InvoiceCommand::List,
        );

        assert_eq!(
            InvoiceCommand::try_from(Words::from("item 98874")).unwrap(),
            InvoiceCommand::Action("98874".to_owned(), InvoiceAction::Item),
        );

        assert_eq!(
            InvoiceCommand::try_from(Words::from("pdf 98874")).unwrap(),
            InvoiceCommand::Action("98874".to_owned(), InvoiceAction::Pdf),
        );
    }

    #[test]
    fn from_words() {
        let mut words: Words = "list".into();
        let _sub_command = words
            .next()
            .ok_or(crate::error::InvoiceCommandError::MissingSubCommand)
            .unwrap();
    }
}
