use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::io;

/// Writes all bytes to a file.
pub fn file_write_all_bytes(path: PathBuf, bytes: &[u8]) -> io::Result<usize> {
    let mut file = File::create(path).unwrap();
    file.set_len(0).unwrap();
    file.write(bytes)
}