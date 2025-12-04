use super::duplicates::DuplicatesHandler;
use super::encryption::EncryptionHandler;
use super::entry_view::EntryView;
use super::names::NamesHandler;
use super::path::PathHandler;
use super::ratios::RatiosHandler;
use super::recommendations::RecommendationsHandler;
use super::symlinks::SymlinksHandler;
use crate::audit::report::ZipAuditReport;

/// Trait for pluggable, focused analyses executed for each entry.
pub trait EntryAuditHandler {
    fn begin(&mut self, _zip_len: usize) {}
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport);
    fn finish(&mut self, _report: &mut ZipAuditReport) {}
}

/// Create the default set of handlers used by `zip_audit_*` APIs.
pub fn default_handlers() -> Vec<Box<dyn EntryAuditHandler>> {
    vec![
        Box::new(PathHandler),
        Box::new(RatiosHandler),
        Box::new(NamesHandler),
        Box::new(EncryptionHandler),
        Box::new(DuplicatesHandler::new()),
        Box::new(SymlinksHandler),
        Box::new(RecommendationsHandler),
    ]
}
