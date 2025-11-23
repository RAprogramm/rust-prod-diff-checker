pub mod diff_parser;
pub mod hunk;

pub use diff_parser::{parse_diff, FileDiff};
pub use hunk::{Hunk, HunkLine, LineType};
