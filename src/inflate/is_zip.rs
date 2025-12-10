use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zip::result::ZipResult;

/// Determines whether the specified file is a ZIP file, or not.
pub fn try_is_zip(file: &PathBuf) -> ZipResult<bool> {
    const ZIP_SIGNATURE: [u8; 2] = [0x50, 0x4b];
    const ZIP_ARCHIVE_FORMAT: [u8; 6] = [0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let mut file = File::open(file)?;
    let mut buffer: [u8; 4] = [0; 4];
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == buffer.len() {
        for i in 0..ZIP_SIGNATURE.len() {
            if buffer[i] != ZIP_SIGNATURE[i] {
                return Ok(false);
            }
        }

        for i in (0..ZIP_ARCHIVE_FORMAT.len()).step_by(2) {
            if buffer[2] == ZIP_ARCHIVE_FORMAT[i] || buffer[3] == ZIP_ARCHIVE_FORMAT[i + 1] {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Determines whether the specified file is a ZIP file, or not.
pub fn is_zip(file: &PathBuf) -> bool {
    try_is_zip(file).unwrap_or_default()
}
