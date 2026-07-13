// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use masterror::AppError;

use super::hunk::{Hunk, HunkLine};
use crate::error::DiffParseError;

/// A file diff containing all hunks for a single file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDiff {
    /// Path to the file (new path if renamed)
    pub path: PathBuf,
    /// Original path (if renamed)
    pub old_path: Option<PathBuf>,
    /// Whether the file was deleted in this diff
    pub is_deleted: bool,
    /// Hunks in this file diff
    pub hunks: Vec<Hunk>,
}

impl FileDiff {
    /// Creates a new file diff
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// A new FileDiff with empty hunks
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::git::FileDiff;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert!(diff.hunks.is_empty());
    /// ```
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            old_path: None,
            is_deleted: false,
            hunks: Vec::new(),
        }
    }

    /// Returns total number of added lines
    ///
    /// # Returns
    ///
    /// Sum of added lines across all hunks
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::git::FileDiff;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert_eq!(diff.total_added(), 0);
    /// ```
    pub fn total_added(&self) -> usize {
        self.hunks.iter().map(|h| h.added_count()).sum()
    }

    /// Returns total number of removed lines
    ///
    /// # Returns
    ///
    /// Sum of removed lines across all hunks
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::git::FileDiff;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert_eq!(diff.total_removed(), 0);
    /// ```
    pub fn total_removed(&self) -> usize {
        self.hunks.iter().map(|h| h.removed_count()).sum()
    }

    /// Returns all added line numbers
    ///
    /// # Returns
    ///
    /// Vector of all added line numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::git::FileDiff;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert!(diff.all_added_lines().is_empty());
    /// ```
    pub fn all_added_lines(&self) -> Vec<usize> {
        self.hunks.iter().flat_map(|h| h.added_lines()).collect()
    }

    /// Returns all removed line numbers
    ///
    /// # Returns
    ///
    /// Vector of all removed line numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::git::FileDiff;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert!(diff.all_removed_lines().is_empty());
    /// ```
    pub fn all_removed_lines(&self) -> Vec<usize> {
        self.hunks.iter().flat_map(|h| h.removed_lines()).collect()
    }

    /// Checks if file path ends with .rs extension
    ///
    /// # Returns
    ///
    /// `true` if file is a Rust source file
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::git::FileDiff;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert!(diff.is_rust_file());
    ///
    /// let diff = FileDiff::new(PathBuf::from("README.md"));
    /// assert!(!diff.is_rust_file());
    /// ```
    pub fn is_rust_file(&self) -> bool {
        self.path
            .extension()
            .map(|ext| ext == "rs")
            .unwrap_or(false)
    }
}

/// Parses unified diff format into structured file diffs
///
/// # Arguments
///
/// * `input` - Unified diff content as string
///
/// # Returns
///
/// Vector of file diffs or parse error
///
/// # Errors
///
/// Returns error if diff format is invalid
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::git::parse_diff;
///
/// let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
/// index 1234567..abcdefg 100644
/// --- a/src/lib.rs
/// +++ b/src/lib.rs
/// @@ -1,3 +1,4 @@
///  fn main() {
/// +    println!("Hello");
///  }
/// "#;
///
/// let files = parse_diff(diff).unwrap();
/// assert_eq!(files.len(), 1);
/// ```
pub fn parse_diff(input: &str) -> Result<Vec<FileDiff>, AppError> {
    let mut files = Vec::new();
    let mut current_file: Option<FileDiff> = None;
    let mut current_hunk: Option<Hunk> = None;
    let mut old_line = 0;
    let mut new_line = 0;

    for line in input.lines() {
        if line.starts_with("diff --git") {
            if let Some(mut file) = current_file.take() {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                files.push(file);
            }

            let path = parse_diff_header(line)?;
            current_file = Some(FileDiff::new(path));
            current_hunk = None;
        } else if line.starts_with("@@") {
            if let Some(ref mut file) = current_file {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }

                let (old_start, old_count, new_start, new_count) = parse_hunk_header(line)?;
                current_hunk = Some(Hunk::new(old_start, old_count, new_start, new_count));
                old_line = old_start;
                new_line = new_start;
            }
        } else if let Some(ref mut hunk) = current_hunk {
            let mut chars = line.chars();
            match chars.next() {
                Some('+') => {
                    hunk.lines
                        .push(HunkLine::added(new_line, chars.as_str().to_string()));
                    new_line += 1;
                }
                Some('-') => {
                    hunk.lines
                        .push(HunkLine::removed(old_line, chars.as_str().to_string()));
                    old_line += 1;
                }
                Some(' ') | None => {
                    hunk.lines.push(HunkLine::context(
                        old_line,
                        new_line,
                        chars.as_str().to_string(),
                    ));
                    old_line += 1;
                    new_line += 1;
                }
                Some(_) => {}
            }
        } else if let Some(ref mut file) = current_file {
            apply_file_metadata(line, file);
        }
    }

    if let Some(mut file) = current_file {
        if let Some(hunk) = current_hunk {
            file.hunks.push(hunk);
        }
        files.push(file);
    }

    Ok(files)
}

