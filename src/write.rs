use std::fs::{self, File};
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
    zip_writer.create_from_directory_with_options(directory, cb_file_options)
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
    zip_writer.create_from_directory_preserve_symlinks_with_options(
        directory,
        true,
        cb_file_options,
    )
}

pub trait ZipWriterExtensions {
    /// Creates a zip archive that contains the files and directories from the specified directory.
    fn create_from_directory(self, directory: &PathBuf) -> ZipResult<()>;

    /// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
    fn create_from_directory_with_options<F, T>(
        self,
        directory: &PathBuf,
        cb_file_options: F,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>;

    /// Creates a zip archive that contains the files, symlinks and directories from the specified directory, uses the specified compression level.
    fn create_from_directory_preserve_symlinks_with_options<F, T>(
        self,
        directory: &PathBuf,
        preserve_symlink: bool,
        cb_file_options: F,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>;
}

impl<W: Write + io::Seek> ZipWriterExtensions for ZipWriter<W> {
    fn create_from_directory(self, directory: &PathBuf) -> ZipResult<()> {
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        self.create_from_directory_with_options(directory, |_| options)
    }

    fn create_from_directory_with_options<F, T>(
        self,
        directory: &PathBuf,
        cb_file_options: F,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>,
    {
        return self.create_from_directory_preserve_symlinks_with_options(
            directory,
            false,
            cb_file_options,
        );
    }

    fn create_from_directory_preserve_symlinks_with_options<F, T>(
        mut self,
        directory: &PathBuf,
        preserve_symlink: bool,
        cb_file_options: F,
    ) -> ZipResult<()>
    where
        T: FileOptionExtension,
        F: Fn(&PathBuf) -> FileOptions<T>,
    {
        let mut paths_queue: Vec<PathBuf> = vec![];
        paths_queue.push(directory.clone());

        let mut buffer = Vec::new();

        while let Some(next) = paths_queue.pop() {
            let directory_entry_iterator = std::fs::read_dir(next)?;

            for entry in directory_entry_iterator {
                let entry_path = entry?.path();
                let file_options = cb_file_options(&entry_path);
                let entry_metadata = std::fs::metadata(entry_path.clone())?;
                let symlink_metadata = std::fs::symlink_metadata(entry_path.clone())?;
                if preserve_symlink && symlink_metadata.is_symlink() {
                    let target = fs::read_link(&entry_path)?;
                    let relative_path = make_relative_path(&directory, &entry_path);

                    self.add_symlink(
                        relative_path.to_str().unwrap(),
                        target.to_str().unwrap(),
                        SimpleFileOptions::default(),
                    )?;
                } else if entry_metadata.is_file() {
                    let mut f = File::open(&entry_path)?;
                    f.read_to_end(&mut buffer)?;
                    let relative_path = make_relative_path(&directory, &entry_path);
                    self.start_file(path_as_string(&relative_path), file_options)?;
                    self.write_all(buffer.as_ref())?;
                    buffer.clear();
                } else if entry_metadata.is_dir() {
                    let relative_path = make_relative_path(&directory, &entry_path);
                    self.add_directory(path_as_string(&relative_path), file_options)?;
                    paths_queue.push(entry_path.clone());
                }
            }
        }

        self.finish()?;
        Ok(())
    }
}
