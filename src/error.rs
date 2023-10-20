use crate::Result;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parse product command: {0}")]
    ParseProductCommand(String),

    #[error("Parse dns command: {0}")]
    ParseDnsCommand(String),

    #[error("Parse vps command: {0}")]
    ParseVpsCommand(String),

    #[error("Api: {0}")]
    Api(#[from] transip::Error),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
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
