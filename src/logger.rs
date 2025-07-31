// src/logger.rs

use std::fs::File;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber;

/// Initializes the logging subsystem using `tracing`.
/// The log level can be set via the input variable
pub fn init(logging_level: &Level) -> WorkerGuard {
    let log_file = File::create("./logs/app.log").unwrap();

    let (writer, guard) = tracing_appender::non_blocking(log_file);
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(writer)
        .with_max_level(*logging_level)
        .init();

    guard
}
