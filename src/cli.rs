use clap::Parser;
use std::path::PathBuf;
use tracing::Level;

/// Utility for cleaning and moving files
#[derive(Debug, Parser)]
#[command(
    name = "FileTamer",
    version,
    about = "CLI utility for cleaning and moving files"
)]
pub struct Args {
    /// Source folder
    #[arg(value_name = "SOURCE", help = "Path to source directory")]
    source: PathBuf,

    /// Destination folder
    #[arg(value_name = "TARGET", help = "Path to target directory")]
    target: PathBuf,

    /// Path to configuration file (YAML/TOML/JSON)
    #[arg(short, long, value_name = "CONFIG", help = "Path to config file")]
    config: Option<PathBuf>,

    /// Show actions without execution
    #[arg(long, help = "Perform a dry run (show what would be done)")]
    dry_run: bool,

    /// Logging level
    #[arg(short, long, help = "Level for programm logging", default_value_t = Level::INFO)]
    logging_level: Level,
}

impl Args {
    pub fn get_logging_level(&self) -> &Level {
        &self.logging_level
    }
    pub fn get_config(&self) -> &Option<PathBuf> {
        &self.config
    }
}

pub fn parse() -> Args {
    Args::parse()
}
