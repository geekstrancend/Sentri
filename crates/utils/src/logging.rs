//! Logging and tracing setup.

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize structured logging with sensible defaults.
///
/// # Arguments
///
/// * `level` - Log level: "trace", "debug", "info", "warn", "error"
pub fn setup_tracing(level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level))
        .add_directive("hyper=info".parse().unwrap_or_default());

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(filter)
        .init();
}
