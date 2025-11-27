use crate::entry_handler::EntryHandler;
use crate::file_utils::{make_relative_path, path_as_string};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};
use zip::ZipWriter;

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
            writer.start_file(path_as_string(&relative), file_options)?;
            writer.write_all(buffer.as_ref())?;
            buffer.clear();
        } else if metadata.is_dir() {
            writer.add_directory(path_as_string(&relative), file_options)?;
        }
        Ok(())
    }
}
