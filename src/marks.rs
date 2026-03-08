//! Buffer mark management.

use nvim_oxi::api::Buffer;

/// A named mark position in a buffer.
#[derive(Debug, Clone, Copy)]
pub struct Mark {
    pub line: usize,
    pub col: usize,
}

/// Get the position of a mark in a buffer.
///
/// Mark names: `a`–`z` (buffer-local), `A`–`Z` (global), `<`, `>`, `[`, `]`, etc.
pub fn get_mark(buf: &Buffer, name: char) -> tane::Result<Option<Mark>> {
    let mark = buf.get_mark(name)?;
    let (line, col) = (mark.0 as usize, mark.1 as usize);
    if line == 0 {
        Ok(None)
    } else {
        Ok(Some(Mark {
            line: line - 1, // Convert 1-indexed to 0-indexed
            col,
        }))
    }
}

/// Get the visual selection range (marks `<` and `>`).
pub fn get_visual_range(buf: &Buffer) -> tane::Result<Option<(Mark, Mark)>> {
    let start = get_mark(buf, '<')?;
    let end = get_mark(buf, '>')?;
    match (start, end) {
        (Some(s), Some(e)) => Ok(Some((s, e))),
        _ => Ok(None),
    }
}