/// Applies file-level metadata lines (`---`, `+++`, `rename`, `deleted file
/// mode`) that appear between a `diff --git` header and the first hunk.
fn apply_file_metadata(line: &str, file: &mut FileDiff) {
    if let Some(rest) = line.strip_prefix("+++ ") {
        let target = extract_path_token(rest);
        if target == "/dev/null" {
            file.is_deleted = true;
        } else {
            let path = target.strip_prefix("b/").unwrap_or(&target);
            file.path = PathBuf::from(path);
            if file.old_path.as_deref() == Some(file.path.as_path()) {
                file.old_path = None;
            }
        }
    } else if let Some(rest) = line.strip_prefix("--- ") {
        let target = extract_path_token(rest);
        if target != "/dev/null" {
            let path = target.strip_prefix("a/").unwrap_or(&target);
            if Path::new(path) != file.path {
                file.old_path = Some(PathBuf::from(path));
            }
        }
    } else if let Some(rest) = line.strip_prefix("rename from ") {
        file.old_path = Some(PathBuf::from(unquote_git_path(rest.trim_end())));
    } else if let Some(rest) = line.strip_prefix("rename to ") {
        file.path = PathBuf::from(unquote_git_path(rest.trim_end()));
    } else if line.starts_with("deleted file mode") {
        file.is_deleted = true;
    }
}

/// Extracts a path from the remainder of a `---`/`+++` line, unquoting
/// git-quoted paths and dropping a trailing tab-separated timestamp.
fn extract_path_token(rest: &str) -> String {
    if rest.starts_with('"') {
        unquote_git_path(rest.trim_end())
    } else {
        rest.split('\t')
            .next()
            .unwrap_or(rest)
            .trim_end()
            .to_string()
    }
}

