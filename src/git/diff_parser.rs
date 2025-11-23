use std::path::PathBuf;

use crate::error::AppError;

use super::hunk::{Hunk, HunkLine};

/// A file diff containing all hunks for a single file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDiff {
    /// Path to the file (new path if renamed)
    pub path: PathBuf,
    /// Original path (if renamed)
    pub old_path: Option<PathBuf>,
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
    /// use rust_diff_analyzer::git::FileDiff;
    /// use std::path::PathBuf;
    ///
    /// let diff = FileDiff::new(PathBuf::from("src/lib.rs"));
    /// assert!(diff.hunks.is_empty());
    /// ```
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            old_path: None,
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
    /// use rust_diff_analyzer::git::FileDiff;
    /// use std::path::PathBuf;
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
    /// use rust_diff_analyzer::git::FileDiff;
    /// use std::path::PathBuf;
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
    /// use rust_diff_analyzer::git::FileDiff;
    /// use std::path::PathBuf;
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
    /// use rust_diff_analyzer::git::FileDiff;
    /// use std::path::PathBuf;
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
    /// use rust_diff_analyzer::git::FileDiff;
    /// use std::path::PathBuf;
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
            if let Some(first_char) = line.chars().next() {
                let content = if line.len() > 1 {
                    line[1..].to_string()
                } else {
                    String::new()
                };

                match first_char {
                    '+' => {
                        hunk.lines.push(HunkLine::added(new_line, content));
                        new_line += 1;
                    }
                    '-' => {
                        hunk.lines.push(HunkLine::removed(old_line, content));
                        old_line += 1;
                    }
                    ' ' => {
                        hunk.lines.push(HunkLine::context(old_line, new_line, content));
                        old_line += 1;
                        new_line += 1;
                    }
                    '\\' => {}
                    _ => {}
                }
            }
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

fn parse_diff_header(line: &str) -> Result<PathBuf, AppError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return Err(AppError::DiffParseError {
            message: format!("invalid diff header: {}", line),
        });
    }

    let b_path = parts[3];
    let path = b_path.strip_prefix("b/").unwrap_or(b_path);
    Ok(PathBuf::from(path))
}

fn parse_hunk_header(line: &str) -> Result<(usize, usize, usize, usize), AppError> {
    let line = line
        .strip_prefix("@@")
        .and_then(|s| s.split("@@").next())
        .ok_or_else(|| AppError::DiffParseError {
            message: format!("invalid hunk header: {}", line),
        })?
        .trim();

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(AppError::DiffParseError {
            message: format!("invalid hunk header: {}", line),
        });
    }

    let old_range = parts[0]
        .strip_prefix('-')
        .ok_or_else(|| AppError::DiffParseError {
            message: format!("invalid old range: {}", parts[0]),
        })?;

    let new_range = parts[1]
        .strip_prefix('+')
        .ok_or_else(|| AppError::DiffParseError {
            message: format!("invalid new range: {}", parts[1]),
        })?;

    let (old_start, old_count) = parse_range(old_range)?;
    let (new_start, new_count) = parse_range(new_range)?;

    Ok((old_start, old_count, new_start, new_count))
}

fn parse_range(range: &str) -> Result<(usize, usize), AppError> {
    let parts: Vec<&str> = range.split(',').collect();

    let start = parts[0].parse::<usize>().map_err(|_| AppError::DiffParseError {
        message: format!("invalid line number: {}", parts[0]),
    })?;

    let count = if parts.len() > 1 {
        parts[1].parse::<usize>().map_err(|_| AppError::DiffParseError {
            message: format!("invalid line count: {}", parts[1]),
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
}
