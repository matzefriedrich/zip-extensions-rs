# zip-extensions-rs

![Rust](https://github.com/matzefriedrich/zip-extensions-rs/workflows/Rust/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/zip-extensions)
![Crates.io Size](https://img.shields.io/crates/size/zip-extensions)
![GitHub License](https://img.shields.io/github/license/matzefriedrich/zip-extensions-rs)


A companion crate to https://github.com/zip-rs/zip2 that offers high‑level utilities for everyday ZIP tasks—such as extracting archives to a directory, creating archives from folders, and supporting .zipignore and symlink preservation.

## Usage examples

### Configure dependencies

Add the following dependencies to the `Cargo.toml` file.

````toml
[dependencies]
zip = "6.0"
zip-extensions = "0.13.0"
````

See https://github.com/zip-rs/zip2 fur further information about `zip` dependencies.


### Extracting an archive to a directory

The `ZipArchiveExtensions` trait provides the `extract` method that can be used to unzip an archive to a directory.

````rust
use std::fs::File;
use std::path::PathBuf;
use zip_extensions::inflate::zip_archive_extensions::ZipArchiveExtensions;

let file = File::create(archive_file)?;
let mut archive = zip::ZipArchive::new(file)?;
archive.extract(&target_path)?;
````

Alternatively, the `zip_extract` helper can be used.

````rust
use std::path::PathBuf;
use zip_extensions::*;

let archive_file: PathBuf = ...
let target_dir: PathBuf = ...

zip_extract(&archive_file, &target_dir)?;
```` 


### Extracting an archive entry into memory

The `zip_extract_file_to_memory` method can be used to extract entries ad-hoc into memory.

````rust
use zip_extensions::*;

let archive_file = PathBuf::from_str(r#"Baloo_Da_2.zip"#)?;
let entry_path = PathBuf::from_str("BalooDa2-Medium.ttf")?;
let mut buffer : Vec<u8> = vec![];
match zip_extract_file_to_memory(&archive_file, &entry_path, &mut buffer) {
    Ok(()) => { println!("Extracted {} bytes from archive.", buffer.len()) },
    Err(e) => { println!("The entry does not exist.") }
};
````


### Creating an archive from a directory

The `ZipWriterExtensions` trait provides the `create_from_directory` and `create_from_directory_with_options` methods that can be used to add an entire directory hierarchy to an archive.

````rust
use std::fs::File;
use std::path::PathBuf;
use zip::ZipWriter;
use zip_extensions::deflate::zip_writer_extensions::ZipWriterExtensions;

let file = File::create(archive_file)?;
let zip = ZipWriter::new(file);

zip.create_from_directory(&source_path)?;
````

Alternatively, the `zip_create_from_directory` helper can be used.

````rust
use std::path::PathBuf;
use zip_extensions::*;

let archive_file: PathBuf = ...
let source_dir: PathBuf = ...

zip_create_from_directory(&archive_file, &source_dir)?;
````


### Creating an archive from a directory with preserved symlinks

Use `create_from_directory_with_options` together with the symlink-preserving helper if you want symbolic links to be stored as links instead of being resolved to their targets. This preserves the link metadata inside the ZIP.

````rust
use std::path::PathBuf;
use zip::CompressionMethod;
use zip::write::SimpleFileOptions;
use zip_extensions::deflate::preserve_symlinks::zip_create_from_directory_preserve_symlinks_with_options;

let archive_file: PathBuf = ...
let source_dir: PathBuf = ...
let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

zip_create_from_directory_preserve_symlinks_with_options(
    &archive_file,
    &source_dir,
    |_p| opts,
)?;
````

**Note:** Preserving symlinks can be unsafe when the ZIP will be extracted by unknown tools that do not validate symlink targets. Prefer the non-preserving variant for general distribution.


### Creating an archive from a directory while respecting .zipignore files

To exclude files and folders based on `.zipignore` rules, pass the `ZipIgnoreEntryHandler` together with `create_from_directory_with_options` on a `ZipWriter`.

````rust
use std::path::PathBuf;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;
use zip_extensions::deflate::zip_ignore_entry_handler::ZipIgnoreEntryHandler;
use zip_extensions::deflate::zip_writer_extensions::ZipWriterExtensions;

let source_dir: PathBuf = ...
let zip_writer = ZipWriter::new(...);
let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

zip_writer.create_from_directory_with_options(
    &source_dir,
    |_p: &PathBuf| opts,
    &ZipIgnoreEntryHandler::new(),
)?;
````

Place a `.zipignore` file in any directory you want to influence.