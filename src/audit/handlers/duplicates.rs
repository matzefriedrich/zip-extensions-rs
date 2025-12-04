use std::collections::HashSet;
use std::path::PathBuf;

use crate::audit::report::ZipAuditReport;
use crate::entry_audit_handler::EntryAuditHandler;
use crate::entry_view::EntryView;

/// A `DuplicatesHandler` is used to track and manage duplicate entries to prevent processing the
/// same paths multiple times. In a ZIP archive, duplicate paths can cause overrides or undefined
/// behavior during deflation.
pub struct DuplicatesHandler {
    seen: HashSet<PathBuf>,
}

impl DuplicatesHandler {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }
}

impl Default for DuplicatesHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EntryAuditHandler for DuplicatesHandler {
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport) {
        let added = self.seen.insert(view.enclosed_name.clone());
        if added == false {
            report.trace_duplicate(view.enclosed_name.clone());
        }
    }
}
