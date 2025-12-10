use crate::default_entry_handler::DefaultEntryHandler;
use crate::entry_handler::EntryHandler;
use crate::file_utils::make_relative_path;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};

/// An `EntryHandler` wrapper that preserves symlinks when zipping.
///
/// If the current entry is a symbolic link, it writes a symlink entry to the ZIP. Otherwise, it
/// delegates to the wrapped `inner` handler. By default, the inner handler
/// is `DefaultEntryHandler`, but you can compose multiple behaviors by wrapping another handler
/// instead.
pub struct PreserveSymlinksHandler<H = DefaultEntryHandler> {
    inner: H,
}

impl PreserveSymlinksHandler<DefaultEntryHandler> {
    pub fn new() -> Self {
        Self {
            inner: DefaultEntryHandler,
        }
    }

    pub fn with_default_inner() -> Self {
        Self::new()
    }
}

impl<H> PreserveSymlinksHandler<H> {
    pub fn with_inner(inner: H) -> Self {
        Self { inner }
    }
}

impl<T: FileOptionExtension, H> EntryHandler<T> for PreserveSymlinksHandler<H>
where
    H: EntryHandler<T>,
{
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

        self.inner
            .handle_entry(writer, root, entry_path, file_options, buffer)
    }
}
