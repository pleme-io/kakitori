//! Range-based text operations.

/// A text range in a buffer (0-indexed line and column).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl TextRange {
    /// Create a new text range.
    #[must_use]
    pub const fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    /// Create a range spanning a single line.
    #[must_use]
    pub const fn line(line: usize) -> Self {
        Self {
            start_line: line,
            start_col: 0,
            end_line: line,
            end_col: usize::MAX,
        }
    }

    /// Check if this range spans a single line.
    #[must_use]
    pub const fn is_single_line(&self) -> bool {
        self.start_line == self.end_line
    }

    /// Number of lines this range spans.
    #[must_use]
    pub const fn line_count(&self) -> usize {
        self.end_line - self.start_line + 1
    }

    /// Check if a position falls within this range.
    #[must_use]
    pub const fn contains(&self, line: usize, col: usize) -> bool {
        if line < self.start_line || line > self.end_line {
            return false;
        }
        if line == self.start_line && col < self.start_col {
            return false;
        }
        if line == self.end_line && col > self.end_col {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_range() {
        let r = TextRange::line(5);
        assert!(r.is_single_line());
        assert_eq!(r.line_count(), 1);
    }

    #[test]
    fn multi_line_range() {
        let r = TextRange::new(1, 0, 3, 10);
        assert!(!r.is_single_line());
        assert_eq!(r.line_count(), 3);
    }

    #[test]
    fn contains_check() {
        let r = TextRange::new(2, 5, 4, 10);
        assert!(!r.contains(1, 0));
        assert!(!r.contains(2, 3));
        assert!(r.contains(2, 5));
        assert!(r.contains(3, 0));
        assert!(r.contains(4, 10));
        assert!(!r.contains(4, 11));
        assert!(!r.contains(5, 0));
    }
}
