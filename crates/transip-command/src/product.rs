use std::{fmt::Display, str::FromStr};
use crate::{error::Error, str_extension::StrExtension};

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
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim() == LIST {
            return Ok(ProductCommand::List);
        }

        if let Some(product_name) = s.one_param(ELEMENTS) {
            return Ok(ProductCommand::Elements(product_name.to_owned()));
        }
        
        Err(Error::ParseProductCommand(s.to_owned()))
    }
}

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
    fn from_str() {
        assert_eq!(
            "list".parse::<ProductCommand>().unwrap(),
            ProductCommand::List,
        );

        assert!(
            "list kdf".parse::<ProductCommand>().is_err()
        );

        assert_eq!(
            "elements lkjdf".parse::<ProductCommand>().unwrap(),
            ProductCommand::Elements("lkjdf".to_owned()),
        );

        assert!(
            "elements ldkfj dkfjf".parse::<ProductCommand>().is_err()
        );
    }
}
