use crate::entry_handler::EntryHandler;
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};

pub trait ZipWriterExtensions {
    /// Creates a zip archive that contains the files and directories from the specified directory.
    fn create_from_directory(&mut self, directory: &PathBuf) -> ZipResult<()>;

    /// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
    fn create_from_directory_with_options<F, T, H>(
        &mut self,
        directory: &PathBuf,
        cb_file_options: F,
        handler: &H,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>,
        H: EntryHandler<T>;
}
