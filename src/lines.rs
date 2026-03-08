//! Line-level buffer operations.

use nvim_oxi::api::Buffer;

/// Get all lines from a buffer.
pub fn get_all_lines(buf: &Buffer) -> tane::Result<Vec<String>> {
    let count = buf.line_count()? as usize;
    let lines = buf.get_lines(0..count, true)?;
    Ok(lines.map(|s| s.to_string_lossy().to_string()).collect())
}

/// Get a single line (0-indexed).
pub fn get_line(buf: &Buffer, line: usize) -> tane::Result<String> {
    let lines = buf.get_lines(line..(line + 1), true)?;
    Ok(lines
        .into_iter()
        .next()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default())
}

/// Replace a range of lines (0-indexed, exclusive end).
pub fn set_lines(buf: &mut Buffer, start: usize, end: usize, replacement: &[&str]) -> tane::Result<()> {
    buf.set_lines(start..end, true, replacement.iter().copied())?;
    Ok(())
}

/// Insert lines after the given line number (0-indexed).
pub fn insert_after(buf: &mut Buffer, after: usize, lines: &[&str]) -> tane::Result<()> {
    buf.set_lines((after + 1)..(after + 1), true, lines.iter().copied())?;
    Ok(())
}

/// Delete a range of lines (0-indexed, exclusive end).
pub fn delete_lines(buf: &mut Buffer, start: usize, end: usize) -> tane::Result<()> {
    buf.set_lines(start..end, true, std::iter::empty::<&str>())?;
    Ok(())
}

/// Append lines at the end of the buffer.
pub fn append(buf: &mut Buffer, lines: &[&str]) -> tane::Result<()> {
    let count = buf.line_count()? as usize;
    buf.set_lines(count..count, true, lines.iter().copied())?;
    Ok(())
}
