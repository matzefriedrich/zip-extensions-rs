use std::path::PathBuf;

/// Threshold used to flag entries with an excessively large compression ratio.
/// Values above this are commonly associated with zip-bomb style payloads.
pub const MAX_SUSPICIOUS_RATIO: f64 = 1000.0;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "audit-json", derive(serde::Serialize))]
pub struct ZipAuditReport {
    pub avg_ratio: f64,
    pub duplicate_names: Vec<PathBuf>,
    pub encrypted_entries: Vec<PathBuf>,
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
    /// The entry appears to be highly compressible (or malformed): its
    /// uncompressed size is disproportionately larger than its compressed size.
    /// This pattern is often seen in zip bombs that expand massively upon
    /// extraction.
    ///
    /// `compressed`: size in bytes stored in the archive
    /// `uncompressed`: expected size in bytes after extraction
    HugeRatio { compressed: u64, uncompressed: u64 },

    /// The path length of the entry exceeds a reasonable limit and may cause
    /// issues on some file systems or be used to obfuscate malicious content.
    ExtremelyLongPath,

    /// The entry name is not valid UTF‑8. This can cause tooling problems and
    /// may be used to hide files or confuse reviewers.
    InvalidUtf8,

    /// The entry name contains control characters (non‑printable ASCII), which
    /// can hide parts of the name in terminals and UIs and is commonly used for
    /// obfuscation.
    ControlCharsInName,

    /// The entry name matches a Windows reserved device name (e.g., `CON`,
    /// `PRN`, `AUX`, `NUL`, `COM1`, `LPT1`, etc.). Such names can fail to
    /// extract or behave unexpectedly on Windows systems.
    WindowsReservedName,

    /// The compressed size is zero (or near zero) while the uncompressed size
    /// is large. This may indicate a zip bomb payload or a malformed header.
    ZeroCompressedButLarge,

    /// The file headers indicate inconsistent or truncated metadata (e.g.,
    /// central directory vs. local header mismatch). Extraction may fail or
    /// produce corrupted output.
    HeaderMismatch,
}

impl ZipAuditReport {
    pub fn new() -> Self {
        Self {
            avg_ratio: 0.0,
            duplicate_names: Vec::new(),
            entry_count: 0,
            encrypted_entries: Vec::new(),
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

    pub fn trace_duplicate(&mut self, name: PathBuf) {
        self.duplicate_names.push(name);
    }

    pub fn trace_encrypted(&mut self, name: PathBuf) {
        self.encrypted_entries.push(name);
        self.has_encrypted_entries = true;
    }
}
