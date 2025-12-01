use crate::default_entry_handler::DefaultEntryHandler;
use crate::entry_handler::EntryHandler;
use crate::file_utils::make_relative_path;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};

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
                file_options,
            )?;
            return Ok(());
        }

        // fallback to default behavior
        DefaultEntryHandler.handle_entry(writer, root, entry_path, file_options, buffer)
    }
}
