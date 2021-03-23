#![allow(dead_code)]

pub use crate::read::*;
pub use crate::write::*;

mod file_utils;
pub mod read;
pub mod write;

#[cfg(test)]
mod tests {
    use crate::is_zip;
    use std::fs::{self, File};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn is_zip_returns_false_if_file_does_not_exists() {
        let archive_file = PathBuf::from_str("missing.zip").unwrap();
        let actual = is_zip(&archive_file);
        assert_eq!(actual, false)
    }

    #[test]
    fn is_zip_returns_true() {
        let archive_file = PathBuf::from_str("empty.zip").unwrap();
        let file = File::create(&archive_file.as_path()).unwrap();
        let mut zip_writer = zip::ZipWriter::new(file);
        zip_writer.set_comment("This is an empty ZIP file.");
        zip_writer.finish().unwrap();
        let actual = is_zip(&archive_file);
        fs::remove_file(&archive_file.as_path()).unwrap();
        assert_eq!(actual, true)
    }
}
