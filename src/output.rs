// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

pub mod comment;
pub mod formatter;
pub mod github;
pub mod json;

pub use comment::{format_comment, get_comment_marker};
pub use formatter::{Formatter, format_output};
pub use github::GithubFormatter;
pub use json::JsonFormatter;
