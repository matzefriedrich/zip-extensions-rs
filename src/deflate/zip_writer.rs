use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::default_entry_handler::DefaultEntryHandler;
use crate::deflate::zip_writer_extensions::ZipWriterExtensions;
use crate::entry_handler::EntryHandler;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions, SimpleFileOptions};
use zip::{CompressionMethod, ZipWriter};

/// Creates a zip archive that contains the files and directories from the specified directory.
pub fn zip_create_from_directory(archive_file: &PathBuf, directory: &PathBuf) -> ZipResult<()> {
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip_create_from_directory_with_options(archive_file, directory, |_| options)
}

/// Creates a zip archive that contains the files and directories from the specified directory.
pub fn zip_create_from_directory_with_options<F, T>(
    archive_file: &PathBuf,
    directory: &PathBuf,
    cb_file_options: F,
) -> ZipResult<()>
where
    T: FileOptionExtension,
    F: Fn(&PathBuf) -> FileOptions<T>,
{
    let file = File::create(archive_file)?;
    let zip_writer = ZipWriter::new(file);
    zip_writer.create_from_directory_with_options(directory, cb_file_options, &DefaultEntryHandler)
}

impl<W: Write + io::Seek> ZipWriterExtensions for ZipWriter<W> {
    fn create_from_directory(self, directory: &PathBuf) -> ZipResult<()> {
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        self.create_from_directory_with_options(directory, |_| options, &DefaultEntryHandler)
    }

    fn create_from_directory_with_options<F, T, H>(
        mut self,
        directory: &PathBuf,
        cb_file_options: F,
        handler: &H,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>,
        H: EntryHandler<T>,
    {
        let mut paths_queue: Vec<PathBuf> = vec![];
        paths_queue.push(directory.clone());

        let mut buffer = Vec::new();

        while let Some(next) = paths_queue.pop() {
            let directory_entry_iterator = std::fs::read_dir(next)?;
            for entry in directory_entry_iterator {
                let entry_path = entry?.path();
                let file_options = cb_file_options(&entry_path);
                handler.handle_entry(
                    &mut self,
                    &directory,
                    &entry_path,
                    file_options,
                    &mut buffer,
                )?;
                let entry_metadata = std::fs::metadata(entry_path.clone())?;
                if entry_metadata.is_dir() {
                    paths_queue.push(entry_path.clone());
                }
            }
        }

        self.finish()?;
        Ok(())
    }
}
