// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Reason why a file was excluded from analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExclusionReason {
    /// File is not a Rust source file
    NonRust,
    /// File matches an ignore pattern
    IgnorePattern(String),
}

/// Information about a skipped file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkippedFile {
    /// Path to the skipped file
    pub path: PathBuf,
    /// Reason for exclusion
    pub reason: ExclusionReason,
}

impl SkippedFile {
    /// Creates a new skipped file entry
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    /// * `reason` - Reason for exclusion
    ///
    /// # Returns
    ///
    /// A new SkippedFile instance
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::{ExclusionReason, SkippedFile};
    ///
    /// let skipped = SkippedFile::new(PathBuf::from("README.md"), ExclusionReason::NonRust);
    /// assert_eq!(skipped.path, PathBuf::from("README.md"));
    /// ```
    pub fn new(path: PathBuf, reason: ExclusionReason) -> Self {
        Self { path, reason }
    }
}

/// Scope of the analysis showing what was analyzed and what was skipped
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisScope {
    /// Files that were analyzed
    pub analyzed_files: Vec<PathBuf>,
    /// Files that were skipped
    pub skipped_files: Vec<SkippedFile>,
    /// Patterns used for exclusion
    pub exclusion_patterns: Vec<String>,
}

impl AnalysisScope {
    /// Creates a new empty analysis scope
    ///
    /// # Returns
    ///
    /// A new empty AnalysisScope instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::AnalysisScope;
    ///
    /// let scope = AnalysisScope::new();
    /// assert!(scope.analyzed_files.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an analyzed file to the scope
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the analyzed file
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::AnalysisScope;
    ///
    /// let mut scope = AnalysisScope::new();
    /// scope.add_analyzed(PathBuf::from("src/lib.rs"));
    /// assert_eq!(scope.analyzed_files.len(), 1);
    /// ```
    pub fn add_analyzed(&mut self, path: PathBuf) {
        self.analyzed_files.push(path);
    }

    /// Adds a skipped file to the scope
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the skipped file
    /// * `reason` - Reason for exclusion
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::{AnalysisScope, ExclusionReason};
    ///
    /// let mut scope = AnalysisScope::new();
    /// scope.add_skipped(
    ///     PathBuf::from("tests/test.rs"),
    ///     ExclusionReason::IgnorePattern("tests/".to_string()),
    /// );
    /// assert_eq!(scope.skipped_files.len(), 1);
    /// ```
    pub fn add_skipped(&mut self, path: PathBuf, reason: ExclusionReason) {
        self.skipped_files.push(SkippedFile::new(path, reason));
    }

    /// Sets the exclusion patterns
    ///
    /// # Arguments
    ///
    /// * `patterns` - List of exclusion patterns
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::AnalysisScope;
    ///
    /// let mut scope = AnalysisScope::new();
    /// scope.set_patterns(vec!["tests/".to_string(), "benches/".to_string()]);
    /// assert_eq!(scope.exclusion_patterns.len(), 2);
    /// ```
    pub fn set_patterns(&mut self, patterns: Vec<String>) {
        self.exclusion_patterns = patterns;
    }

    /// Returns count of non-Rust files skipped
    ///
    /// # Returns
    ///
    /// Number of non-Rust files
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::{AnalysisScope, ExclusionReason};
    ///
    /// let mut scope = AnalysisScope::new();
    /// scope.add_skipped(PathBuf::from("README.md"), ExclusionReason::NonRust);
    /// scope.add_skipped(PathBuf::from("Cargo.toml"), ExclusionReason::NonRust);
    /// assert_eq!(scope.non_rust_count(), 2);
    /// ```
    pub fn non_rust_count(&self) -> usize {
        self.skipped_files
            .iter()
            .filter(|f| matches!(f.reason, ExclusionReason::NonRust))
            .count()
    }

    /// Returns count of files skipped due to ignore patterns
    ///
    /// # Returns
    ///
    /// Number of ignored files
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::{AnalysisScope, ExclusionReason};
    ///
    /// let mut scope = AnalysisScope::new();
    /// scope.add_skipped(
    ///     PathBuf::from("tests/test.rs"),
    ///     ExclusionReason::IgnorePattern("tests/".to_string()),
    /// );
    /// assert_eq!(scope.ignored_count(), 1);
    /// ```
    pub fn ignored_count(&self) -> usize {
        self.skipped_files
            .iter()
            .filter(|f| matches!(f.reason, ExclusionReason::IgnorePattern(_)))
            .count()
    }
}