/// Decodes a git-quoted path (`"a/\321\204.rs"`) into its unquoted form.
///
/// Handles the C-style escapes git emits with `core.quotepath=true`:
/// `\\`, `\"`, `\t`, `\n`, `\r` and octal byte escapes (`\321`). Input
/// without surrounding double quotes is returned unchanged.
fn unquote_git_path(raw: &str) -> String {
    let inner = match raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        Some(inner) => inner,
        None => return raw.to_string(),
    };

    let mut bytes = Vec::with_capacity(inner.len());
    let mut iter = inner.bytes().peekable();
    while let Some(byte) = iter.next() {
        if byte != b'\\' {
            bytes.push(byte);
            continue;
        }
        match iter.next() {
            Some(b'n') => bytes.push(b'\n'),
            Some(b't') => bytes.push(b'\t'),
            Some(b'r') => bytes.push(b'\r'),
            Some(digit @ b'0'..=b'7') => {
                let mut value = u32::from(digit - b'0');
                for _ in 0..2 {
                    match iter.peek().copied() {
                        Some(next @ b'0'..=b'7') => {
                            value = value * 8 + u32::from(next - b'0');
                            iter.next();
                        }
                        _ => break,
                    }
                }
                bytes.push(value as u8);
            }
            Some(other) => bytes.push(other),
            None => {}
        }
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

fn parse_diff_header(line: &str) -> Result<PathBuf, AppError> {
    let invalid_header = || {
        AppError::from(DiffParseError {
            message: format!("invalid diff header: {}", line),
        })
    };

    let rest = line
        .strip_prefix("diff --git ")
        .ok_or_else(invalid_header)?;
    let b_part = if rest.ends_with('"') {
        rest.rfind(" \"").map(|pos| &rest[pos + 1..])
    } else {
        rest.rfind(" b/").map(|pos| &rest[pos + 1..])
    };
    let b_part = b_part
        .or_else(|| rest.split_whitespace().nth(1))
        .ok_or_else(invalid_header)?;

    let unquoted = unquote_git_path(b_part);
    let path = unquoted.strip_prefix("b/").unwrap_or(&unquoted);
    Ok(PathBuf::from(path))
}

fn parse_hunk_header(line: &str) -> Result<(usize, usize, usize, usize), AppError> {
    let line = line
        .strip_prefix("@@")
        .and_then(|s| s.split("@@").next())
        .ok_or_else(|| {
            AppError::from(DiffParseError {
                message: format!("invalid hunk header: {}", line),
            })
        })?
        .trim();

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(DiffParseError {
            message: format!("invalid hunk header: {}", line),
        }
        .into());
    }

    let old_range = parts[0].strip_prefix('-').ok_or_else(|| {
        AppError::from(DiffParseError {
            message: format!("invalid old range: {}", parts[0]),
        })
    })?;

    let new_range = parts[1].strip_prefix('+').ok_or_else(|| {
        AppError::from(DiffParseError {
            message: format!("invalid new range: {}", parts[1]),
        })
    })?;

    let (old_start, old_count) = parse_range(old_range)?;
    let (new_start, new_count) = parse_range(new_range)?;

    Ok((old_start, old_count, new_start, new_count))
}

