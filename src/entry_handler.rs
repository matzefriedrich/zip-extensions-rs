use std::io;
use std::io::Write;
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};
use zip::ZipWriter;

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
