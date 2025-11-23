// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

pub mod diff_parser;
pub mod hunk;

pub use diff_parser::{FileDiff, parse_diff};
pub use hunk::{Hunk, HunkLine, LineType};
