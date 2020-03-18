use std::io::{Read, Error, ErrorKind};
use std::io;
use zip::ZipArchive;
use std::path::PathBuf;
use zip::read::ZipFile;
use zip::result::{ZipResult, ZipError};
use crate::file_utils::file_write_all_bytes;

trait ZipArchiveExtensions {
    fn extract(&mut self, path: &PathBuf) -> ZipResult<()>;
}

impl<R: Read + io::Seek> ZipArchiveExtensions for ZipArchive<R> {
    /// Extracts the current archive to the given directory path.
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
                file_write_all_bytes(extracted_file_path, buffer.as_ref()).unwrap();
            }
        }

        Ok(())
    }
}