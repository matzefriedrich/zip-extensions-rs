pub fn count_path_components_bytes(name: &[u8]) -> usize {
    PathDepthAnalyzer { name }.count_components()
}

struct PathDepthAnalyzer<'a> {
    name: &'a [u8],
}

impl<'a> PathDepthAnalyzer<'a> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    #[inline]
    fn is_separator(b: u8) -> bool {
        b == b'/' || b == b'\\'
    }

    #[inline]
    fn count_components(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        let mut depth = 0usize;
        let mut start = 0usize;
        for (i, &b) in self.name.iter().enumerate() {
            if Self::is_separator(b) {
                if i > start {
                    depth += 1;
                }
                start = i + 1;
            }
        }
        if self.name.len() > start {
            depth += 1;
        }
        depth
    }
}
