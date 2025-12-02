# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

* Refactored the project structure by grouping functionality into deflate, inflate, and utilities modules. [#29](https://github.com/matzefriedrich/zip-extensions-rs/pull/29)
* Adds the `ZipIgnoreEntryHandler` to handle `.zipignore` rules for file exclusions [#30](https://github.com/matzefriedrich/zip-extensions-rs/pull/30)


## [0.10.0] - 2025-12-01

* Bumps `zip` crate from version `3.0` to `6.0` [#27](https://github.com/matzefriedrich/zip-extensions-rs/pull/27)
* Updates crate edition to 2024 [#28](https://github.com/matzefriedrich/zip-extensions-rs/pull/28)


## [0.9.1] - 2025-11-27

This release improves archive creation and adds symlink preservation. The core directory traversal logic does now accept pluggable handlers, enabling flexible behavior for different compression strategies. 

### Changed

* Introduces the `EntryHandler` trait that allows customization of how filesystem entries are processed during archive creation. The existing file-handling logic is refactored into the `DefaultEntryHandler`. [#26](https://github.com/matzefriedrich/zip-extensions-rs/pull/26)

* A new `PreserveSymlinksHandler` implementation detects symbolic links and writes them to the ZIP archive as symlinks rather than following their targets. **Use this feature with caution**; refer to the security notes added to the documentation of the newly added `zip_create_from_directory_preserve_symlinks_with_options` function. [#26](https://github.com/matzefriedrich/zip-extensions-rs/pull/26)

* The modules structure has been reorganized for clarity and maintainability. [#26](https://github.com/matzefriedrich/zip-extensions-rs/pull/26)


## [0.8.3] - 2025-05-16

Upgraded the `zip` crate from version `2.6` to `3.0` and updated related feature flags.

### Changed

- Removed obsolete features and added new ones such as `nt-time` and `jiff-02`. Adjusted the default features list accordingly. [#21](https://github.com/matzefriedrich/zip-extensions-rs/pull/21)


## [0.8.2] - 2025-04-22

### Changed

- Updated zip crate dependency from version `2.1` to `2.6` and removed the no longer supported `rand` feature. [#20](https://github.com/matzefriedrich/zip-extensions-rs/pull/20)
- Replaced `ZipFile` with `ZipFile<R>` to fix missing generics. [#20](https://github.com/matzefriedrich/zip-extensions-rs/pull/20)


## [0.8.1] - 2024-07-26

### Changed

- [PR #17] The crate now reflects the `zip` crate features, allowing consumers to configure the required zip features and gain fine-grained control over binary size.


## [0.8.0] - 2024-06-02

The project follows the active development of the `zip` crate and has thus been updated to work with **zip2**; this release comes with several breaking changes in the `write` module.

### Changed

- Upgrades the `zip` package reference; uses the new **zip2** version
- Adds `FileOptionExtension` type argument to the `zip_create_from_directory_with_options` trait and implementation to address zip2 build issues
- Removes the `mut` modifier from the `ZipWriterExtensions` to fix issues


## [0.7.0] - 2024-06-01

### Changed

- Adds support for per-item file options by the `create_from_directory_with_options` method. This introduces a breaking change; instead of passing a `FileOptions` directly an `Fn` must be specified that is called for each file, and must return a `FileOptions` value. [#13](https://github.com/matzefriedrich/zip-extensions-rs/pull/13)
- Upgraded the zip dependency to version `0.6.6`.


## [0.6.2] - 2023-09-03

### Changed

- Upgraded the zip dependency to version `0.6.2` [#10](https://github.com/matzefriedrich/zip-extensions-rs/pull/10)


## [0.6.1] - 2021-07-30

### Fixed

- Fixes formatting and linter warnings [#6](https://github.com/matzefriedrich/zip-extensions-rs/pull/6)


## [0.6.0] - 2020-11-30

### Changed

- Pass through Zip and IO errors (replaces all instances of `unwrap()`) [#4](https://github.com/matzefriedrich/zip-extensions-rs/pull/4)
- [PR #4] Adds tests; extends the `try_is_zip`  method so that it can detect different archive formats [#4](https://github.com/matzefriedrich/zip-extensions-rs/pull/4)


## [0.5.0] - 2020-07-24

### Fixed

- Fixes a bug in the `create_from_directory_with_options` method that could cause files not entirely written to disk; use `write_all` instead of `write`. [#1](https://github.com/matzefriedrich/zip-extensions-rs/pull/1)


## [0.4.0] - 2020-03-25

### Added

- New archive extraction traits `extract`, `extract_file`, and `extract_file_to_memory`
- New entry query traits `entry_path` and `file_number`
- New archive writer traits `create_from_directory` and  `create_from_directory_with_options`
- Helper function that can perform all operations base on a given archive file path