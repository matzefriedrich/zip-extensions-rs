use std::io::{Read, Seek};
use zip::read::ZipArchive;
use zip::result::{ZipError, ZipResult};

use crate::audit::handlers::entry_audit_handler::{EntryAuditHandler, default_handlers};
use crate::audit::handlers::entry_view::EntryView;
use crate::audit::report::ZipAuditReport;

/// Scan a ZIP archive and produce an audit report using the default handler pipeline.
pub(crate) fn scan_zip<R: Read + Seek>(reader: R) -> ZipResult<ZipAuditReport> {
    scan_zip_with_handlers(reader, default_handlers())
}

/// Scan a ZIP archive with a custom set of entry-analysis handlers.
pub(crate) fn scan_zip_with_handlers<R: Read + Seek>(
    reader: R,
    mut handlers: Vec<Box<dyn EntryAuditHandler>>,
) -> ZipResult<ZipAuditReport> {
    let mut report = ZipAuditReport::new();

    let mut zip = match ZipArchive::new(reader) {
        Ok(z) => z,
        Err(e) => return Err(e),
    };

    for h in handlers.iter_mut() {
        h.begin(zip.len());
    }

    for i in 0..zip.len() {
        let entry_res = zip.by_index(i);
        let entry = match entry_res {
            Ok(e) => e,
            Err(e) => {
                if matches!(e, ZipError::InvalidArchive(_)) {
                    report.truncated_or_mismatch = true;
                }
                return Err(e);
            }
        };

        report.entry_count += 1;

        let view = EntryView::from_entry(entry);

        for h in handlers.iter_mut() {
            h.visit(&view, &mut report);
        }
    }

    for h in handlers.iter_mut() {
        h.finish(&mut report);
    }

    Ok(report)
}
