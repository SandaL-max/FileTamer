// src/scanner.rs
use crate::config::Filters;
use globset::{GlobBuilder, GlobSetBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan(source: &Path, filters: &Filters) -> Vec<PathBuf> {
    // Collecting include/exclude sets
    let mut inc_builder = GlobSetBuilder::new();
    for pat in &filters.include_patterns {
        inc_builder.add(GlobBuilder::new(pat).build().unwrap());
    }
    let inc_set = inc_builder.build().unwrap();

    let mut exc_builder = GlobSetBuilder::new();
    for pat in &filters.exclude_patterns {
        exc_builder.add(GlobBuilder::new(pat).build().unwrap());
    }
    let exc_set = exc_builder.build().unwrap();

    let mut results = Vec::new();
    for entry in WalkDir::new(source).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let rel = path.strip_prefix(source).unwrap();
            // check include + exclude
            if inc_set.is_match(rel) && !exc_set.is_match(rel) {
                // TODO: here you can also filter by date and by size
                results.push(path.to_path_buf());
            }
        }
    }
    results
}
