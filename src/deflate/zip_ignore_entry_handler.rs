use crate::default_entry_handler::DefaultEntryHandler;
use crate::entry_handler::EntryHandler;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use zip::ZipWriter;
use zip::result::ZipResult;
use zip::write::{FileOptionExtension, FileOptions};

/// An EntryHandler wrapper that honors `.zipignore` files similar to how `.gitignore` works.
/// Patterns from `.zipignore` files are merged from the root directory down to deeper levels.
/// If a path is not ignored, delegation continues to the wrapped `inner` handler.
pub struct ZipIgnoreEntryHandler<H = DefaultEntryHandler> {
    per_directory_matcher_cache: Mutex<HashMap<PathBuf, Gitignore>>,
    ignore_filename: &'static str,
    inner: H,
}

pub(crate) const IGNORE_FILENAME: &str = ".zipignore";

impl ZipIgnoreEntryHandler<DefaultEntryHandler> {
    pub fn new() -> Self {
        Self {
            per_directory_matcher_cache: Mutex::new(HashMap::new()),
            ignore_filename: IGNORE_FILENAME,
            inner: DefaultEntryHandler,
        }
    }
}

impl<H> ZipIgnoreEntryHandler<H> {
    pub fn with_inner(inner: H) -> Self {
        Self {
            per_directory_matcher_cache: Mutex::new(HashMap::new()),
            ignore_filename: IGNORE_FILENAME,
            inner,
        }
    }

    fn parent_dir(path: &Path) -> &Path {
        path.parent().unwrap_or(path)
    }

    fn build_matcher(&self, root: &Path, dir: &Path) -> io::Result<Gitignore> {
        // Build by adding all .zipignore files encountered from root to current dir
        let mut ignore_builder = GitignoreBuilder::new(root);

        // Collect directories from root to dir
        let mut stack: Vec<PathBuf> = vec![];
        let mut current_dir = dir;
        loop {
            stack.push(current_dir.to_path_buf());
            if current_dir == root {
                break;
            }
            match current_dir.parent() {
                Some(p) => current_dir = p,
                None => break,
            }
        }
        stack.reverse();

        for d in stack {
            let ignore_file = d.join(self.ignore_filename);
            if ignore_file.exists() {
                let _ = ignore_builder.add(ignore_file);
            }
        }
        let built = ignore_builder
            .build()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(built)
    }

    fn matcher_for_dir(&self, root: &Path, dir: &Path) -> io::Result<Gitignore> {
        let mut cache = self.per_directory_matcher_cache.lock().unwrap();
        if let Some(existing_matcher) = cache.get(dir) {
            return Ok(existing_matcher.clone());
        }
        let new_matcher = self.build_matcher(root, dir)?;
        cache.insert(dir.to_path_buf(), new_matcher.clone());
        Ok(new_matcher)
    }

    fn is_ignored(&self, root: &Path, path: &Path, is_dir: bool) -> bool {
        let dir = if path.is_dir() {
            path
        } else {
            Self::parent_dir(path)
        };
        match self.matcher_for_dir(root, dir) {
            Ok(matcher) => matcher
                .matched_path_or_any_parents(path, is_dir)
                .is_ignore(),
            Err(_) => false,
        }
    }
}

impl<T: FileOptionExtension, H> EntryHandler<T> for ZipIgnoreEntryHandler<H>
where
    H: EntryHandler<T>,
{
    fn handle_entry<W: Write + io::Seek>(
        &self,
        writer: &mut ZipWriter<W>,
        root: &PathBuf,
        entry_path: &PathBuf,
        file_options: FileOptions<T>,
        buffer: &mut Vec<u8>,
    ) -> ZipResult<()> {
        let metadata = std::fs::metadata(entry_path)?;
        let is_dir = metadata.is_dir();
        if self.is_ignored(root.as_path(), entry_path.as_path(), is_dir) {
            return Ok(());
        }
        self.inner
            .handle_entry(writer, root, entry_path, file_options, buffer)
    }
}
