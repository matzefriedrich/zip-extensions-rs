#[cfg(test)]
mod tests {
    use crate::audit::utils::windows_reserved_name_checker::is_windows_reserved_name;
    use std::path::PathBuf;

    fn reserved(name: &str) -> bool {
        is_windows_reserved_name(&PathBuf::from(name))
    }

    #[test]
    fn basic_reserved_devices() {
        // Core devices
        assert!(reserved("CON"));
        assert!(reserved("PRN"));
        assert!(reserved("AUX"));
        assert!(reserved("NUL"));

        // Case-insensitive matching
        assert!(reserved("con"));
        assert!(reserved("PrN"));
    }

    #[test]
    fn com_ranges_and_superscripts() {
        // COM1..COM9
        for n in 1..=9 {
            let s = format!("COM{n}");
            assert!(reserved(&s), "{} should be reserved", s);
        }

        // Superscript variants (explicit)
        assert!(reserved("COM¹"));
        assert!(reserved("COM²"));
        assert!(reserved("COM³"));
    }

    #[test]
    fn lpt_ranges_and_superscripts() {
        // LPT1..LPT9
        for n in 1..=9 {
            let s = format!("LPT{n}");
            assert!(reserved(&s), "{} should be reserved", s);
        }

        // Superscript variants (explicit)
        assert!(reserved("LPT¹"));
        assert!(reserved("LPT²"));
        assert!(reserved("LPT³"));
    }

    #[test]
    fn reserved_with_extensions_and_trailing_dots() {
        // Names followed by extension must be treated as reserved
        assert!(reserved("NUL.txt"));
        assert!(reserved("lpt9.tar.gz"));

        // Trailing dots trimmed before checking the base
        assert!(reserved("CON."));
        assert!(reserved("CON..."));
        assert!(reserved("NUL.txt..."));
    }

    #[test]
    fn non_reserved_similar_names() {
        // Similar but not reserved
        assert!(!reserved("CONSOLE"));
        assert!(!reserved("LPT10"));
        assert!(!reserved("COM0"));
        assert!(!reserved("COM")); // no number
        assert!(!reserved("LPT")); // no number
        assert!(!reserved("NULl")); // extra letter
        assert!(!reserved("AUX1")); // AUX only without a number
    }
}
