#[cfg(test)]
mod tests {
    use crate::audit::utils::path_depth_analyzer::count_path_components_bytes;

    #[test]
    fn count_components_empty_and_single() {
        assert_eq!(count_path_components_bytes(b""), 0);
        assert_eq!(count_path_components_bytes(b"file"), 1);
    }

    #[test]
    fn count_components_multiple_and_mixed_separators() {
        assert_eq!(count_path_components_bytes(b"a/b/c"), 3);
        assert_eq!(count_path_components_bytes(b"a\\b\\c"), 3);
        assert_eq!(count_path_components_bytes(b"a/b\\c"), 3);
    }

    #[test]
    fn count_components_with_consecutive_and_edge_separators() {
        // consecutive separators should not create empty components
        assert_eq!(count_path_components_bytes(b"a//b///c"), 3);
        // leading/trailing separators should be ignored as components
        assert_eq!(count_path_components_bytes(b"/a/b/"), 2);
        assert_eq!(count_path_components_bytes(b"\\a\\b\\"), 2);
        // only separators -> zero components
        assert_eq!(count_path_components_bytes(b"///"), 0);
        assert_eq!(count_path_components_bytes(b"\\\\"), 0);
    }
}
