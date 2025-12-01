#[cfg(test)]
mod tests {
    use crate::preserve_symlinks::zip_create_from_directory_preserve_symlinks_with_options;
    use std::fs::{self, File};
    use std::io::Read;
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};
    use zip::CompressionMethod;
    use zip::read::{ZipArchive, ZipFile};
    use zip::write::SimpleFileOptions;

    #[test]
    fn test_zip_preserves_symlinks() {
        // Arrange
        let source = create_zip_archive_source_directory();

        #[cfg(unix)]
        std::os::unix::fs::symlink(&source.target_file_path, &source.symlink_target_path).unwrap();

        // Act
        let archive_path = source.output_folder_path.join("archive.zip");
        zip_create_from_directory_preserve_symlinks_with_options(
            &archive_path,
            &source.source_folder_path,
            |_p: &PathBuf| {
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored)
            },
        )
        .expect("Failed to create archive");

        // Assert
        let file = File::open(&archive_path).unwrap();
        let mut zip = ZipArchive::new(file).unwrap();

        verify_regular_file_exists(&mut zip, source.expected_file_content);

        let expected_symlink_entry_name = source
            .symlink_target_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let symlink =
            verify_symlink_is_stored_as_symlink_entry(&mut zip, expected_symlink_entry_name);
        verify_zip_entry_is_marked_as_symlink(symlink);

        #[cfg(unix)]
        {
            let mut link_target = String::new();
            let mut f = zip.by_name(expected_symlink_entry_name).unwrap();
            f.read_to_string(&mut link_target).unwrap();
            assert_eq!(link_target, source.target_file_path.to_str().unwrap());
        }
    }

    struct SourceArchive {
        tmp: TempDir,
        source_folder_path: PathBuf,
        file_path: PathBuf,
        expected_file_content: String,
        target_file_path: PathBuf,
        expected_target_file_content: String,
        symlink_target_path: PathBuf,
        output_folder_path: PathBuf,
    }

    fn create_zip_archive_source_directory() -> SourceArchive {
        let tmp = tempdir().unwrap();
        let source_folder_path = tmp.path().join("source");
        fs::create_dir(&source_folder_path).unwrap();

        let output_folder_path = tmp.path().join("output");
        fs::create_dir(&output_folder_path).unwrap();

        let file_content = "Hello World";
        let file_path = add_regular_file(
            &source_folder_path,
            "file.txt".to_string(),
            file_content.to_string(),
        );
        let target_file_content = "target";
        let target_file_path = add_regular_file(
            &source_folder_path,
            "target.txt".to_string(),
            target_file_content.to_string(),
        );
        let symlink_target_path = source_folder_path.join("symlink-target.txt");

        let archive = SourceArchive {
            tmp,
            source_folder_path,
            file_path,
            expected_file_content: file_content.to_string(),
            target_file_path,
            expected_target_file_content: target_file_content.to_string(),
            symlink_target_path,
            output_folder_path,
        };

        archive
    }

    fn add_regular_file(root_folder_path: &PathBuf, name: String, content: String) -> PathBuf {
        let file_path = root_folder_path.join(name);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    fn verify_zip_entry_is_marked_as_symlink(symlink: ZipFile<File>) {
        #[cfg(unix)]
        {
            let unix_mode = symlink.unix_mode().expect("Missing unix_mode");
            let file_type = unix_mode & libc::S_IFMT;

            assert_eq!(
                file_type,
                libc::S_IFLNK,
                "ZIP entry should be a symlink, but unix_mode was {:o}",
                unix_mode
            );
        }
    }

    fn verify_symlink_is_stored_as_symlink_entry<'a>(
        zip: &'a mut ZipArchive<File>,
        name: &'a str,
    ) -> ZipFile<'a, File> {
        let symlink = zip.by_name(name).expect("symlink missing");
        symlink
    }

    fn verify_regular_file_exists(zip: &mut ZipArchive<File>, expected_content: String) {
        let mut f = zip.by_name("file.txt").expect("file.txt missing");
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, expected_content, "file.txt content mismatch");
    }
}
