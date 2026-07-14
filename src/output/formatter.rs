// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::AppError;

use super::{comment::format_comment, github::GithubFormatter, json::JsonFormatter};
use crate::{
    config::{Config, OutputFormat},
    types::AnalysisResult,
};

/// Trait for output formatters
pub trait Formatter {
    /// Formats analysis result as string
    ///
    /// # Arguments
    ///
    /// * `result` - Analysis result to format
    /// * `config` - Configuration
    ///
    /// # Returns
    ///
    /// Formatted string or error
    ///
    /// # Errors
    ///
    /// Returns error if formatting fails
    fn format(&self, result: &AnalysisResult, config: &Config) -> Result<String, AppError>;
}

/// Formats analysis result using configured format
///
/// # Arguments
///
/// * `result` - Analysis result to format
/// * `config` - Configuration
///
/// # Returns
///
/// Formatted string or error
///
/// # Errors
///
/// Returns error if formatting fails
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     config::Config,
///     output::format_output,
///     types::{AnalysisResult, AnalysisScope, Summary},
/// };
///
/// let result = AnalysisResult::new(vec![], Summary::default(), AnalysisScope::new());
/// let config = Config::default();
/// let output = format_output(&result, &config).unwrap();
/// ```
pub fn format_output(result: &AnalysisResult, config: &Config) -> Result<String, AppError> {
    match config.output.format {
        OutputFormat::Github => GithubFormatter.format(result, config),
        OutputFormat::Json => JsonFormatter.format(result, config),
        OutputFormat::Human => format_human(result, config),
        OutputFormat::Comment => Ok(format_comment(result, config)),
    }
}

fn format_human(result: &AnalysisResult, _config: &Config) -> Result<String, AppError> {
    let mut output = String::new();

    output.push_str("=== Rust Diff Analysis ===\n\n");

    output.push_str("Production:\n");
    output.push_str(&format!("  Functions: {}\n", result.summary.prod_functions));
    output.push_str(&format!("  Structs: {}\n", result.summary.prod_structs));
    output.push_str(&format!("  Other: {}\n", result.summary.prod_other));
    output.push_str(&format!(
        "  Lines: +{} -{}\n",
        result.summary.prod_lines_added, result.summary.prod_lines_removed
    ));

    output.push_str("\nTest:\n");
    output.push_str(&format!("  Units: {}\n", result.summary.test_units));
    output.push_str(&format!(
        "  Lines: +{} -{}\n",
        result.summary.test_lines_added, result.summary.test_lines_removed
    ));

    output.push_str(&format!(
        "\nWeighted score: {}\n",
        result.summary.weighted_score
    ));

    if result.summary.exceeds_limit {
        output.push_str("\nLIMIT EXCEEDED\n");
    }

    if !result.changes.is_empty() {
        output.push_str("\nChanges:\n");
        for change in &result.changes {
            output.push_str(&format!(
                "  - {} ({}) in {} [+{} -{}]\n",
                change.unit.name,
                change.unit.kind.as_str(),
                change.file_path.display(),
                change.lines_added,
                change.lines_removed
            ));
        }
    }

    Ok(output)
}
