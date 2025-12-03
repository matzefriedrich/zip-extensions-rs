use crate::audit::report::ZipAuditReport;
use crate::entry_audit_handler::EntryAuditHandler;
use crate::entry_view::EntryView;

/// An `EncryptionHandler` is used to track and report on encrypted entries. Encrypted entries can
/// trigger password prompts or hide payloads. Many extraction workflows choose to refuse them.
pub struct EncryptionHandler;

impl EntryAuditHandler for EncryptionHandler {
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport) {
        if view.encrypted {
            report.has_encrypted_entries = true;
        }
    }
}
