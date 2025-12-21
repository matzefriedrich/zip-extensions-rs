use crate::audit::utils::{
    absolute_path_checker, path_depth_analyzer, windows_reserved_name_checker,
};
use std::path::PathBuf;

pub fn is_absolute_path_bytes(name: &[u8]) -> bool {
    absolute_path_checker::is_absolute_path_bytes(name)
}

pub fn has_parent_components_bytes(name: &[u8]) -> bool {
    absolute_path_checker::has_parent_components_bytes(name)
}

pub fn depth_hint_bytes(name: &[u8]) -> usize {
    path_depth_analyzer::count_path_components_bytes(name)
}

pub fn compression_ratio(c: u64, u: u64) -> f64 {
    if c == 0 {
        return f64::INFINITY;
    }
    (u as f64) / (c as f64)
}

pub fn path_is_extremely_long(name: &[u8]) -> bool {
    name.len() > 255 || depth_hint_bytes(name) > 40
}

pub fn contains_control_chars(name: &[u8]) -> bool {
    name.iter().any(|&b| b < 0x20 || b == 0x7F)
}

pub fn is_windows_reserved_name(path: &PathBuf) -> bool {
    windows_reserved_name_checker::is_windows_reserved_name(path)
}

pub fn is_symlink_unix_mode(unix_mode: Option<u32>) -> bool {
    if let Some(m) = unix_mode {
        (m & 0o170000) == 0o120000
    } else {
        false
    }
}

pub fn is_within_root(target: &str) -> bool {
    let bytes = target.as_bytes();
    !is_absolute_path_bytes(bytes) && !has_parent_components_bytes(bytes)
}

pub fn name_raw_and_utf8(bytes: &[u8]) -> (Vec<u8>, Option<String>) {
    let vec = bytes.to_vec();
    let utf8 = String::from_utf8(vec.clone()).ok();
    (vec, utf8)
}
