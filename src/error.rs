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
}

