use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};

pub trait EntryHandler<T: FileOptionExtension> {
    fn handle_entry<W: Write + io::Seek>(
        &self,
        writer: &mut ZipWriter<W>,
        root: impl AsRef<Path>,
        entry_path: impl AsRef<Path>,
        file_options: FileOptions<T>,
        buffer: &mut Vec<u8>,
    ) -> ZipResult<()>;
}
