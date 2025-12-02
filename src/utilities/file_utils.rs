use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Write};
use std::path::{Component, PathBuf};

/// Writes all bytes to a file.
pub fn file_write_all_bytes(path: PathBuf, bytes: &[u8], overwrite: bool) -> io::Result<usize> {
    if path.exists() && !overwrite {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "The specified file already exists.",
        ));
    }
    let mut file = File::create(path)?;
    file.set_len(0)?;
    file.write(bytes)
}

/// Returns a relative path from one path to another.
pub(crate) fn make_relative_path(root: &PathBuf, current: &PathBuf) -> PathBuf {
    let mut result = PathBuf::new();
    let root_components = root.components().collect::<Vec<Component>>();
    let current_components = current.components().collect::<Vec<_>>();
    for i in 0..current_components.len() {
        let current_path_component: Component = current_components[i];
        if i < root_components.len() {
            let other: Component = root_components[i];
            if other != current_path_component {
                break;
            }
        } else {
            result.push(current_path_component)
        }
    }
    result
}

// Returns a String representing the given Path.
pub(crate) fn path_as_string(path: &std::path::Path) -> String {
    let mut path_str = String::new();
    for component in path.components() {
        if let Component::Normal(os_str) = component {
            if !path_str.is_empty() {
                path_str.push('/');
            }
            path_str.push_str(&*os_str.to_string_lossy());
        }
    }
    path_str
}
