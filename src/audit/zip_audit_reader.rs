use crate::audit::handlers::entry_audit_handler::EntryAuditHandler;
use crate::audit::report::ZipAuditReport;
use crate::audit::scan;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use zip::result::ZipResult;

/// Produce a fast, side-effect-free risk assessment of a ZIP archive from a file path.
pub fn zip_audit_file<P: AsRef<Path>>(archive: P) -> ZipResult<ZipAuditReport> {
    let file = File::open(archive)?;
    zip_audit(file)
}

/// Reader-based audit API. Does not extract or write anything.
pub fn zip_audit<R: Read + Seek>(reader: R) -> ZipResult<ZipAuditReport> {
    scan::scan_zip(reader)
}

/// Reader-based audit API with a custom pipeline of entry-analysis handlers.
///
/// This enables advanced users to inject their own stateful analysis handlers or
/// replace the defaults while reusing the same scanning loop.
pub fn zip_audit_with_handlers<R: Read + Seek>(
    reader: R,
    handlers: Vec<Box<dyn EntryAuditHandler>>,
) -> ZipResult<ZipAuditReport> {
    scan::scan_zip_with_handlers(reader, handlers)
}

#[cfg(feature = "audit-json")]
/// Convenience function returning a JSON representation of the audit report.
pub fn zip_audit_json<P: AsRef<Path>>(archive: P) -> ZipResult<serde_json::Value> {
    let report = zip_audit_file(archive)?;
    Ok(serde_json::to_value(&report).expect("serialization should not fail"))
}
