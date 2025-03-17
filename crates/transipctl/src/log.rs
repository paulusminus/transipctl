use std::path::PathBuf;
use tracing_log::LogTracer;
use tracing_subscriber::{
    EnvFilter, filter::LevelFilter, fmt::time::LocalTime,
    prelude::__tracing_subscriber_SubscriberExt,
};

use crate::error::{Error, ErrorExt};

const QUALIFIER: &str = "nl";
const ORGANISATION: &str = "paulmin";
const APPLICATION: &str = "transip";

pub fn log_dir() -> PathBuf {
    let local_data_dir = directories::ProjectDirs::from(QUALIFIER, ORGANISATION, APPLICATION)
        .map(|project_dirs| project_dirs.data_local_dir().to_path_buf())
        .ok_or(Error::Xdg("XDG not found"));

    std::env::var("TRANSIP_API_LOG_DIR")
        .err_into()
        .map_or_else(|_| local_data_dir.unwrap(), PathBuf::from)
}

pub fn setup_logging() {
    LogTracer::init().unwrap();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let log_dir = log_dir();
    std::fs::create_dir_all(&log_dir).unwrap();
    let writer = tracing_appender::rolling::daily(&log_dir, env!("CARGO_PKG_NAME"));

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(writer)
        .with_timer(LocalTime::rfc_3339());

    let subscriber = tracing_subscriber::registry().with(env_filter).with(layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
