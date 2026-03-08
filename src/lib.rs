//! Kakitori (書き取り) — text and buffer operation utilities.
//!
//! High-level helpers for manipulating Neovim buffers: line operations,
//! range replacements, mark management, and text diffing. Used by
//! plugin crates like `ayumi`, `fuda`, and `migaku`.

pub mod diff;
pub mod lines;
pub mod marks;
pub mod range;
