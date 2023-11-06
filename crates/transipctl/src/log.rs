use std::path::PathBuf;

use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, filter::LevelFilter, EnvFilter, fmt::time::LocalTime};

fn log_dir() -> PathBuf {
    let local_data_dir = directories::ProjectDirs::from("nl", "paulmin", "transip")
        .unwrap()
        .data_local_dir()
        .to_path_buf();

    std::env::var("TRANSIP_API_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or(local_data_dir)
}

pub fn setup_logging() {
    LogTracer::init().unwrap();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let log_dir = log_dir();
    dbg!(&log_dir);
    std::fs::create_dir_all(&log_dir).unwrap();
    let writer = tracing_appender::rolling::daily(
        &log_dir,
        env!("CARGO_PKG_NAME"),
    );

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(writer)
        .with_timer(LocalTime::rfc_3339());

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
