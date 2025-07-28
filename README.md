# FileTamer

**FileTamer** is a CLI utility written in Rust for cleaning (deleting or archiving) and moving files between directories based on configurable rules.

## Features

* **Flexible filtering**: include/exclude patterns (glob), age-based (`older_than_days`), size-based (`min_size`/`max_size`).
* **Cleanup options**: delete files, archive files (ZIP or TAR+GZIP/BZIP2/XZ), keep originals.
* **Transfer options**: move or copy, preserve directory structure, handle name conflicts with custom suffix.
* **Config-driven**: YAML, TOML, or JSON configuration file with sensible defaults.
* **Dry-run mode**: preview actions without making changes.
* **Logging**: `tracing`-based logging with customizable levels via `--logging-level`.
* **Extensible**: optional features for asynchronous (Tokio) or parallel (Rayon) processing.

## Installation

Ensure you have Rust and Cargo installed (requires Rust 1.60+).

```bash
# Clone the repository
git clone https://github.com/SandaL-max/FileTamer.git
cd FileTamer

# Build in release mode
cargo build --release

# Optionally install globally
cargo install --path .
```

## Usage

```bash
# Basic move from source to target
filetamer ./source_dir ./target_dir

# Dry-run with verbose output
filetamer ./src ./dest --dry-run

# With a config file
filetamer ./src ./dest --config config.yaml
```

Run `filetamer --help` for full list of options.

## Configuration

Create a config file in YAML, TOML, or JSON. Example `config.yaml`:

```yaml
filters:
  include_patterns:
    - "**/*.log"
  exclude_patterns:
    - "**/debug/*.log"
  older_than_days: 30
  min_size: 1024

cleanup:
  delete: false
  archive: true
  archive_format: "targz"
  archive_output: "./archives/logs_$(date +%Y%m%d).tar.gz"
  keep_original: false
  compression_level: 9

transfer:
  copy: false
  preserve_structure: true
  conflict_suffix: "_backup"
```

## Project Structure

```
FileTamer/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs       # Entry point, CLI setup, and orchestration
│   ├── cli.rs        # Argument parser definitions
│   ├── config.rs     # Config structs and loading logic
│   ├── scanner.rs    # Directory traversal and filtering
│   ├── cleaner.rs    # File deletion and archiving
│   ├── mover.rs      # File moving/copying logic
│   └── logger.rs     # Tracing-based logging initialization
└── tests/            # Integration and unit tests
```

## Logging

Configure log level via `--logging-level` cli argument:

```bash
filetamer ./src ./dest --logging-level DEBUG
```

Logs will be output to STDOUT by default. You can extend `logger.rs` to add file or JSON output.

## Optional Features

Enable in `Cargo.toml`:

```toml
[features]
default = []
async = ["tokio"]
parallel = ["rayon"]
```

Then build with:

```bash
cargo build --release --features async,parallel
```

## Testing

```bash
# Run unit and integration tests
github actions or locally:
cargo test
```
