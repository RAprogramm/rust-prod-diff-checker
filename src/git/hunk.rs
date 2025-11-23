use serde::{Deserialize, Serialize};

/// Type of line in a diff hunk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineType {
    /// Line was added
    Added,
    /// Line was removed
    Removed,
    /// Context line (unchanged)
    Context,
}

/// A single line in a diff hunk
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HunkLine {
    /// Type of the line
    pub line_type: LineType,
    /// Line number in original file (for removed/context)
    pub old_line: Option<usize>,
    /// Line number in new file (for added/context)
    pub new_line: Option<usize>,
    /// Content of the line
    pub content: String,
}

impl HunkLine {
    /// Creates a new added line
    ///
    /// # Arguments
    ///
    /// * `new_line` - Line number in new file
    /// * `content` - Content of the line
    ///
    /// # Returns
    ///
    /// A new HunkLine for an added line
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::HunkLine;
    ///
    /// let line = HunkLine::added(10, "let x = 5;".to_string());
    /// assert_eq!(line.new_line, Some(10));
    /// ```
    pub fn added(new_line: usize, content: String) -> Self {
        Self {
            line_type: LineType::Added,
            old_line: None,
            new_line: Some(new_line),
            content,
        }
    }

    /// Creates a new removed line
    ///
    /// # Arguments
    ///
    /// * `old_line` - Line number in original file
    /// * `content` - Content of the line
    ///
    /// # Returns
    ///
    /// A new HunkLine for a removed line
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::HunkLine;
    ///
    /// let line = HunkLine::removed(5, "let y = 10;".to_string());
    /// assert_eq!(line.old_line, Some(5));
    /// ```
    pub fn removed(old_line: usize, content: String) -> Self {
        Self {
            line_type: LineType::Removed,
            old_line: Some(old_line),
            new_line: None,
            content,
        }
    }

    /// Creates a new context line
    ///
    /// # Arguments
    ///
    /// * `old_line` - Line number in original file
    /// * `new_line` - Line number in new file
    /// * `content` - Content of the line
    ///
    /// # Returns
    ///
    /// A new HunkLine for a context line
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::HunkLine;
    ///
    /// let line = HunkLine::context(5, 6, "fn main() {".to_string());
    /// assert_eq!(line.old_line, Some(5));
    /// assert_eq!(line.new_line, Some(6));
    /// ```
    pub fn context(old_line: usize, new_line: usize, content: String) -> Self {
        Self {
            line_type: LineType::Context,
            old_line: Some(old_line),
            new_line: Some(new_line),
            content,
        }
    }

    /// Checks if this is an added line
    ///
    /// # Returns
    ///
    /// `true` if line was added
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::HunkLine;
    ///
    /// let line = HunkLine::added(10, "code".to_string());
    /// assert!(line.is_added());
    /// ```
    pub fn is_added(&self) -> bool {
        matches!(self.line_type, LineType::Added)
    }

    /// Checks if this is a removed line
    ///
    /// # Returns
    ///
    /// `true` if line was removed
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::HunkLine;
    ///
    /// let line = HunkLine::removed(5, "code".to_string());
    /// assert!(line.is_removed());
    /// ```
    pub fn is_removed(&self) -> bool {
        matches!(self.line_type, LineType::Removed)
    }
}

/// A hunk in a unified diff
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hunk {
    /// Starting line in original file
    pub old_start: usize,
    /// Number of lines in original file
    pub old_count: usize,
    /// Starting line in new file
    pub new_start: usize,
    /// Number of lines in new file
    pub new_count: usize,
    /// Lines in the hunk
    pub lines: Vec<HunkLine>,
}

impl Hunk {
    /// Creates a new hunk
    ///
    /// # Arguments
    ///
    /// * `old_start` - Starting line in original file
    /// * `old_count` - Number of lines in original file
    /// * `new_start` - Starting line in new file
    /// * `new_count` - Number of lines in new file
    ///
    /// # Returns
    ///
    /// A new Hunk with empty lines
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::Hunk;
    ///
    /// let hunk = Hunk::new(10, 5, 10, 7);
    /// assert_eq!(hunk.old_start, 10);
    /// assert!(hunk.lines.is_empty());
    /// ```
    pub fn new(old_start: usize, old_count: usize, new_start: usize, new_count: usize) -> Self {
        Self {
            old_start,
            old_count,
            new_start,
            new_count,
            lines: Vec::new(),
        }
    }

    /// Returns count of added lines
    ///
    /// # Returns
    ///
    /// Number of added lines in hunk
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::{Hunk, HunkLine};
    ///
    /// let mut hunk = Hunk::new(1, 1, 1, 2);
    /// hunk.lines.push(HunkLine::added(1, "new line".to_string()));
    /// assert_eq!(hunk.added_count(), 1);
    /// ```
    pub fn added_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_added()).count()
    }

    /// Returns count of removed lines
    ///
    /// # Returns
    ///
    /// Number of removed lines in hunk
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::{Hunk, HunkLine};
    ///
    /// let mut hunk = Hunk::new(1, 2, 1, 1);
    /// hunk.lines.push(HunkLine::removed(1, "old line".to_string()));
    /// assert_eq!(hunk.removed_count(), 1);
    /// ```
    pub fn removed_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_removed()).count()
    }

    /// Returns all added line numbers in new file
    ///
    /// # Returns
    ///
    /// Vector of line numbers that were added
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::{Hunk, HunkLine};
    ///
    /// let mut hunk = Hunk::new(1, 1, 1, 2);
    /// hunk.lines.push(HunkLine::added(5, "new".to_string()));
    /// hunk.lines.push(HunkLine::added(6, "lines".to_string()));
    /// assert_eq!(hunk.added_lines(), vec![5, 6]);
    /// ```
    pub fn added_lines(&self) -> Vec<usize> {
        self.lines
            .iter()
            .filter_map(|l| {
                if l.is_added() {
                    l.new_line
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns all removed line numbers in old file
    ///
    /// # Returns
    ///
    /// Vector of line numbers that were removed
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::git::{Hunk, HunkLine};
    ///
    /// let mut hunk = Hunk::new(1, 2, 1, 1);
    /// hunk.lines.push(HunkLine::removed(3, "old".to_string()));
    /// assert_eq!(hunk.removed_lines(), vec![3]);
    /// ```
    pub fn removed_lines(&self) -> Vec<usize> {
        self.lines
            .iter()
            .filter_map(|l| {
                if l.is_removed() {
                    l.old_line
                } else {
                    None
                }
            })
            .collect()
    }
}
