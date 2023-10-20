#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Parse product command: {0}")]
    ParseProductCommand(String),

    #[error("Parse dns command: {0}")]
    ParseDnsCommand(String),

    #[error("Parse vps command: {0}")]
    ParseVpsCommand(String),
}

