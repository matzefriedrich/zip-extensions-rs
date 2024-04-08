use crate::file_utils::{make_relative_path, path_as_string};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::{write, CompressionMethod, ZipWriter};

/// Creates a zip archive that contains the files and directories from the specified directory.
pub fn zip_create_from_directory(archive_file: &PathBuf, directory: &PathBuf) -> ZipResult<()> {
    let options = write::FileOptions::default().compression_method(CompressionMethod::Stored);
    zip_create_from_directory_with_options(archive_file, directory, |_| options)
}

/// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
pub fn zip_create_from_directory_with_options<F>(
    archive_file: &PathBuf,
    directory: &PathBuf,
    options_map: F,
) -> ZipResult<()>
where
    F: Fn(&PathBuf) -> FileOptions,
{
    let file = File::create(archive_file)?;
    let mut zip_writer = zip::ZipWriter::new(file);
    zip_writer.create_from_directory_with_options(directory, options_map)
}

pub trait ZipWriterExtensions {
    /// Creates a zip archive that contains the files and directories from the specified directory.
    fn create_from_directory(&mut self, directory: &PathBuf) -> ZipResult<()>;

    /// Creates a zip archive that contains the files and directories from the specified directory, uses the specified compression level.
    fn create_from_directory_with_options<F>(
        &mut self,
        directory: &PathBuf,
        options_map: F,
    ) -> ZipResult<()>
    where
        F: Fn(&PathBuf) -> FileOptions;
}

impl<W: Write + io::Seek> ZipWriterExtensions for ZipWriter<W> {
    fn create_from_directory(&mut self, directory: &PathBuf) -> ZipResult<()> {
        let options = write::FileOptions::default().compression_method(CompressionMethod::Stored);
        self.create_from_directory_with_options(directory, |_| options)
    }

    fn create_from_directory_with_options<F>(
        &mut self,
        directory: &PathBuf,
        options_map: F,
    ) -> ZipResult<()>
    where
        F: Fn(&PathBuf) -> FileOptions,
    {
        let mut paths_queue: Vec<PathBuf> = vec![];
        paths_queue.push(directory.clone());

        let mut buffer = Vec::new();

        while let Some(next) = paths_queue.pop() {
            let directory_entry_iterator = std::fs::read_dir(next)?;

            for entry in directory_entry_iterator {
                let entry_path = entry?.path();
                let file_options = options_map(&entry_path);
                let entry_metadata = std::fs::metadata(entry_path.clone())?;
                if entry_metadata.is_file() {
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
