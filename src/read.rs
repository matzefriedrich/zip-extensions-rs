use std::io::{Read, Error, ErrorKind};
use std::io;
use zip::ZipArchive;
use std::path::PathBuf;
use zip::read::ZipFile;
use zip::result::{ZipResult, ZipError};
use crate::file_utils::file_write_all_bytes;
use std::fs::File;

/// Extracts a ZIP file to the given directory.
pub fn zip_extract(archive_file: &PathBuf, target_dir: &PathBuf) -> ZipResult<()> {
    let file = File::open(archive_file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive.extract(target_dir)
}

/// Extracts and entry in the ZIP archive to the given directory.
pub fn zip_extract_file(archive_file: &PathBuf, entry_path: &PathBuf, target_dir: &PathBuf, overwrite: bool) -> ZipResult<()> {
    let file = File::open(archive_file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let file_number: usize = match archive.file_number(entry_path){
        Some(index) => index,
        None => return Err(ZipError::FileNotFound)
    };
    let destination_file_path = target_dir.join(entry_path);
    archive.extract_file(file_number, &destination_file_path, overwrite)
}

/// Extracts an entry in the ZIP archive to the given memory buffer.
pub fn zip_extract_file_to_memory(archive_file: &PathBuf, entry_path: &PathBuf, buffer: &mut Vec<u8>) -> ZipResult<()> {
    let file = File::open(archive_file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let file_number: usize = match archive.file_number(entry_path){
        Some(index) => index,
        None => return Err(ZipError::FileNotFound)
    };
    return archive.extract_file_to_memory(file_number, buffer);
}

/// Determines whether the specified file is a ZIP file, or not.
pub fn is_zip(file: &PathBuf) -> bool {
    const ZIP_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];
    let mut file = File::open(file).unwrap();
    let mut buffer: [u8; 4] = [0; 4];
    let bytes_read = file.read(&mut buffer).unwrap();
    if bytes_read == buffer.len() && bytes_read == ZIP_SIGNATURE.len() {
        for i in 0..ZIP_SIGNATURE.len() {
            if buffer[i] != ZIP_SIGNATURE[i] {
                return false;
            }
        }
        return true;
    }
    false
}

pub trait ZipArchiveExtensions {
    /// Extracts the current archive to the given directory path.
    fn extract(&mut self, path: &PathBuf) -> ZipResult<()>;

    /// Extracts an entry in the zip archive to a file.
    fn extract_file(&mut self, file_number: usize, destination_file_path: &PathBuf, overwrite: bool) -> ZipResult<()>;

    /// Extracts an entry in the ZIP archive to the given memory buffer.
    fn extract_file_to_memory(&mut self, file_number: usize, buffer: &mut Vec<u8>) -> ZipResult<()>;

    /// Gets an entry´s path.
    fn entry_path(&mut self, file_number: usize) -> PathBuf;

    /// Finds the index of the specified entry.
    fn file_number(&mut self, entry_path: &PathBuf) -> Option<usize>;
}

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
        let mut buffer: Vec<u8> = Vec::new();
        self.extract_file_to_memory(file_number, &mut buffer).unwrap();
        file_write_all_bytes(destination_file_path.to_path_buf(), buffer.as_ref(), overwrite).unwrap();
        return Ok(());
    }

    fn extract_file_to_memory(&mut self, file_number: usize, buffer: &mut Vec<u8>) -> ZipResult<()> {
        let mut next: ZipFile = self.by_index(file_number).unwrap();
        if next.is_file() {
            let _bytes_read = next.read_to_end(buffer).unwrap();
            return Ok(());
        }
        return Err(ZipError::Io(Error::new(ErrorKind::InvalidInput, "The specified index does not indicate a file entry.")));
    }

    fn entry_path(&mut self, file_number: usize) -> PathBuf {
        let next: ZipFile = self.by_index(file_number).unwrap();
        return next.sanitized_name();
    }

    fn file_number(&mut self, entry_path: &PathBuf) -> Option<usize> {
        for file_number in 0..self.len() {
            let next: ZipFile = self.by_index(file_number).unwrap();
            let sanitized_name = next.sanitized_name();
            if sanitized_name == *entry_path{
                return Some(file_number);
            }
        }
        None
    }
}