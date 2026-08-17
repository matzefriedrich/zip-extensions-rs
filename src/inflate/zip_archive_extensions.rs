use std::path::{Path, PathBuf};
use zip::result::ZipResult;

pub trait ZipArchiveExtensions {
    /// Extracts the current archive to the given directory path.
    fn extract(&mut self, path: impl AsRef<Path>) -> ZipResult<()>;

    /// Extracts an entry in the zip archive to a file.
    fn extract_file(
        &mut self,
        file_number: usize,
        destination_file_path: impl AsRef<Path>,
        overwrite: bool,
    ) -> ZipResult<()>;

    /// Extracts an entry in the ZIP archive to the given memory buffer.
    fn extract_file_to_memory(&mut self, file_number: usize, buffer: &mut Vec<u8>)
    -> ZipResult<()>;

    /// Gets an entry´s path.
    fn entry_path(&mut self, file_number: usize) -> ZipResult<PathBuf>;

    /// Finds the index of the specified entry.
    fn file_number(&mut self, entry_path: impl AsRef<Path>) -> Option<usize>;
}
