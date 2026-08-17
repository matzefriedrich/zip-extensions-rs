use crate::inflate::zip_archive_extensions::ZipArchiveExtensions;
use std::fs::File;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use zip::result::{ZipError, ZipResult};

/// Extracts a ZIP file to the given directory.
pub fn zip_extract(archive_file: impl AsRef<Path>, target_dir: impl AsRef<Path>) -> ZipResult<()> {
    let file = File::open(archive_file)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(target_dir)
}

/// Extracts and entry in the ZIP archive to the given directory.
pub fn zip_extract_file(
    archive_file: impl AsRef<Path>,
    entry_path: impl AsRef<Path>,
    target_dir: impl AsRef<Path>,
    overwrite: bool,
) -> ZipResult<()> {
    let archive_file = archive_file.as_ref();
    let entry_path = entry_path.as_ref();
    let target_dir = target_dir.as_ref();

    let file = File::open(archive_file)?;
    let mut archive = ZipArchive::new(file)?;
    let file_number: usize = match archive.file_number(entry_path) {
        Some(index) => index,
        None => return Err(ZipError::FileNotFound),
    };
    let destination_file_path = target_dir.join(entry_path);
    archive.extract_file(file_number, &destination_file_path, overwrite)
}

/// Extracts an entry in the ZIP archive to the given memory buffer.
pub fn zip_extract_file_to_memory(
    archive_file: impl AsRef<Path>,
    entry_path: impl AsRef<Path>,
    buffer: &mut Vec<u8>,
) -> ZipResult<()> {
    let file = File::open(archive_file)?;
    let mut archive = ZipArchive::new(file)?;
    let file_number: usize = match archive.file_number(entry_path) {
        Some(index) => index,
        None => return Err(ZipError::FileNotFound),
    };
    archive.extract_file_to_memory(file_number, buffer)
}
