use super::util;
use crate::audit::entry_audit_handler::EntryAuditHandler;
use crate::audit::entry_view::EntryView;
use crate::audit::report::ZipAuditReport;

/// A `SymlinksHandler` is used to detect POSIX-style symlink entries and optionally count those
/// that point outside a safe extraction root. Since symlinks can redirect extraction to unexpected
/// locations, flagging and counting them enables extraction policies.
pub struct SymlinksHandler;

impl EntryAuditHandler for SymlinksHandler {
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport) {
        if view.symlink {
            report.has_symlinks = true;
            if let Some(t) = &view.symlink_target {
                if !util::is_within_root(t) {
                    report.symlinks_point_outside_root += 1;
                }
            }
        }
    }
}
