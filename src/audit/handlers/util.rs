use std::path::PathBuf;

pub fn is_absolute_path_bytes(name: &[u8]) -> bool {
    if name.is_empty() {
        return false;
    }
    if name[0] == b'/' || name[0] == b'\\' {
        return true;
    }
    if name.len() >= 3 && name[1] == b':' && (name[2] == b'/' || name[2] == b'\\') {
        let c = name[0];
        if (b'A'..=b'Z').contains(&c) || (b'a'..=b'z').contains(&c) {
            return true;
        }
    }
    if name.len() >= 2 && name[0] == b'\\' && name[1] == b'\\' {
        return true;
    }
    false
}

pub fn has_parent_components_bytes(name: &[u8]) -> bool {
    name.windows(3).any(|w| w == b"../") || name.windows(3).any(|w| w == b"..\\")
}

pub fn depth_hint_bytes(name: &[u8]) -> usize {
    let mut depth = 0usize;
    let mut start = 0usize;
    for (i, b) in name.iter().enumerate() {
        if *b == b'/' || *b == b'\\' {
            if i > start {
                depth += 1;
            }
            start = i + 1;
        }
    }
    if name.len() > start {
        depth += 1;
    }
    depth
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
    if let Some(os) = path.file_name() {
        if let Some(stem) = os.to_str() {
            let s = stem.trim_matches('.').to_ascii_uppercase();
            const RESERVED: [&str; 9] = [
                "CON", "AUX", "PRN", "NUL", "COM1", "COM2", "COM3", "LPT1", "LPT2",
            ];
            return RESERVED.contains(&s.as_str());
        }
    }
    false
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
