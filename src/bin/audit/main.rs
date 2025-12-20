use std::env;
use std::path::Path;
use std::process;
use zip_extensions::audit::zip_audit_reader::zip_audit_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_zipfile>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("Error: File '{}' does not exist.", file_path);
        process::exit(1);
    }

    match zip_audit_file(path) {
        Ok(report) => {
            serde_json::to_writer_pretty(std::io::stdout(), &report).unwrap();
            // println!("{:#?}", report);
        }
        Err(e) => {
            eprintln!("Error auditing zip file: {}", e);
            process::exit(1);
        }
    }
}
