use crate::audit::handlers::util;
use std::io::{Read, Seek};
use std::path::PathBuf;
use zip::read::ZipFile;

/// A lightweight, precomputed view over a ZIP entry used by analysis handlers.
#[derive(Debug, Clone)]
pub struct EntryView {
    pub compressed_size: u64,
    pub depth_hint: usize,
    pub enclosed_name: PathBuf,
    pub encrypted: bool,
    pub has_abs: bool,
    pub has_parent_components: bool,
    pub invalid_utf8: bool,
    pub name_raw: Vec<u8>,
    pub ratio: f64,
    pub symlink: bool,
    pub symlink_target: Option<String>,
    pub uncompressed_size: u64,
    pub unix_mode: Option<u32>,
}

impl EntryView {
    /// Processes a `ZipFile` and extracts relevant metadata and properties into an `EntryView` for
    /// easier management and inspection. It performs operations like determining the entry name,
    /// checking for invalid UTF-8 in the file name, computing compression ratios, and identifying
    /// symbolic links.
    pub fn from_entry<R: Read + Seek>(mut entry: ZipFile<R>) -> Self {
        // Build EntryView with precomputed fields
        let name_raw_slice: &[u8] = entry.name_raw();
        let (name_raw, utf8_opt) = util::name_raw_and_utf8(name_raw_slice);
        let invalid_utf8 = utf8_opt.is_none();
        let enclosed_name = entry.mangled_name();

        let has_abs = util::is_absolute_path_bytes(&name_raw);
        let has_parent = util::has_parent_components_bytes(&name_raw);
        let depth_hint = util::depth_hint_bytes(&name_raw);

        let compressed_size = entry.compressed_size();
        let uncompressed_size = entry.size();
        let ratio = util::compression_ratio(compressed_size, uncompressed_size);

        let encrypted = entry.encrypted();

        let unix_mode = entry.unix_mode();
        let symlink = util::is_symlink_unix_mode(unix_mode);
        let mut symlink_target: Option<String> = None;
        const SAFE_UNCOMPRESSED_SIZE_LIMIT: u64 = 8192;
        if symlink && uncompressed_size <= SAFE_UNCOMPRESSED_SIZE_LIMIT {
            let mut target = String::new();
            let _ = entry.read_to_string(&mut target);
            if !target.is_empty() {
                symlink_target = Some(target);
            }
        }

        let view = EntryView {
            compressed_size,
            depth_hint,
            enclosed_name,
            encrypted,
            has_abs,
            has_parent_components: has_parent,
            invalid_utf8,
            name_raw,
            ratio,
            symlink,
            symlink_target,
            uncompressed_size,
            unix_mode,
        };

        view
    }
}
