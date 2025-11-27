#![allow(dead_code)]

pub use crate::read::*;
pub use crate::write::*;

pub mod default_entry_handler;
pub mod entry_handler;
mod file_utils;
pub mod preserve_symlinks_handler;
pub mod read;
pub mod write;
pub mod preserve_symlinks;

// Compile unit tests from `src/tests/*` only when running tests
#[cfg(test)]
mod tests;

