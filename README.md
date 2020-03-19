# zip-extensions-rs

[![Build status](https://ci.appveyor.com/api/projects/status/41lavncr30iyv5rk/branch/master?svg=true)](https://ci.appveyor.com/project/matzefriedrich/zip-extensions-rs/branch/master)
![Crates.io](https://img.shields.io/crates/v/zip-extensions)


An extension crate for https://github.com/mvdnes/zip-rs that provides high-level functions for common ZIP tasks, such as extracting archives to a directory.

## Usage examples

### Configure dependencies

Add the following dependencies to the `Cargo.toml` file.

````toml
[dependencies]
zip = "0.5.5"
zip-extensions = "0.1.4"
````

See https://github.com/mvdnes/zip-rs fur further information about `zip` dependencies.

### Extracting an archive to a directory

The `ZipArchiveExtensions` trait provides the `extract` method that can be used to unzip an archive to a directory.

````rust
use std::fs::File;
use zip_extensions::read_extensions::ZipArchiveExtensions;
...

let file = File::create(archive_file).unwrap();
let mut archive = zip::ZipArchive::new(file).unwrap();
archive.extract(&target_path).unwrap();
````

### Creating an archive from a directory

The `ZipWriterExtensions` trait provides the `create_from_directory` and `create_from_directory_with_options` methods that can be used to add an entire directory hierarchy to an archive.

````rust
use zip::ZipWriter;
use zip_extensions::write_extensions::ZipWriterExtensions;
...

let file = File::create(archive_file).unwrap();
let mut zip = ZipWriter::new(file);
zip.create_from_directory(&source_path).unwrap()
````