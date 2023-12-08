use crate::{error::ProductCommandError, str_extension::Words};
use std::{fmt::Display, str::FromStr};

pub type ProductName = String;

const LIST: &str = "list";
const ELEMENTS: &str = "elements";

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

impl FromStr for ProductCommand {
    type Err = ProductCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ProductCommand::try_from(Words::from(s))
    }
}

impl<'a> TryFrom<Words<'a>> for ProductCommand {
    type Error = ProductCommandError;

    fn try_from(mut words: Words<'a>) -> Result<Self, Self::Error> {
        let sub_command = words.next().ok_or(ProductCommandError::MissingSubCommand)?;

        if sub_command == LIST {
            if let Some(rest) = words.rest() {
                Err(ProductCommandError::TooManyParameters(rest.to_owned()))
            } else {
                Ok(ProductCommand::List)
            }
        } else if sub_command == ELEMENTS {
            let product_name = words
                .next()
                .ok_or(ProductCommandError::MissingProductName)?;
            if let Some(rest) = words.rest() {
                Err(ProductCommandError::TooManyParameters(rest.to_owned()))
            } else {
                Ok(ProductCommand::Elements(product_name.to_owned()))
            }
        } else {
            Err(ProductCommandError::WrongSubCommand(sub_command.to_owned()))
        }
    }
}

// impl FromStr for ProductCommand {
//     type Err = Error;

//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         if s.trim() == LIST {
//             return Ok(ProductCommand::List);
//         }

//         if let Some(product_name) = s.one_param(ELEMENTS) {
//             return Ok(ProductCommand::Elements(product_name.to_owned()));
//         }

//         Err(Error::ParseProductCommand(s.to_owned()))
//     }
// }

impl Display for ProductCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductCommand::Elements(name) => write!(f, "{} {}", ELEMENTS, name),
            ProductCommand::List => write!(f, "{}", LIST),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::str_extension::Words;

    use super::ProductCommand;

    #[test]
    fn display() {
        assert_eq!(
            ProductCommand::Elements("haip-basic-contract".to_owned()).to_string(),
            "elements haip-basic-contract".to_owned(),
        );

        assert_eq!(ProductCommand::List.to_string(), "list".to_string(),);
    }

    #[test]
    fn from_words() {
        assert_eq!(
            ProductCommand::try_from(Words::from("list")).unwrap(),
            ProductCommand::List,
        );

        assert!(ProductCommand::try_from(Words::from("list kdf")).is_err());

        assert_eq!(
            ProductCommand::try_from(Words::from("elements lkjdf")).unwrap(),
            ProductCommand::Elements("lkjdf".to_owned()),
        );

        assert!(ProductCommand::try_from(Words::from("elements ldkfj dkfjf")).is_err());
    }
}
