use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Main utility configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// File filtering rules before operations
    #[serde(default)]
    pub filters: Filters,

    /// Cleanup options (delete or archive)
    #[serde(default)]
    pub cleanup: CleanupOptions,

    /// Transfer options (copy or move)
    #[serde(default)]
    pub transfer: TransferOptions,
}

/// File filtering rules
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Filters {
    /// List of glob patterns to include
    pub include_patterns: Vec<String>,
    /// List of glob patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Filtering by age (days)
    pub older_than_days: Option<u64>,
    /// Minimum file size in bytes
    pub min_size: Option<u64>,
    /// Maximum file size in bytes
    pub max_size: Option<u64>,
}

/// File cleaning options
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CleanupOptions {
    /// Delete files instead of archiving
    pub delete: bool,
    /// Archive files instead of deleting
    pub archive: bool,
    /// Archive format: zip, targz, tarbz2, tarxz
    pub archive_format: String,
    /// Path to output archive
    pub archive_output: Option<PathBuf>,
    /// Do not delete original files after archiving
    pub keep_original: bool,
    /// Compression level (0-9)
    pub compression_level: u32,
}

/// File transfer options
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct TransferOptions {
    /// Use copy instead of move
    pub copy: bool,
    /// Preserve directory structure
    pub preserve_structure: bool,
    /// Suffix for conflicting file names
    pub conflict_suffix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filters: Filters::default(),
            cleanup: CleanupOptions::default(),
            transfer: TransferOptions::default(),
        }
    }
}

impl Default for Filters {
    fn default() -> Self {
        Filters {
            include_patterns: vec!["**/*".into()],
            exclude_patterns: Vec::new(),
            older_than_days: None,
            min_size: None,
            max_size: None,
        }
    }
}

impl Default for CleanupOptions {
    fn default() -> Self {
        CleanupOptions {
            delete: false,
            archive: false,
            archive_format: "zip".into(),
            archive_output: None,
            keep_original: false,
            compression_level: 6,
        }
    }
}

impl Default for TransferOptions {
    fn default() -> Self {
        TransferOptions {
            copy: false,
            preserve_structure: true,
            conflict_suffix: "_copy".into(),
        }
    }
}

impl Config {
    /// Loads configuration from a YAML, TOML or JSON file by extension
    pub fn from_file(path: &Path) -> Result<Self> {
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        let cfg = match ext {
            "yaml" | "yml" => serde_yaml::from_str(&data).context("Error parsing YAML config")?,
            "toml" => toml::from_str(&data).context("Error parsing TOML config")?,
            "json" => serde_json::from_str(&data).context("Error parsing JSON config")?,
            _ => anyhow::bail!("Unknown config file format: {}", ext),
        };
        Ok(cfg)
    }
}
