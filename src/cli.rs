use crate::config::Config;
use crate::scanner;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use tracing::Level;

use tracing::{debug, error, info};

/// Utility for cleaning and moving files
#[derive(Debug, Parser)]
#[command(
    name = "FileTamer",
    version,
    about = "CLI utility for cleaning and moving files"
)]
pub struct Cli {
    /// Logging level
    #[arg(short, long, help = "Level for programm logging", default_value_t = Level::DEBUG)]
    logging_level: Level,

    #[command(subcommand)]
    pub cmd: Commands,
}

impl Cli {
    pub fn get_logging_level(&self) -> &Level {
        &self.logging_level
    }
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run(RunCmd),
    List(ListCmd),
}

pub trait HasConfig {
    fn config_path(&self) -> Option<&PathBuf>;
}

#[derive(Args, Debug)]
pub struct RunCmd {
    /// Source folder
    #[arg(value_name = "SOURCE", help = "Path to source directory")]
    source: PathBuf,

    /// Destination folder
    #[arg(value_name = "TARGET", help = "Path to target directory")]
    target: PathBuf,

    /// Path to configuration file (YAML/TOML/JSON)
    #[arg(short, long, value_name = "CONFIG", help = "Path to config file")]
    pub config: Option<PathBuf>,

    /// Show actions without execution
    #[arg(long, help = "Perform a dry run (show what would be done)")]
    dry_run: bool,
}

impl RunCmd {
    pub fn run(&self) {
        let config = get_config(self);

        debug!("Config: {:#?}", config);

        // TODO: Scan source folder
        // TODO: Cleanup (delete or archive)
        // TODO: Move files
    }
}

impl HasConfig for RunCmd {
    fn config_path(&self) -> Option<&PathBuf> {
        self.config.as_ref()
    }
}

#[derive(Args, Debug)]
pub struct ListCmd {
    /// Source folder
    #[arg(value_name = "SOURCE", help = "Path to source directory")]
    pub source: PathBuf,

    /// Path to configuration file (YAML/TOML/JSON)
    #[arg(short, long, value_name = "CONFIG", help = "Path to config file")]
    pub config: Option<PathBuf>,
}

impl ListCmd {
    pub fn run(&self) {
        let config = get_config(self);
        let files = scanner::scan(&self.source, &config.filters);

        // Просто печатаем пути
        for path in files {
            info!("{}", path.display());
        }
    }
}

impl HasConfig for ListCmd {
    fn config_path(&self) -> Option<&PathBuf> {
        self.config.as_ref()
    }
}

fn get_config<T: HasConfig>(cmd: &T) -> Config {
    let config = match cmd.config_path() {
        Some(path) => Config::from_file(path),
        None => Ok(Config::default()),
    };
    if let Err(e) = &config {
        error!("Error loading config: {}", e);
        std::process::exit(1);
    }
    config.unwrap()
}
