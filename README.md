# zip-extensions-rs

![Rust](https://github.com/matzefriedrich/zip-extensions-rs/workflows/Rust/badge.svg)
[![Build status](https://ci.appveyor.com/api/projects/status/41lavncr30iyv5rk/branch/master?svg=true)](https://ci.appveyor.com/project/matzefriedrich/zip-extensions-rs/branch/master)
![Crates.io](https://img.shields.io/crates/v/zip-extensions)


An extension crate for https://github.com/mvdnes/zip-rs that provides high-level functions for common ZIP tasks, such as extracting archives to a directory.

## Usage examples

### Configure dependencies

Add the following dependencies to the `Cargo.toml` file.

````toml
[dependencies]
zip = "0.5.5"
zip-extensions = "0.3.0"
````

See https://github.com/mvdnes/zip-rs fur further information about `zip` dependencies.

### Extracting an archive to a directory

The `ZipArchiveExtensions` trait provides the `extract` method that can be used to unzip an archive to a directory.

````rust
use std::fs::File;
use zip_extensions::read::ZipArchiveExtensions;
...

let file = File::create(archive_file).unwrap();
let mut archive = zip::ZipArchive::new(file).unwrap();
archive.extract(&target_path).unwrap();
````

Alternatively, the `zip_extract` helper can be used.

````rust
use zip_extensions::*;
...
let archive_file: PathBuf = ...
let target_dir: PathBuf = ...
zip_extract(&archive_file, &target_dir).unwrap();
```` 

### Extracting an archive entry into memory

The `zip_extract_file_to_memory` method can be used to extract entries ad-hoc into memory.

````rust
use zip_extensions::*;

let archive_file = PathBuf::from_str(r#"Baloo_Da_2.zip"#).unwrap();
let entry_path = PathBuf::from_str("BalooDa2-Medium.ttf").unwrap();

let mut buffer : Vec<u8> = vec![];
match zip_extensions::zip_extract_file_to_memory(&archive_file, &entry_path, &mut buffer) {
    Ok(()) => { println!("Extracted {} bytes from archive.", buffer.len()) },
    Err(e) => { println!("The entry does not exist.") }
};
````

### Creating an archive from a directory

The `ZipWriterExtensions` trait provides the `create_from_directory` and `create_from_directory_with_options` methods that can be used to add an entire directory hierarchy to an archive.

````rust
use zip::ZipWriter;
use zip_extensions::write::ZipWriterExtensions;
...

let file = File::create(archive_file).unwrap();
let mut zip = ZipWriter::new(file);
zip.create_from_directory(&source_path).unwrap()
````

Alternatively, the `zip_create_from_directory` helper can be used.

````rust
use zip_extensions::*;
...
let archive_file: PathBuf = ...
let source_dir: PathBuf = ...
zip_create_from_directory(&archive_file, &source_dir).unwrap();
````