#[cfg(test)]
mod tests {
    use crate::audit::utils::absolute_path_checker::{
        has_parent_components_bytes, is_absolute_path_bytes,
    };

    #[test]
    fn absolute_path_bytes_basic_cases() {
        assert_eq!(is_absolute_path_bytes(b""), false, "empty is not absolute");
        assert_eq!(is_absolute_path_bytes(b"/"), true, "POSIX root is absolute");
        assert_eq!(
            is_absolute_path_bytes(b"\\"),
            true,
            "Windows root is absolute"
        );
        assert_eq!(is_absolute_path_bytes(b"foo/bar"), false, "relative path");
    }

    #[test]
    fn absolute_path_bytes_windows_forms() {
        assert!(is_absolute_path_bytes(b"C:/Windows"));
        assert!(is_absolute_path_bytes(b"C:\\Windows"));
        assert!(is_absolute_path_bytes(b"\\\\server\\share"), "UNC path");
        assert!(
            !is_absolute_path_bytes(b"C:Windows"),
            "no separator after drive letter"
        );
        assert!(
            !is_absolute_path_bytes(b"C\\Windows"),
            "missing colon after drive letter"
        );
    }

    #[test]
    fn absolute_path_mixed_separators_and_leading() {
        assert!(
            is_absolute_path_bytes(b"/foo\\bar"),
            "leading '/' makes it absolute"
        );
    }

    #[test]
    fn has_parent_components_bytes_checks() {
        assert_eq!(has_parent_components_bytes(b""), false);
        assert_eq!(has_parent_components_bytes(b"a/../b"), true);
        assert_eq!(has_parent_components_bytes(b"a\\..\\b"), true);
        assert_eq!(
            has_parent_components_bytes(b".."),
            false,
            "requires separator after '..'"
        );
        assert_eq!(
            has_parent_components_bytes(b"a..b/c"),
            false,
            "embedded '..' without separator"
        );
        assert_eq!(has_parent_components_bytes(b"../a"), true);
        assert_eq!(has_parent_components_bytes(b"..\\a"), true);
        assert_eq!(has_parent_components_bytes(b"a/..b"), false);
    }
}
