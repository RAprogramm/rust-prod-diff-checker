// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{classification::CodeType, scope::AnalysisScope, semantic_unit::SemanticUnit};

/// A change to a semantic unit
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Change {
    /// Path to the file containing the change
    pub file_path: PathBuf,
    /// The semantic unit that was changed
    pub unit: SemanticUnit,
    /// Classification of the code
    pub classification: CodeType,
    /// Number of lines added
    pub lines_added: usize,
    /// Number of lines removed
    pub lines_removed: usize,
}

impl Change {
    /// Creates a new change
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file
    /// * `unit` - The semantic unit that was changed
    /// * `classification` - Classification of the code
    /// * `lines_added` - Number of lines added
    /// * `lines_removed` - Number of lines removed
    ///
    /// # Returns
    ///
    /// A new Change instance
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::{
    ///     Change, CodeType, LineSpan, SemanticUnit, SemanticUnitKind, Visibility,
    /// };
    ///
    /// let unit = SemanticUnit::new(
    ///     SemanticUnitKind::Function,
    ///     "parse".to_string(),
    ///     Visibility::Public,
    ///     LineSpan::new(10, 30),
    ///     vec![],
    /// );
    ///
    /// let change = Change::new(
    ///     PathBuf::from("src/parser.rs"),
    ///     unit,
    ///     CodeType::Production,
    ///     10,
    ///     5,
    /// );
    ///
    /// assert_eq!(change.lines_added, 10);
    /// ```
    pub fn new(
        file_path: PathBuf,
        unit: SemanticUnit,
        classification: CodeType,
        lines_added: usize,
        lines_removed: usize,
    ) -> Self {
        Self {
            file_path,
            unit,
            classification,
            lines_added,
            lines_removed,
        }
    }

    /// Returns total lines changed (added + removed)
    ///
    /// # Returns
    ///
    /// Sum of lines added and removed
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    ///
    /// use rust_diff_analyzer::types::{
    ///     Change, CodeType, LineSpan, SemanticUnit, SemanticUnitKind, Visibility,
    /// };
    ///
    /// let unit = SemanticUnit::new(
    ///     SemanticUnitKind::Function,
    ///     "parse".to_string(),
    ///     Visibility::Public,
    ///     LineSpan::new(10, 30),
    ///     vec![],
    /// );
    ///
    /// let change = Change::new(
    ///     PathBuf::from("src/parser.rs"),
    ///     unit,
    ///     CodeType::Production,
    ///     10,
    ///     5,
    /// );
    ///
    /// assert_eq!(change.total_lines(), 15);
    /// ```
    pub fn total_lines(&self) -> usize {
        self.lines_added + self.lines_removed
    }
}

/// Summary of analysis results
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    /// Number of production functions changed
    pub prod_functions: usize,
    /// Number of production structs changed
    pub prod_structs: usize,
    /// Number of other production units changed
    pub prod_other: usize,
    /// Total number of test-related units changed
    pub test_units: usize,
    /// Lines added in production code
    pub prod_lines_added: usize,
    /// Lines removed from production code
    pub prod_lines_removed: usize,
    /// Lines added in test code
    pub test_lines_added: usize,
    /// Lines removed from test code
    pub test_lines_removed: usize,
    /// Weighted score based on configuration
    pub weighted_score: usize,
    /// Whether any limit was exceeded
    pub exceeds_limit: bool,
}

impl Summary {
    /// Returns total number of production units changed
    ///
    /// # Returns
    ///
    /// Sum of all production unit counts
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::Summary;
    ///
    /// let summary = Summary {
    ///     prod_functions: 5,
    ///     prod_structs: 2,
    ///     prod_other: 1,
    ///     test_units: 10,
    ///     prod_lines_added: 50,
    ///     prod_lines_removed: 20,
    ///     test_lines_added: 100,
    ///     test_lines_removed: 30,
    ///     weighted_score: 0,
    ///     exceeds_limit: false,
    /// };
    ///
    /// assert_eq!(summary.total_prod_units(), 8);
    /// ```
    pub fn total_prod_units(&self) -> usize {
        self.prod_functions + self.prod_structs + self.prod_other
    }
}

/// Complete analysis result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// List of all changes
    pub changes: Vec<Change>,
    /// Aggregated summary
    pub summary: Summary,
    /// Analysis scope information
    pub scope: AnalysisScope,
}

impl AnalysisResult {
    /// Creates a new analysis result
    ///
    /// # Arguments
    ///
    /// * `changes` - List of changes
    /// * `summary` - Aggregated summary
    /// * `scope` - Analysis scope information
    ///
    /// # Returns
    ///
    /// A new AnalysisResult instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{AnalysisResult, AnalysisScope, Summary};
    ///
    /// let result = AnalysisResult::new(vec![], Summary::default(), AnalysisScope::new());
    /// assert!(result.changes.is_empty());
    /// ```
    pub fn new(changes: Vec<Change>, summary: Summary, scope: AnalysisScope) -> Self {
        Self {
            changes,
            summary,
            scope,
        }
    }

    /// Returns only production changes
    ///
    /// # Returns
    ///
    /// Iterator over production changes
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{AnalysisResult, AnalysisScope, Summary};
    ///
    /// let result = AnalysisResult::new(vec![], Summary::default(), AnalysisScope::new());
    /// assert_eq!(result.production_changes().count(), 0);
    /// ```
    pub fn production_changes(&self) -> impl Iterator<Item = &Change> {
        self.changes
            .iter()
            .filter(|c| c.classification.is_production())
    }

    /// Returns only test-related changes
    ///
    /// # Returns
    ///
    /// Iterator over test-related changes
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{AnalysisResult, AnalysisScope, Summary};
    ///
    /// let result = AnalysisResult::new(vec![], Summary::default(), AnalysisScope::new());
    /// assert_eq!(result.test_changes().count(), 0);
    /// ```
    pub fn test_changes(&self) -> impl Iterator<Item = &Change> {
        self.changes
            .iter()
            .filter(|c| c.classification.is_test_related())
    }
}
