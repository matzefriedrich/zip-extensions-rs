use crate::audit::entry_audit_handler::EntryAuditHandler;
use crate::audit::entry_view::EntryView;
use crate::audit::report::{MAX_SUSPICIOUS_RATIO, ZipAuditReport};

/// A `RecommendationsHandler` is used to provide actionable guidance to configure safe extraction.
/// This is a report finalization handler that generates human-readable recommendations based
/// on aggregated report fields (e.g., depth, ratios, encryption, and such).
pub struct RecommendationsHandler;

impl EntryAuditHandler for RecommendationsHandler {
    fn visit(&mut self, _view: &EntryView, _report: &mut ZipAuditReport) {}
    fn finish(&mut self, report: &mut ZipAuditReport) {
        if report.has_absolute_paths {
            report
                .recommendations
                .push("Reject ZIPs containing absolute paths.".to_string());
        }
        if report.max_ratio > MAX_SUSPICIOUS_RATIO {
            report
                .recommendations
                .push("Limit max compression ratio (500 recommended).".to_string());
        }
        if report.has_encrypted_entries {
            report
                .recommendations
                .push("Refuse encrypted entries to prevent password prompts.".to_string());
        }
        if report.max_depth_hint > 25 {
            report
                .recommendations
                .push("Limit directory depth during extraction.".to_string());
        }
    }
}
