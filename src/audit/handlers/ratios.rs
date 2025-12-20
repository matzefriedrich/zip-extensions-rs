use crate::audit::report::{
    MAX_SUSPICIOUS_RATIO, SuspiciousEntry, SuspiciousReason, ZipAuditReport,
};
use crate::entry_audit_handler::EntryAuditHandler;
use crate::entry_view::EntryView;

/// A `RatiosHandler` is used to track and report on compression ratios and file sizes. It detects
/// compression-ratio bombs and provides global stats for policy decisions by accumulating total
/// compressed/uncompressed sizes and by maintaining incremental average and maximum compression
/// ratios.
pub struct RatiosHandler;

impl EntryAuditHandler for RatiosHandler {
    fn visit(&mut self, view: &EntryView, report: &mut ZipAuditReport) {
        report.total_compressed = report.total_compressed.saturating_add(view.compressed_size);
        report.total_uncompressed = report
            .total_uncompressed
            .saturating_add(view.uncompressed_size);

        if view.ratio.is_finite() {
            let n = report.entry_count as f64;
            report.avg_ratio = report.avg_ratio + (view.ratio - report.avg_ratio) / n;
        }
        if view.ratio > report.max_ratio {
            report.max_ratio = view.ratio;
        }

        if view.ratio > MAX_SUSPICIOUS_RATIO {
            report.suspicious_entries.push(SuspiciousEntry {
                name: view.enclosed_name.clone(),
                reason: SuspiciousReason::HugeRatio {
                    compressed: view.compressed_size,
                    uncompressed: view.uncompressed_size,
                },
            });
        }
        if view.compressed_size == 0 && view.uncompressed_size > 1024 * 1024 {
            report.suspicious_entries.push(SuspiciousEntry {
                name: view.enclosed_name.clone(),
                reason: SuspiciousReason::ZeroCompressedButLarge,
            });
        }
    }
}
