use super::util;
use crate::audit::report::{SuspiciousEntry, SuspiciousReason, ZipAuditReport};
use crate::entry_audit_handler::EntryAuditHandler;
use crate::entry_view::EntryView;

pub struct NamesHandler;

/// An `NamesHandler` is used to track and report on suspicious (odd or OS-incompatible) names,
/// which often indicate obfuscation or extraction issues. For instance, the handler flags long
/// paths and very deep hierarchies, control characters in names, and certain reserved names.
impl EntryAuditHandler for NamesHandler {
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport) {
        if util::path_is_extremely_long(&view.name_raw) {
            report.suspicious_entries.push(SuspiciousEntry {
                name: view.enclosed_name.clone(),
                reason: SuspiciousReason::ExtremelyLongPath,
            });
        }
        if util::contains_control_chars(&view.name_raw) {
            report.suspicious_entries.push(SuspiciousEntry {
                name: view.enclosed_name.clone(),
                reason: SuspiciousReason::ControlCharsInName,
            });
        }
        if util::is_windows_reserved_name(&view.enclosed_name) {
            report.suspicious_entries.push(SuspiciousEntry {
                name: view.enclosed_name.clone(),
                reason: SuspiciousReason::WindowsReservedName,
            });
        }
    }
}
