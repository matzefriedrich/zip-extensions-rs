pub fn is_absolute_path_bytes(name: &[u8]) -> bool {
    AbsolutePathChecker { name }.is_absolute()
}

pub fn has_parent_components_bytes(name: &[u8]) -> bool {
    super::parent_components_path_checker::has_parent_components_bytes(name)
}

struct AbsolutePathChecker<'a> {
    name: &'a [u8],
}

impl<'a> AbsolutePathChecker<'a> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    #[inline]
    fn starts_with_separator(&self) -> bool {
        // POSIX root "/" or Windows root "\\"
        !self.is_empty() && (self.name[0] == b'/' || self.name[0] == b'\\')
    }

    #[inline]
    fn has_unc_prefix(&self) -> bool {
        // Windows UNC path: "\\\\server\\share" starts with two backslashes
        self.name.len() >= 2 && self.name[0] == b'\\' && self.name[1] == b'\\'
    }

    #[inline]
    fn is_ascii_alpha(c: u8) -> bool {
        (b'A'..=b'Z').contains(&c) || (b'a'..=b'z').contains(&c)
    }

    #[inline]
    fn has_windows_drive_prefix(&self) -> bool {
        const DRIVE_SEPARATOR_INDEX: usize = 1;
        const PATH_SEPARATOR_INDEX: usize = 2;
        if self.name.len() >= 3
            && self.name[DRIVE_SEPARATOR_INDEX] == b':'
            && (self.name[PATH_SEPARATOR_INDEX] == b'/' || self.name[PATH_SEPARATOR_INDEX] == b'\\')
        {
            let drive = self.name[0];
            return Self::is_ascii_alpha(drive);
        }
        false
    }

    #[inline]
    fn is_absolute(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.starts_with_separator() || self.has_windows_drive_prefix() || self.has_unc_prefix()
    }
}
