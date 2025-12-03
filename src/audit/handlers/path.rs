use std::path::PathBuf;

use crate::audit::report::{SuspiciousEntry, SuspiciousReason, ZipAuditReport};
use crate::entry_audit_handler::EntryAuditHandler;
use crate::entry_view::EntryView;

/// A `PathHandler` is used to track and report on suspicious paths, and signal safe extraction
/// policies. For instance, the handler detects absolute paths and parent components in entry names,
/// tracks the maximum directory depth hint across entries, and flags entries with invalid UTF-8
/// names.
pub struct PathHandler;

impl EntryAuditHandler for PathHandler {
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport) {
        if view.has_abs {
            report.has_absolute_paths = true;
        }
        if view.has_parent_components {
            report.has_parent_components = true;
        }
        if view.depth_hint > report.max_depth_hint {
            report.max_depth_hint = view.depth_hint;
        }
        if view.invalid_utf8 {
            report.suspicious_entries.push(SuspiciousEntry {
                name: PathBuf::from(String::from_utf8_lossy(&view.name_raw).into_owned()),
                reason: SuspiciousReason::InvalidUtf8,
            });
        }
    }
}
