use crate::Result;
use std::env::VarError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
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

    #[error("Transip Api: {0}")]
    TransipApi(#[from] transip_command_execute::Error),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Variable: {0}")]
    Var(#[from] VarError),

    #[error("Command: {0}")]
    Command(#[from] transip_command::Error),
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
