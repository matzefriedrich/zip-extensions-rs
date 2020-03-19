use std::io::{Read, Error, ErrorKind};
use std::io;
use zip::ZipArchive;
use std::path::PathBuf;
use zip::read::ZipFile;
use zip::result::{ZipResult, ZipError};
use crate::file_utils::file_write_all_bytes;

pub trait ZipArchiveExtensions {
    /// Extracts the current archive to the given directory path.
    fn extract(&mut self, path: &PathBuf) -> ZipResult<()>;

    /// Extracts an entry in the zip archive to a file.
    fn extract_file(&mut self, file_number: usize, destination_file_path: &PathBuf, overwrite: bool) -> ZipResult<()>;

    /// Gets an entry´s path.
    fn entry_path(&mut self, file_number: usize) -> PathBuf;
}

/// ```
/// use std::fs::File;
/// use zip_extensions::read_extensions::ZipArchiveExtensions;
///
/// let file = File::open(archive_file).unwrap();
/// let mut archive = zip::ZipArchive::new(file).unwrap();
/// archive.extract(&target_path).unwrap();
/// ```
impl<R: Read + io::Seek> ZipArchiveExtensions for ZipArchive<R> {
    fn extract(&mut self, target_directory: &PathBuf) -> ZipResult<()> {
        if target_directory.is_dir() == false {
            return Err(ZipError::Io(Error::new(ErrorKind::InvalidInput, "The specified path does not indicate a valid directory path.")));
        }

        for file_number in 0..self.len() {
            let mut next: ZipFile = self.by_index(file_number).unwrap();
            let sanitized_name = next.sanitized_name();
            if next.is_dir() {
                let extracted_folder_path = target_directory.join(sanitized_name);
                std::fs::create_dir_all(extracted_folder_path).unwrap();
            } else if next.is_file() {
                let mut buffer: Vec<u8> = Vec::new();
                let _bytes_read = next.read_to_end(&mut buffer).unwrap();
                let extracted_file_path = target_directory.join(sanitized_name);
                file_write_all_bytes(extracted_file_path, buffer.as_ref(), true).unwrap();
            }
        }

        Ok(())
    }

    fn extract_file(&mut self, file_number: usize, destination_file_path: &PathBuf, overwrite: bool) -> ZipResult<()> {
        let mut next: ZipFile = self.by_index(file_number).unwrap();
        if next.is_file() {
            let mut buffer: Vec<u8> = Vec::new();
            let _bytes_read = next.read_to_end(&mut buffer).unwrap();
            file_write_all_bytes(destination_file_path.to_path_buf(), buffer.as_ref(), overwrite).unwrap();
            return Ok(());
        }
        return Err(ZipError::Io(Error::new(ErrorKind::InvalidInput, "The specified index does not indicate a file entry.")));
    }

    fn entry_path(&mut self, file_number: usize) -> PathBuf {
        let next: ZipFile = self.by_index(file_number).unwrap();
        return next.sanitized_name();
    }
}