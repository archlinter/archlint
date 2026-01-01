use memchr::memchr_iter;

/// Pre-computed line index for O(log n) line/column lookup instead of O(n) scanning.
/// Uses SIMD-accelerated newline search via memchr.
pub struct LineIndex {
    /// Byte offsets of each line start (first line starts at 0)
    line_starts: Vec<u32>,
}

impl LineIndex {
    /// Build line index from source text using SIMD-accelerated newline search
    #[inline]
    pub fn new(text: &str) -> Self {
        let bytes = text.as_bytes();
        // Pre-allocate based on average line length estimate (~40 chars)
        let estimated_lines = (bytes.len() / 40).max(16);
        let mut line_starts = Vec::with_capacity(estimated_lines);
        line_starts.push(0);

        // Use memchr for SIMD-accelerated newline search
        for pos in memchr_iter(b'\n', bytes) {
            line_starts.push((pos + 1) as u32);
        }

        Self { line_starts }
    }

    /// Get 1-based line and column for a byte offset using binary search (O(log n))
    #[inline]
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset as u32;
        // Binary search for the line containing this offset
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(exact) => exact,
            Err(insert_pos) => insert_pos.saturating_sub(1),
        };
        let line_start = self.line_starts[line_idx];
        let column = (offset - line_start) as usize + 1;
        (line_idx + 1, column)
    }

    /// Get just the 1-based line number
    #[inline]
    pub fn line(&self, offset: usize) -> usize {
        self.line_col(offset).0
    }

    /// Total number of lines
    #[inline]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_index() {
        let text = "line1\nline2\nline3";
        let index = LineIndex::new(text);

        assert_eq!(index.line_col(0), (1, 1)); // 'l' in line1
        assert_eq!(index.line_col(5), (1, 6)); // '\n' after line1
        assert_eq!(index.line_col(6), (2, 1)); // 'l' in line2
        assert_eq!(index.line_col(11), (2, 6)); // '\n' after line2
        assert_eq!(index.line_col(12), (3, 1)); // 'l' in line3
    }

    #[test]
    fn test_empty() {
        let index = LineIndex::new("");
        assert_eq!(index.line_col(0), (1, 1));
        assert_eq!(index.line_count(), 1);
    }

    #[test]
    fn test_single_line() {
        let index = LineIndex::new("hello");
        assert_eq!(index.line_col(0), (1, 1));
        assert_eq!(index.line_col(4), (1, 5));
    }
}
