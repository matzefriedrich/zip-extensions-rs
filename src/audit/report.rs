use std::path::PathBuf;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "audit-json", derive(serde::Serialize))]
pub struct ZipAuditReport {
    pub avg_ratio: f64,
    pub duplicate_names: Vec<PathBuf>,
    pub entry_count: u64,
    pub has_absolute_paths: bool,
    pub has_encrypted_entries: bool,
    pub has_parent_components: bool,
    pub has_symlinks: bool,
    pub max_depth_hint: usize,
    pub max_ratio: f64,
    pub recommendations: Vec<String>,
    pub suspicious_entries: Vec<SuspiciousEntry>,
    pub symlinks_point_outside_root: usize,
    pub total_compressed: u64,
    pub total_uncompressed: u64,
    pub truncated_or_mismatch: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "audit-json", derive(serde::Serialize))]
pub struct SuspiciousEntry {
    pub name: PathBuf,
    pub reason: SuspiciousReason,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "audit-json", derive(serde::Serialize))]
pub enum SuspiciousReason {
    HugeRatio { compressed: u64, uncompressed: u64 },
    ExtremelyLongPath,
    InvalidUtf8,
    ControlCharsInName,
    WindowsReservedName,
    ZeroCompressedButLarge,
    HeaderMismatch,
}

impl ZipAuditReport {
    pub fn new() -> Self {
        Self {
            avg_ratio: 0.0,
            duplicate_names: Vec::new(),
            entry_count: 0,
            has_absolute_paths: false,
            has_encrypted_entries: false,
            has_parent_components: false,
            has_symlinks: false,
            max_depth_hint: 0,
            max_ratio: 0.0,
            recommendations: Vec::new(),
            suspicious_entries: Vec::new(),
            symlinks_point_outside_root: 0,
            total_compressed: 0,
            total_uncompressed: 0,
            truncated_or_mismatch: false,
        }
    }
}
