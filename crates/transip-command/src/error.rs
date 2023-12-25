use crate::Result;
use std::{env::VarError, num::ParseIntError};
use strum::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum DnsCommandError {
    #[error("Domain name missing")]
    DomainNameMissing,

    #[error("Validation missing")]
    ValidationMissing,

    #[error("Record type missing")]
    RecordTypeMissing,

    #[error("Invalid record type")]
    InvalidRecord(strum::ParseError),

    #[error("Content missing")]
    ContentMissing,

    #[error("Too many parameter for {0}")]
    TooManyParameters(String),

    #[error("Wrong subcommand {0}")]
    WrongSubCommand(String),

    #[error("Missing subcommand")]
    MissingSubCommand,

    #[error("Dns recordname not provided")]
    NoDnsRecordName,

    #[error("Dns ttl not provided")]
    NoTTL,

    #[error("Invalid ttl: {0}")]
    InvalidTTL(#[from] ParseIntError),

    #[error("Invalid record type: {0}")]
    InvalidRecordType(ParseError),

    #[error("Environment: {0}")]
    Environment(#[from] VarError),
}

#[derive(thiserror::Error, Debug)]
pub enum VpsCommandError {
    #[error("Too many parameter for {0}")]
    TooManyParameters(String),

    #[error("Wrong subcommand {0}")]
    WrongSubCommand(&'static str),

    #[error("Missing subcommand")]
    MissingSubCommand,

    #[error("Parsing action")]
    ParsingAction(#[from] strum::ParseError),

    #[error("Environment: {0}")]
    Environment(#[from] VarError),
}

#[derive(thiserror::Error, Debug)]
pub enum ProductCommandError {
    #[error("Too many parameter for {0}")]
    TooManyParameters(String),

    #[error("Wrong subcommand {0}")]
    WrongSubCommand(String),

    #[error("Missing subcommand")]
    MissingSubCommand,

    #[error("Missing product name")]
    MissingProductName,
}

#[derive(thiserror::Error, Debug)]
pub enum InvoiceCommandError {
    #[error("Too many parameter for {0}")]
    TooManyParameters(String),

    #[error("Wrong subcommand {0}")]
    WrongSubCommand(&'static str),

    #[error("Missing subcommand")]
    MissingSubCommand,

    #[error("Missing invoice number")]
    MissingInvoiceNumber,

    #[error("Parsing action")]
    ParsingAction(#[from] strum::ParseError),
}

#[derive(thiserror::Error, Debug)]
pub enum DomainCommandError {
    #[error("Too many parameter for {0}")]
    TooManyParameters(String),

    #[error("Wrong subcommand {0}")]
    WrongSubCommand(String),

    #[error("Missing subcommand")]
    MissingSubCommand,

    #[error("Missing domain name")]
    MissingDomainName,

    #[error("Environment: {0}")]
    Environment(#[from] VarError),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Dns: {0}")]
    Dns(#[from] DnsCommandError),

    #[error("Dns: {0}")]
    Vps(#[from] VpsCommandError),

    #[error("Product: {0}")]
    Product(#[from] ProductCommandError),

    #[error("Invoice: {0}")]
    Invoice(#[from] InvoiceCommandError),

    #[error("Invoice: {0}")]
    Domain(#[from] DomainCommandError),

    #[error("Strum: {0}")]
    Strum(#[from] ParseError),

    #[error("Parse transip command: {0}")]
    ParseTransipCommand(String),

    #[error("Parse product command: {0}")]
    ParseProductCommand(String),

    #[error("Parse domain command: {0}")]
    ParseDomainCommand(String),

    #[error("Parse dns command: {0}")]
    ParseDnsCommand(String),

    #[error("Parse vps command: {0}")]
    ParseVpsCommand(String),

    #[error("Parse invoice command: {0}")]
    ParseInvoiceCommand(String),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Variable: {0}")]
    Var(#[from] VarError),

    #[error("Parse: {0}")]
    ParseInt(#[from] ParseIntError),
}

// #[derive(Debug, Error)]
// pub enum TooManyError {
//     #[error("Too many parameters: {0}")]
//     TooMany(String),
// }

pub trait ErrorExt<T, E> {
    fn err_into(self) -> Result<T>;
}

impl<T, E> ErrorExt<T, E> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn err_into(self) -> Result<T> {
        self.map_err(Into::into)
    }
}

pub trait TooMany {
    fn too_many(self) -> std::result::Result<(), String>;
}

impl TooMany for Option<&str> {
    fn too_many(self) -> std::result::Result<(), String> {
        if let Some(value) = self {
            Err(value.to_owned())
        } else {
            Ok(())
        }
    }
}
