use crate::file_utils::file_write_all_bytes;
use crate::inflate::zip_archive_extensions::ZipArchiveExtensions;
use std::io;
use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};

impl<R: Read + io::Seek> ZipArchiveExtensions for ZipArchive<R> {
    fn extract(&mut self, target_directory: &PathBuf) -> ZipResult<()> {
        if !target_directory.is_dir() {
            return Err(ZipError::Io(Error::new(
                ErrorKind::InvalidInput,
                "The specified path does not indicate a valid directory path.",
            )));
        }

        for file_number in 0..self.len() {
            let mut next: ZipFile<R> = self.by_index(file_number)?;
            let sanitized_name = match next.enclosed_name() {
                Some(name) => name,
                None => continue,
            };
            if next.is_dir() {
                let extracted_folder_path = target_directory.join(sanitized_name);
                std::fs::create_dir_all(extracted_folder_path)?;
            } else if next.is_file() {
                let mut buffer: Vec<u8> = Vec::new();
                let _bytes_read = next.read_to_end(&mut buffer)?;
                let extracted_file_path = target_directory.join(sanitized_name);
                file_write_all_bytes(extracted_file_path, buffer.as_ref(), true)?;
            }
        }

        Ok(())
    }

    fn extract_file(
        &mut self,
        file_number: usize,
        destination_file_path: &PathBuf,
        overwrite: bool,
    ) -> ZipResult<()> {
        let mut buffer: Vec<u8> = Vec::new();
        self.extract_file_to_memory(file_number, &mut buffer)?;
        file_write_all_bytes(
            destination_file_path.to_path_buf(),
            buffer.as_ref(),
            overwrite,
        )?;
        Ok(())
    }

    fn extract_file_to_memory(
        &mut self,
        file_number: usize,
        buffer: &mut Vec<u8>,
    ) -> ZipResult<()> {
        let mut next: ZipFile<R> = self.by_index(file_number)?;
        if next.is_file() {
            let _bytes_read = next.read_to_end(buffer)?;
            return Ok(());
        }
        Err(ZipError::Io(Error::new(
            ErrorKind::InvalidInput,
            "The specified index does not indicate a file entry.",
        )))
    }

    fn entry_path(&mut self, file_number: usize) -> ZipResult<PathBuf> {
        let next: ZipFile<R> = self.by_index(file_number)?;
        Ok(next.mangled_name())
    }

    fn file_number(&mut self, entry_path: &PathBuf) -> Option<usize> {
        for file_number in 0..self.len() {
            if let Ok(next) = self.by_index(file_number) {
                let sanitized_name = next.mangled_name();
                if sanitized_name == *entry_path {
                    return Some(file_number);
                }
            }
        }
        None
    }
}