fn parse_range(range: &str) -> Result<(usize, usize), AppError> {
    let parts: Vec<&str> = range.split(',').collect();

    let start = parts[0].parse::<usize>().map_err(|_| {
        AppError::from(DiffParseError {
            message: format!("invalid line number: {}", parts[0]),
        })
    })?;

    let count = if parts.len() > 1 {
        parts[1].parse::<usize>().map_err(|_| {
            AppError::from(DiffParseError {
                message: format!("invalid line count: {}", parts[1]),
            })
        })?
    } else {
        1
    };

    Ok((start, count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_diff() {
        let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
index 1234567..abcdefg 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("Hello");
 }
"#;

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, PathBuf::from("src/lib.rs"));
        assert_eq!(files[0].hunks.len(), 1);
        assert_eq!(files[0].total_added(), 1);
    }

    #[test]
    fn test_parse_multiple_hunks() {
        let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,4 @@
 fn main() {
+    println!("Hello");
 }
@@ -10,2 +11,3 @@
 fn test() {
+    assert!(true);
 }
"#;

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].hunks.len(), 2);
        assert_eq!(files[0].total_added(), 2);
    }

    #[test]
    fn test_parse_multiple_files() {
        let diff = r#"diff --git a/src/a.rs b/src/a.rs
--- a/src/a.rs
+++ b/src/a.rs
@@ -1,1 +1,2 @@
 fn a() {}
+fn a2() {}
diff --git a/src/b.rs b/src/b.rs
--- a/src/b.rs
+++ b/src/b.rs
@@ -1,1 +1,2 @@
 fn b() {}
+fn b2() {}
"#;

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].path, PathBuf::from("src/a.rs"));
        assert_eq!(files[1].path, PathBuf::from("src/b.rs"));
    }

    #[test]
    fn test_is_rust_file() {
        let rust_diff = FileDiff::new(PathBuf::from("src/lib.rs"));
        assert!(rust_diff.is_rust_file());

        let md_diff = FileDiff::new(PathBuf::from("README.md"));
        assert!(!md_diff.is_rust_file());
    }

    #[test]
    fn test_parse_multibyte_content_does_not_panic() {
        let diff = "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ \
                    -1,2 +1,2 @@\n контекст\n-старая строка\n+новая строка\nПривет без префикса\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].total_added(), 1);
        assert_eq!(files[0].total_removed(), 1);
        let hunk = &files[0].hunks[0];
        assert_eq!(hunk.added_lines(), vec![2]);
        assert_eq!(hunk.removed_lines(), vec![2]);
    }

    #[test]
    fn test_parse_deleted_file() {
        let diff = "diff --git a/src/old.rs b/src/old.rs\ndeleted file mode 100644\nindex \
                    1234567..0000000\n--- a/src/old.rs\n+++ /dev/null\n@@ -1,2 +0,0 @@\n-fn \
                    gone() {}\n-fn also_gone() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert!(files[0].is_deleted);
        assert_eq!(files[0].path, PathBuf::from("src/old.rs"));
        assert_eq!(files[0].total_removed(), 2);
    }

    #[test]
    fn test_parse_new_file_is_not_deleted() {
        let diff = "diff --git a/src/new.rs b/src/new.rs\nnew file mode 100644\n--- \
                    /dev/null\n+++ b/src/new.rs\n@@ -0,0 +1,1 @@\n+fn fresh() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert!(!files[0].is_deleted);
        assert!(files[0].old_path.is_none());
        assert_eq!(files[0].path, PathBuf::from("src/new.rs"));
    }

    #[test]
    fn test_parse_renamed_file() {
        let diff = "diff --git a/src/before.rs b/src/after.rs\nsimilarity index 90%\nrename from \
                    src/before.rs\nrename to src/after.rs\n--- a/src/before.rs\n+++ \
                    b/src/after.rs\n@@ -1,1 +1,2 @@\n fn kept() {}\n+fn added() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].path, PathBuf::from("src/after.rs"));
        assert_eq!(files[0].old_path, Some(PathBuf::from("src/before.rs")));
        assert!(!files[0].is_deleted);
    }

    #[test]
    fn test_parse_path_with_spaces() {
        let diff = "diff --git a/src/my file.rs b/src/my file.rs\n--- a/src/my file.rs\n+++ \
                    b/src/my file.rs\n@@ -1,1 +1,2 @@\n fn a() {}\n+fn b() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].path, PathBuf::from("src/my file.rs"));
        assert!(files[0].old_path.is_none());
    }

    #[test]
    fn test_parse_quoted_path_with_octal_escapes() {
        let diff = "diff --git \"a/src/\\321\\204.rs\" \"b/src/\\321\\204.rs\"\n--- \
                    \"a/src/\\321\\204.rs\"\n+++ \"b/src/\\321\\204.rs\"\n@@ -1,1 +1,2 @@\n fn \
                    a() {}\n+fn b() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].path, PathBuf::from("src/ф.rs"));
    }

    #[test]
    fn test_unquote_escapes() {
        assert_eq!(unquote_git_path("plain/path.rs"), "plain/path.rs");
        assert_eq!(unquote_git_path("\"a b.rs\""), "a b.rs");
        assert_eq!(unquote_git_path("\"tab\\there\""), "tab\there");
        assert_eq!(unquote_git_path("\"q\\\"uote\""), "q\"uote");
        assert_eq!(unquote_git_path("\"\\321\\204\""), "ф");
    }

    #[test]
    fn test_removed_line_numbers_across_hunks() {
        let diff = "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ \
                    -1,3 +1,2 @@\n fn a() {}\n-fn removed_one() {}\n fn c() {}\n@@ -10,3 +9,2 \
                    @@\n fn x() {}\n-fn removed_two() {}\n fn z() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].all_removed_lines(), vec![2, 11]);
        assert!(files[0].all_added_lines().is_empty());
    }

    #[test]
    fn test_empty_line_in_hunk_is_empty_context() {
        let diff = "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ \
                    -1,3 +1,4 @@\n fn a() {}\n\n+fn b() {}\n fn c() {}\n";

        let files = parse_diff(diff).expect("parse should succeed");
        assert_eq!(files[0].all_added_lines(), vec![3]);
    }
}
