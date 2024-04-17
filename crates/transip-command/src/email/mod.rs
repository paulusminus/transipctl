use std::{fmt::Display, str::FromStr};

use crate::{
    error::{EmailCommandError, ErrorExt, TooMany},
    str_extension::Words,
    Error, Result,
};

const DELETE: &str = "delete";
const INSERT: &str = "insert";
const ITEM: &str = "item";
const LIST: &str = "list";
const UPDATE: &str = "update";

type DomainName = String;

#[derive(Debug, PartialEq)]
pub enum EmailCommand<I: Display + FromStr>
where
    I: Display + FromStr,
    <I as FromStr>::Err: Into<Error>,
{
    Delete(DomainName, I),
    Insert(DomainName, String),
    Item(DomainName, I),
    List(DomainName),
    Update(DomainName, I, String),
}

impl<I> std::fmt::Display for EmailCommand<I>
where
    I: Display + FromStr,
    <I as FromStr>::Err: Into<Error>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailCommand::Delete(domain_name, i) => write!(f, "{} {} {}", DELETE, domain_name, i),
            EmailCommand::Insert(domain_name, s) => write!(f, "{} {} {}", INSERT, domain_name, s),
            EmailCommand::Item(domain_name, i) => write!(f, "{} {} {}", ITEM, domain_name, i),
            EmailCommand::List(domain_name) => write!(f, "{} {}", LIST, domain_name),
            EmailCommand::Update(domain_name, i, s) => {
                write!(f, "{} {} {} {}", UPDATE, domain_name, i, s)
            }
        }
    }
}

impl<I> FromStr for EmailCommand<I>
where
    I: Display + FromStr,
    <I as FromStr>::Err: Into<Error>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(Words::from(s))
    }
}

impl<I> TryFrom<Words<'_>> for EmailCommand<I>
where
    I: Display + FromStr,
    <I as FromStr>::Err: Into<Error>,
{
    type Error = Error;
    fn try_from(mut value: Words<'_>) -> Result<Self> {
        let subcommand = value.next().ok_or(EmailCommandError::MissingSubCommand)?;
        let domain_name = value.next().ok_or(EmailCommandError::MissingDomainName)?;

        if subcommand == LIST {
            value
                .next()
                .too_many()
                .map_err(EmailCommandError::TooManyParameters)?;
            Ok(Self::List(domain_name.to_owned()))
        } else if subcommand == ITEM {
            let id_str = value.next().ok_or(EmailCommandError::MissingId)?;
            let id = id_str.parse::<I>().err_into()?;
            value
                .next()
                .too_many()
                .map_err(EmailCommandError::TooManyParameters)?;
            Ok(Self::Item(domain_name.to_owned(), id))
        } else if subcommand == DELETE {
            let id_str = value.next().ok_or(EmailCommandError::MissingId)?;
            let id = id_str.parse::<I>().err_into()?;
            value
                .next()
                .too_many()
                .map_err(EmailCommandError::TooManyParameters)?;
            Ok(Self::Delete(domain_name.to_owned(), id))
        } else if subcommand == INSERT {
            let v = value.next().ok_or(EmailCommandError::MissingId)?;
            value
                .next()
                .too_many()
                .map_err(EmailCommandError::TooManyParameters)?;
            Ok(Self::Insert(domain_name.to_owned(), v.to_owned()))
        } else if subcommand == UPDATE {
            let id_str = value.next().ok_or(EmailCommandError::MissingId)?;
            let id = id_str.parse::<I>().err_into()?;
            let v = value.next().ok_or(EmailCommandError::MissingId)?;
            value
                .next()
                .too_many()
                .map_err(EmailCommandError::TooManyParameters)?;
            Ok(Self::Update(domain_name.to_owned(), id, v.to_owned()))
        } else {
            Err(EmailCommandError::WrongSubCommand(subcommand).into())
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_from_str() {
        let s = "hallo".parse::<String>().unwrap();
    }
}
