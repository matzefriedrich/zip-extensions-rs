# Experimental: Auditing a ZIP for potential risks (ZIP audit)

The crate provides a fast, side‑effect‑free audit of ZIP files that scans entries for common risk patterns (e.g., extremely deep paths, parent components like `../`, Windows reserved names, suspicious compression ratios, truncated headers, and more).

Enable the `zip-audit` feature to use the ready‑made CLI and, if desired, JSON output serialization.

````toml
[dependencies]
zip-extensions = { version = "0.14", features = ["zip-audit"] }
````

## Run the audit CLI

````bash
cargo run --package zip-extensions \
    --features="zip-audit" \
    --bin zip-audit -- <zip-file>
````

This prints a `JSON` report to `stdout` describing the archive, including suspicious entries and recommendations.


## Create an audit report in your own code

Use the `zip_audit_file` helper to scan a ZIP by file path and work with the structured `ZipAuditReport`.

````rust
use std::path::PathBuf;
use zip_extensions::eap::audit::zip_audit_reader::zip_audit_file;

fn main() -> zip::result::ZipResult<()> {
    let zip_path = PathBuf::from("example.zip");
    let report = zip_audit_file(&zip_path)?;

    println!("Entries: {}", report.entry_count);
    println!("Total compressed: {} bytes", report.total_compressed);
    println!("Total uncompressed: {} bytes", report.total_uncompressed);
    println!("Max compression ratio: {:.2}", report.max_ratio);
    println!("Max path depth hint: {}", report.max_depth_hint);

    if report.has_absolute_paths {
        println!("Warning: Archive contains absolute paths");
    }
    if report.has_parent_components {
        println!("Warning: Archive contains parent components (..)");
    }
    if report.has_symlinks {
        println!("Note: Archive contains symbolic links");
    }

    if !report.suspicious_entries.is_empty() {
        println!("Suspicious entries:");
        for s in &report.suspicious_entries {
            println!("  - {:?}: {:?}", s.name, s.reason);
        }
    }

    if !report.recommendations.is_empty() {
        println!("Recommendations:");
        for r in &report.recommendations {
            println!("  - {}", r);
        }
    }

    Ok(())
}
````

Alternatively, if you already have a `Read + Seek` source (e.g., an in‑memory buffer or a custom reader), use `zip_audit`:

````rust
use std::fs::File;
use zip_extensions::eap::audit::zip_audit_reader::zip_audit_reader;

fn from_file() -> zip::result::ZipResult<()> {
    let file = File::open("example.zip")?;
    let report = zip_audit(file)?;
    println!("Found {} entries", report.entry_count);
    Ok(())
}
````

To serialize the report to `JSON` in your application, enable the `audit-json` feature (automatically included by `zip-audit`) and use `serde_json`:

````rust
#[cfg(feature = "audit-json")]
fn print_json(report: &zip_extensions::eap::audit::report::ZipAuditReport) {
    println!("{}", serde_json::to_string_pretty(report).unwrap());
}
````
