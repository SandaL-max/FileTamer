use crate::config::TransferOptions;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::{error, info};

/// Move or copy files to a target directory, preserving structure and handling conflicts.
///
/// - `files`: list of full file paths to move or copy
/// - `source_root`: base directory to strip when preserving structure
/// - `target_root`: destination directory
/// - `opts.copy`: if true, copy files; if false, rename/move
/// - `opts.preserve_structure`: if true, replicate directory tree under `target_root`
/// - `opts.conflict_suffix`: suffix added to filename on conflict (e.g. "_copy")
/// - `dry_run`: if true, only log actions without touching disk
pub fn move_files(
    files: &[PathBuf],
    source_root: &Path,
    target_root: &Path,
    opts: &TransferOptions,
    dry_run: bool,
) {
    for src in files {
        // Determine relative path under source_root
        let rel_path = match src.strip_prefix(source_root) {
            Ok(r) if opts.preserve_structure => r.to_path_buf(),
            _ => match src.file_name() {
                Some(name) => PathBuf::from(name),
                None => {
                    error!("Skipping invalid path: {}", src.display());
                    continue;
                }
            },
        };

        // Compute destination path
        let mut dest = target_root.join(&rel_path);

        // Handle name conflicts
        if dest.exists() {
            let stem = dest.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
            let ext = dest
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{}", e))
                .unwrap_or_default();
            let new_name = format!("{}{}{}", stem, opts.conflict_suffix, ext);
            dest = dest.with_file_name(new_name);
            info!("Conflict detected, renaming to {}", dest.display());
        }

        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            if !dry_run {
                if let Err(e) = fs::create_dir_all(parent) {
                    error!("Failed to create directory {}: {}", parent.display(), e);
                    continue;
                }
            } else {
                info!("[DRY RUN] Would create directory {}", parent.display());
            }
        }

        // Perform move or copy
        if dry_run {
            if opts.copy {
                info!(
                    "[DRY RUN] Would copy {} → {}",
                    src.display(),
                    dest.display()
                );
            } else {
                info!(
                    "[DRY RUN] Would move {} → {}",
                    src.display(),
                    dest.display()
                );
            }
        } else {
            let result = if opts.copy {
                fs::copy(src, &dest).map(|_| ())
            } else {
                fs::rename(src, &dest)
            };

            match result {
                Ok(_) => info!(
                    "{} → {}",
                    if opts.copy { "Copied" } else { "Moved" },
                    dest.display()
                ),
                Err(e) => error!(
                    "Failed to {} {}: {}",
                    if opts.copy { "copy" } else { "move" },
                    src.display(),
                    e
                ),
            }
        }
    }
}
