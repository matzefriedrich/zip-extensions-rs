#[cfg(test)]
mod tests {
    use crate::deflate::zip_ignore_entry_handler::ZipIgnoreEntryHandler;
    use crate::deflate::zip_writer_extensions::ZipWriterExtensions;
    use crate::zip_ignore_entry_handler::IGNORE_FILENAME;
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};
    use zip::read::ZipArchive;
    use zip::write::SimpleFileOptions;
    use zip::{CompressionMethod, ZipWriter};

    #[test]
    fn zip_respects_ignore_files() {
        // Arrange
        let source = create_zip_archive_source_directory();

        // Act
        source.create_archive();

        // Assert
        verify_zip_entries(source);
    }

    fn create_zip_archive_source_directory() -> SourceArchive {
        let source = SourceArchive::new();

        let source_directory_builder = source.dir();
        source_directory_builder
            .add_regular_file("a.txt", "Hello World")
            .add_regular_file("secret.txt", "topsecret")
            .add_regular_file("app.log", "ignored log")
            .zip_ignore(&["secret.txt", "*.log", "sub/ignoredir"]);

        let sub = source_directory_builder.add_directory("sub");
        let sub_directory_builder = sub
            .add_regular_file("keep.txt", "keep")
            .add_regular_file("tmp.tmp", "ignored tmp")
            .zip_ignore(&["*.tmp", "!keep.txt"]);

        let ignored_dir_builder = sub_directory_builder.add_directory("ignoredir");
        ignored_dir_builder.add_regular_file("file.txt", "ignored by dir");

        source
    }

    fn verify_zip_entries(source: SourceArchive) {
        let f = File::open(&source.archive_path).unwrap();
        let mut zip = ZipArchive::new(f).unwrap();

        assert!(has_entry(&mut zip, "a.txt"));
        assert!(has_entry(&mut zip, "sub/"));
        assert!(has_entry(&mut zip, "sub/keep.txt"));

        assert!(!has_entry(&mut zip, "secret.txt"));
        assert!(!has_entry(&mut zip, "app.log"));
        assert!(!has_entry(&mut zip, "sub/tmp.tmp"));
        assert!(!has_entry(&mut zip, "sub/ignoredir/"));
        assert!(!has_entry(&mut zip, "sub/ignoredir/file.txt"));
    }

    struct SourceArchive {
        tmp: TempDir,
        source_folder_path: PathBuf,
        output_folder_path: PathBuf,
        archive_path: PathBuf,
    }

    struct DirectoryEntryBuilder {
        path: PathBuf,
    }

    impl SourceArchive {
        fn new() -> Self {
            let tmp = tempdir().unwrap();

            let source_folder_path = tmp.path().join("source");
            fs::create_dir_all(&source_folder_path).unwrap();

            let output_folder_path = tmp.path().join("output");
            fs::create_dir_all(&output_folder_path).unwrap();

            let archive_path = source_folder_path.join("archive.zip");

            Self {
                tmp,
                source_folder_path,
                output_folder_path,
                archive_path,
            }
        }

        fn dir(&self) -> DirectoryEntryBuilder {
            DirectoryEntryBuilder {
                path: self.source_folder_path.clone(),
            }
        }

        fn create_archive(&self) {
            let file = File::create(&self.archive_path).unwrap();
            let zip_writer = ZipWriter::new(file);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
            zip_writer
                .create_from_directory_with_options(
                    &self.source_folder_path,
                    |_p: &PathBuf| options,
                    &ZipIgnoreEntryHandler::new(),
                )
                .expect("Failed to create archive");
        }
    }

    impl DirectoryEntryBuilder {
        fn add_regular_file(&self, name: &str, content: &str) -> &DirectoryEntryBuilder {
            let file_path = self.path.join(name);
            fs::write(&file_path, content).unwrap();
            self
        }

        fn add_directory(&self, name: &str) -> DirectoryEntryBuilder {
            let sub = self.path.join(name);
            fs::create_dir_all(&sub).unwrap();
            DirectoryEntryBuilder { path: sub }
        }

        fn zip_ignore(&self, pattern: &[&str]) -> &DirectoryEntryBuilder {
            let contents = join_all(pattern);
            fs::write(self.path.join(IGNORE_FILENAME), contents).unwrap();
            self
        }
    }

    fn join_all(parts: &[&str]) -> String {
        parts.join("\n") + "\n"
    }

    fn has_entry<R: std::io::Read + std::io::Seek>(zip: &mut ZipArchive<R>, name: &str) -> bool {
        for i in 0..zip.len() {
            if let Ok(f) = zip.by_index(i) {
                if f.name() == name {
                    return true;
                }
            }
        }
        false
    }
}
