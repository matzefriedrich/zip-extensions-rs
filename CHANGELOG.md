# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2024-06-02

The project follows the active development of the `zip` crate and has thus been updated to work with **zip2**; this release comes with several breaking changes in the `write` module.

### Changed

- Upgrades the `zip` package reference; uses the new **zip2** version
- Adds `FileOptionExtension` type argument to the `zip_create_from_directory_with_options` trait and implementation to address zip2 build issues
- Removes the `mut` modifier from the `ZipWriterExtensions` to fix issues


## [0.7.0] - 2024-06-01

### Changed

- [PR #13] Adds support for per-item file options by the `create_from_directory_with_options` method. This introduces a breaking change; instead of passing a `FileOptions` directly an `Fn` must be specified that is called for each file, and must return a `FileOptions` value.
- Upgraded the zip dependency to version 0.6.6.


## [0.6.2] - 2023-09-03

### Changed

- [PR #10] Upgraded the zip dependency to version 0.6.2


## [0.6.1] - 2021-07-30

### Fixed

- [PR #6] Fixes formatting and linter warnings


## [0.6.0] - 2020-11-30

### Changed

- [PR #4] Pass through Zip and IO errors (replaces all instances of `unwrap()`)
- [PR #4] Adds tests; extends the `try_is_zip`  method so that it can detect different archive formats


## [0.5.0] - 2020-07-24

### Fixed

- [PR #1] Fixes a bug in the `create_from_directory_with_options` method that could cause files not entirely written to disk; use `write_all` instead of `write`.


## [0.4.0] - 2020-03-25

### Added

- New archive extraction traits `extract`, `extract_file`, and `extract_file_to_memory`
- New entry query traits `entry_path` and `file_number`
- New archive writer traits `create_from_directory` and  `create_from_directory_with_options`
- Helper function that can perform all operations base on a given archive file path