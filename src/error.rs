use std::env::VarError;

use strum::ParseError;

use crate::{Result, Rule};

#[derive(thiserror::Error, Debug)]
pub enum Error {
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

    #[error("Api: {0}")]
    Api(#[from] transip::Error),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Parsing: {0}")]
    Pest(#[from] Box<pest::error::Error<Rule>>),

    #[error("Variable: {0}")]
    Var(#[from] VarError),
}

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
