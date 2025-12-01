use crate::ZipWriterExtensions;
use crate::preserve_symlinks_handler::PreserveSymlinksHandler;
use std::fs::File;
use std::path::PathBuf;
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};

/// Creates a ZIP archive from a directory while preserving symbolic links.
///
/// ### Security Warning
///
/// Preserving symlinks inside ZIP archives can introduce security risks
/// for consumers who extract the archive using tools or libraries that
/// do **not** validate paths or symlink targets.
///
/// Malicious archives may embed symlinks whose resolved target points
/// outside the intended extraction directory, potentially enabling
/// path traversal or overwriting arbitrary files (“Zip Slip”–style
/// issues).
///
/// This library's own extraction implementation protects against such
/// attacks by sanitizing all paths with `ZipFile::enclosed_name`, and
/// only creating symlinks whose canonicalized target remains inside the
/// extraction root.
///
/// **However, these guarantees do not apply to other extractors.**
/// Developers distributing archives to unknown consumers should prefer
/// [`zip_create_from_directory_with_options`], which resolves symlinks
/// to their pointed-to content instead of preserving them as links.
///
/// ### Recommended use
///
/// - Use this function **only** when you control the extraction
///   environment and know that it properly validates paths and symlinks.
/// - For general distribution or untrusted extraction environments,
///   use `zip_create_from_directory_with_options` instead.
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
