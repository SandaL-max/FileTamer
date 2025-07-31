// src/main.rs

mod cleaner;
mod cli;
mod config;
mod logger;
mod mover;
mod scanner;

use cli::{Cli, Commands};
use config::Config;

use tracing::{debug, error, info};

fn main() {
    let cli = Cli::parse_args();

    // Initialize logging
    let _guard = logger::init(cli.get_logging_level());

    info!("FileTamer program started!");

    debug!("Arguments: {:#?}", cli);

    match &cli.cmd {
        Commands::Run(cmd) => cmd.run(),
        Commands::List(cmd) => cmd.run(),
    }

    info!("Operations completed.");
}
