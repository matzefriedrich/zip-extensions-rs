pub fn has_parent_components_bytes(name: &[u8]) -> bool {
    ParentComponentsChecker { name }.has_parent_components()
}

struct ParentComponentsChecker<'a> {
    name: &'a [u8],
}

impl<'a> ParentComponentsChecker<'a> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    #[inline]
    fn contains_posix_parent_component(&self) -> bool {
        // Looks for "../" anywhere in the byte slice
        self.name.windows(3).any(|w| w == b"../")
    }

    #[inline]
    fn contains_windows_parent_component(&self) -> bool {
        // Looks for "..\\" anywhere in the byte slice
        self.name.windows(3).any(|w| w == b"..\\")
    }

    #[inline]
    fn has_parent_components(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.contains_posix_parent_component() || self.contains_windows_parent_component()
    }
}
