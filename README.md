# zip-extensions-rs

![Crates.io](https://img.shields.io/crates/v/zip-extensions)


An extension crate for https://github.com/mvdnes/zip-rs that provides high-level functions for common ZIP tasks, such as extracting archives to a directory.

## Usage examples

### Extracting an archive to directory

The `ZipArchiveExtensions` trail provides the `extract` method that can be used to extract all contents of a ZIP archive to a directory.

````rust
use std::fs::File;
use zip_extensions::read_extensions::ZipArchiveExtensions;

let file = File::open(archive_file).unwrap();
let mut archive = zip::ZipArchive::new(file).unwrap();
archive.extract(&target_path).unwrap();
````