use crate::config::CleanupOptions;
use flate2::{Compression, write::GzEncoder};
use std::{
    fs,
    fs::File,
    io,
    path::{Path, PathBuf},
};
use tar::Builder;
use tracing::{error, info};
use zip::{
    CompressionMethod, ZipWriter,
    write::{ExtendedFileOptions, FileOptions},
};

/// Main entry point for cleaning files: delete or archive based on options
pub fn clean(files: &[PathBuf], opts: &CleanupOptions, dry_run: bool) {
    if opts.archive {
        if dry_run {
            info!(
                "[DRY RUN] Would archive {} files to {:?}",
                files.len(),
                opts.archive_output
            );
        } else if let Err(e) = archive_files(files, opts) {
            error!("Archive error: {}", e);
        }
    } else if opts.delete {
        for path in files {
            delete_file(path, dry_run);
        }
    } else {
        // No cleanup action specified
        for path in files {
            info!("No cleanup action for {}", path.display());
        }
    }
}

fn delete_file(path: &PathBuf, dry_run: bool) {
    if dry_run {
        info!("[DRY RUN] Would delete {}", path.display());
    } else {
        match fs::remove_file(path) {
            Ok(_) => info!("Deleted {}", path.display()),
            Err(e) => error!("Failed to delete {}: {}", path.display(), e),
        }
    }
}

fn archive_files(files: &[PathBuf], opts: &CleanupOptions) -> io::Result<()> {
    let out_path = opts
        .archive_output
        .as_ref()
        .map(PathBuf::as_path)
        .unwrap_or(Path::new("archive.zip"));
    match opts.archive_format.as_str() {
        "zip" => archive_zip(files, opts, out_path),
        "targz" => archive_targz(files, opts, out_path),
        "tar" => archive_tar(files, out_path),
        other => {
            error!("Unsupported archive format: {}", other);
            Ok(())
        }
    }
}

fn archive_zip(files: &[PathBuf], opts: &CleanupOptions, out_path: &Path) -> io::Result<()> {
    let f = File::create(out_path)?;
    let mut zip = ZipWriter::new(f);
    let options: FileOptions<'_, ExtendedFileOptions> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(opts.compression_level as i64));

    for path in files {
        let name = path.file_name().unwrap().to_string_lossy();
        let options_clone = options.clone();
        zip.start_file(name.as_ref(), options_clone)?;
        let mut src = File::open(path)?;
        io::copy(&mut src, &mut zip)?;
        if !opts.keep_original {
            fs::remove_file(path).ok();
        }
        info!("Archived {} into {:?}", path.display(), out_path);
    }
    zip.finish()?;
    Ok(())
}

fn archive_tar(files: &[PathBuf], out_path: &Path) -> io::Result<()> {
    let f = File::create(out_path)?;
    let mut tar = Builder::new(f);
    for path in files {
        let prefix = "/home/pluto/Downloads";
        tar.append_path_with_name(path, path.strip_prefix(prefix).unwrap())?;
        info!("Added {} to TAR {:?}", path.display(), out_path);
    }
    tar.finish()?;
    Ok(())
}

fn archive_targz(files: &[PathBuf], opts: &CleanupOptions, out_path: &Path) -> io::Result<()> {
    let f = File::create(out_path)?;
    let enc = GzEncoder::new(f, Compression::new(opts.compression_level));
    let mut tar = Builder::new(enc);
    for path in files {
        let prefix = "/home/pluto/Downloads";
        tar.append_path_with_name(path, path.strip_prefix(prefix).unwrap())?;
        if !opts.keep_original {
            fs::remove_file(path).ok();
        }
        info!("Added {} to TAR GZ {:?}", path.display(), out_path);
    }
    tar.finish()?;
    Ok(())
}
