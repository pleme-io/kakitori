//! Simple line-level text diffing.
//!
//! Used for computing minimal edits between old and new buffer content.

/// A single diff operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffOp {
    /// Lines are equal.
    Equal(usize),
    /// Insert `count` lines from new text starting at `new_start`.
    Insert { new_start: usize, count: usize },
    /// Delete `count` lines from old text starting at `old_start`.
    Delete { old_start: usize, count: usize },
    /// Replace `old_count` lines starting at `old_start` with `new_count` lines.
    Replace {
        old_start: usize,
        old_count: usize,
        new_start: usize,
        new_count: usize,
    },
}

/// Compute a line-level diff between old and new text.
///
/// Returns a list of diff operations that transform `old` into `new`.
#[must_use]
pub fn diff_lines(old: &[&str], new: &[&str]) -> Vec<DiffOp> {
    let mut ops = Vec::new();
    let mut oi = 0;
    let mut ni = 0;

    while oi < old.len() && ni < new.len() {
        if old[oi] == new[ni] {
            // Count consecutive equal lines.
            let start = oi;
            while oi < old.len() && ni < new.len() && old[oi] == new[ni] {
                oi += 1;
                ni += 1;
            }
            ops.push(DiffOp::Equal(oi - start));
        } else {
            // Find the next sync point using a simple lookahead.
            let sync = find_sync(old, new, oi, ni);
            match sync {
                Some((old_end, new_end)) => {
                    let old_count = old_end - oi;
                    let new_count = new_end - ni;
                    if old_count == 0 {
                        ops.push(DiffOp::Insert {
                            new_start: ni,
                            count: new_count,
                        });
                    } else if new_count == 0 {
                        ops.push(DiffOp::Delete {
                            old_start: oi,
                            count: old_count,
                        });
                    } else {
                        ops.push(DiffOp::Replace {
                            old_start: oi,
                            old_count,
                            new_start: ni,
                            new_count,
                        });
                    }
                    oi = old_end;
                    ni = new_end;
                }
                None => {
                    // No sync found — rest is a replace.
                    ops.push(DiffOp::Replace {
                        old_start: oi,
                        old_count: old.len() - oi,
                        new_start: ni,
                        new_count: new.len() - ni,
                    });
                    oi = old.len();
                    ni = new.len();
                }
            }
        }
    }

    if oi < old.len() {
        ops.push(DiffOp::Delete {
            old_start: oi,
            count: old.len() - oi,
        });
    }
    if ni < new.len() {
        ops.push(DiffOp::Insert {
            new_start: ni,
            count: new.len() - ni,
        });
    }

    ops
}

fn find_sync(old: &[&str], new: &[&str], oi: usize, ni: usize) -> Option<(usize, usize)> {
    let lookahead = 10;

    for d in 1..=lookahead {
        // Check if old[oi+d] matches new[ni+d] (both advanced).
        if oi + d < old.len() && ni + d < new.len() && old[oi + d] == new[ni + d] {
            return Some((oi + d, ni + d));
        }
        // Check if old[oi] matches new[ni+d] (insertion in new).
        if ni + d < new.len() && oi < old.len() && old[oi] == new[ni + d] {
            return Some((oi, ni + d));
        }
        // Check if old[oi+d] matches new[ni] (deletion from old).
        if oi + d < old.len() && ni < new.len() && old[oi + d] == new[ni] {
            return Some((oi + d, ni));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_texts() {
        let old = vec!["a", "b", "c"];
        let ops = diff_lines(&old, &old);
        assert_eq!(ops, vec![DiffOp::Equal(3)]);
    }

    #[test]
    fn pure_insertion() {
        let old: Vec<&str> = vec!["a", "c"];
        let new: Vec<&str> = vec!["a", "b", "c"];
        let ops = diff_lines(&old, &new);
        assert!(ops.iter().any(|op| matches!(op, DiffOp::Insert { .. })));
    }

    #[test]
    fn pure_deletion() {
        let old: Vec<&str> = vec!["a", "b", "c"];
        let new: Vec<&str> = vec!["a", "c"];
        let ops = diff_lines(&old, &new);
        assert!(ops.iter().any(|op| matches!(op, DiffOp::Delete { .. })));
    }

    #[test]
    fn empty_to_content() {
        let old: Vec<&str> = vec![];
        let new: Vec<&str> = vec!["hello"];
        let ops = diff_lines(&old, &new);
        assert_eq!(
            ops,
            vec![DiffOp::Insert {
                new_start: 0,
                count: 1,
            }]
        );
    }

    #[test]
    fn content_to_empty() {
        let old: Vec<&str> = vec!["hello"];
        let new: Vec<&str> = vec![];
        let ops = diff_lines(&old, &new);
        assert_eq!(
            ops,
            vec![DiffOp::Delete {
                old_start: 0,
                count: 1,
            }]
        );
    }
}
