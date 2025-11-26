use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions, SimpleFileOptions};
use zip::{CompressionMethod, ZipWriter};

use crate::file_utils::{make_relative_path, path_as_string};

/// Creates a zip archive that contains the files and directories from the specified directory.
pub fn zip_create_from_directory(archive_file: &PathBuf, directory: &PathBuf) -> ZipResult<()> {
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip_create_from_directory_with_options(archive_file, directory, |_| options)
}

/// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
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

/// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
pub fn zip_create_from_directory_preserve_symlinks_with_options<F, T>(
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
    zip_writer.create_from_directory_with_options(
        directory,
        cb_file_options,
        &PreserveSymlinksHandler,
    )
}

pub trait ZipWriterExtensions {
    /// Creates a zip archive that contains the files and directories from the specified directory.
    fn create_from_directory(self, directory: &PathBuf) -> ZipResult<()>;

    /// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
    fn create_from_directory_with_options<F, T, H>(
        self,
        directory: &PathBuf,
        cb_file_options: F,
        handler: &H,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>,
        H: EntryHandler<T>;
}

pub trait EntryHandler<T: FileOptionExtension> {
    fn handle_entry<W: Write + io::Seek>(
        &self,
        writer: &mut ZipWriter<W>,
        root: &PathBuf,
        entry_path: &PathBuf,
        file_options: FileOptions<T>,
        buffer: &mut Vec<u8>,
    ) -> ZipResult<()>;
}

pub struct DefaultEntryHandler;

impl<T: FileOptionExtension> EntryHandler<T> for DefaultEntryHandler {
    fn handle_entry<W: Write + io::Seek>(
        &self,
        writer: &mut ZipWriter<W>,
        root: &PathBuf,
        entry_path: &PathBuf,
        file_options: FileOptions<T>,
        buffer: &mut Vec<u8>,
    ) -> ZipResult<()> {
        let metadata = std::fs::metadata(entry_path)?;
        let relative = make_relative_path(root, entry_path);

        if metadata.is_file() {
            let mut f = File::open(&entry_path)?;
            f.read_to_end(buffer)?;
            let relative_path = make_relative_path(&relative, &entry_path);
            writer.start_file(path_as_string(&relative_path), file_options)?;
            writer.write_all(buffer.as_ref())?;
            buffer.clear();
        } else if metadata.is_dir() {
            writer.add_directory(path_as_string(&relative), file_options)?;
        }
        Ok(())
    }
}

pub struct PreserveSymlinksHandler;

impl<T: FileOptionExtension> EntryHandler<T> for PreserveSymlinksHandler {
    fn handle_entry<W: Write + io::Seek>(
        &self,
        writer: &mut ZipWriter<W>,
        root: &PathBuf,
        entry_path: &PathBuf,
        file_options: FileOptions<T>,
        buffer: &mut Vec<u8>,
    ) -> ZipResult<()> {
        let symlink_metadata = std::fs::symlink_metadata(entry_path)?;
        let relative = make_relative_path(root, entry_path);

        if symlink_metadata.is_symlink() {
            let target = std::fs::read_link(entry_path)?;
            writer.add_symlink(
                relative.to_str().unwrap(),
                target.to_str().unwrap(),
                SimpleFileOptions::default(),
            )?;
            return Ok(());
        }

        // fallback to default behavior
        DefaultEntryHandler.handle_entry(writer, root, entry_path, file_options, buffer)
    }
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
