// src/main.rs

mod cleaner;
mod cli;
mod config;
mod logger;
mod mover;
mod scanner;

use config::Config;

use tracing::{debug, error, info};

fn main() {
    let args = cli::parse();

    // Initialize logging
    logger::init(args.get_logging_level());

    info!("FileTamer program started!");

    debug!("Arguments: {:#?}", args);

    // TODO: Load configuration

    let config = match args.get_config() {
        Some(path) => Config::from_file(path),
        None => Ok(Config::default()),
    };
    if let Err(e) = &config {
        error!("Error loading config: {}", e);
        std::process::exit(1);
    }
    let config = config.unwrap();

    debug!("Config: {:#?}", config);

    // TODO: Scan source folder
    // TODO: Cleanup (delete or archive)
    // TODO: Move files

    info!("Operations completed.");
}
