// src/logger.rs

use tracing::Level;
use tracing_subscriber;

/// Initializes the logging subsystem using `tracing`.
/// The log level can be set via the input variable
pub fn init(logging_level: &Level) {
    tracing_subscriber::fmt()
        .with_max_level(*logging_level)
        .init();
}
