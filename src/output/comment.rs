// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use crate::{
    config::Config,
    types::{AnalysisResult, SemanticUnitKind},
};

const COMMENT_MARKER: &str = "<!-- rust-diff-analyzer-comment -->";

/// Formats analysis result as a markdown PR comment
///
/// # Arguments
///
/// * `result` - Analysis result to format
/// * `config` - Configuration for formatting
///
/// # Returns
///
/// Formatted markdown string for PR comment
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     config::Config,
///     output::comment::format_comment,
///     types::{AnalysisResult, Summary},
/// };
///
/// let result = AnalysisResult::new(vec![], Summary::default());
/// let config = Config::default();
/// let output = format_comment(&result, &config);
/// assert!(output.contains("Rust Diff Analysis"));
/// ```
pub fn format_comment(result: &AnalysisResult, config: &Config) -> String {
    let summary = &result.summary;
    let status = if summary.exceeds_limit { "❌" } else { "✅" };

    let mut output = String::new();

    output.push_str(COMMENT_MARKER);
    output.push('\n');
    output.push_str("## Rust Diff Analysis\n\n");

    output.push_str("| Metric | Production | Test |\n");
    output.push_str("|--------|------------|------|\n");
    output.push_str(&format!("| Functions | {} | - |\n", summary.prod_functions));
    output.push_str(&format!(
        "| Structs/Enums | {} | - |\n",
        summary.prod_structs
    ));
    output.push_str(&format!("| Other | {} | - |\n", summary.prod_other));
    output.push_str(&format!(
        "| Lines added | {} | {} |\n",
        summary.prod_lines_added, summary.test_lines_added
    ));
    output.push_str(&format!(
        "| Lines removed | {} | {} |\n",
        summary.prod_lines_removed, summary.test_lines_removed
    ));
    output.push_str(&format!(
        "| Total units | {} | {} |\n",
        summary.total_prod_units(),
        summary.test_units
    ));

    output.push_str("\n### Score\n\n");
    output.push_str(&format!(
        "**{}** / {} {}\n",
        summary.weighted_score, config.limits.max_weighted_score, status
    ));

    if config.output.include_details && !result.changes.is_empty() {
        output.push_str("\n<details>\n");
        output.push_str(&format!(
            "<summary>Changed units ({})</summary>\n\n",
            result.changes.len()
        ));

        let prod_changes: Vec<_> = result.production_changes().collect();
        let test_changes: Vec<_> = result.test_changes().collect();

        if !prod_changes.is_empty() {
            output.push_str(&format!("#### Production ({})\n\n", prod_changes.len()));
            for change in prod_changes {
                let kind = match change.unit.kind {
                    SemanticUnitKind::Function => "function",
                    SemanticUnitKind::Struct => "struct",
                    SemanticUnitKind::Enum => "enum",
                    SemanticUnitKind::Trait => "trait",
                    SemanticUnitKind::Impl => "impl",
                    _ => "other",
                };
                output.push_str(&format!(
                    "- `{}` → `{}` ({})\n",
                    change.file_path.display(),
                    change.unit.name,
                    kind
                ));
            }
            output.push('\n');
        }

        if !test_changes.is_empty() {
            output.push_str(&format!("#### Test ({})\n\n", test_changes.len()));
            for change in test_changes {
                output.push_str(&format!(
                    "- `{}` → `{}`\n",
                    change.file_path.display(),
                    change.unit.name
                ));
            }
            output.push('\n');
        }

        output.push_str("</details>\n");
    }

    output.push_str("\n---\n");
    output.push_str(
        "<sub>[Rust Diff Analyzer](https://github.com/RAprogramm/rust-prod-diff-checker)</sub>\n",
    );

    output
}

/// Returns the comment marker for finding existing comments
///
/// # Returns
///
/// The marker string used to identify analyzer comments
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::output::comment::get_comment_marker;
///
/// let marker = get_comment_marker();
/// assert!(marker.contains("rust-diff-analyzer"));
/// ```
pub fn get_comment_marker() -> &'static str {
    COMMENT_MARKER
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Summary;

    #[test]
    fn test_format_comment() {
        let result = AnalysisResult::new(vec![], Summary::default());
        let config = Config::default();
        let output = format_comment(&result, &config);

        assert!(output.contains(COMMENT_MARKER));
        assert!(output.contains("Rust Diff Analysis"));
        assert!(output.contains("Production"));
        assert!(output.contains("Test"));
    }

    #[test]
    fn test_format_comment_with_exceeded_limit() {
        let summary = Summary {
            exceeds_limit: true,
            ..Default::default()
        };
        let result = AnalysisResult::new(vec![], summary);
        let config = Config::default();
        let output = format_comment(&result, &config);

        assert!(output.contains("❌"));
    }

    #[test]
    fn test_get_comment_marker() {
        let marker = get_comment_marker();
        assert!(marker.contains("rust-diff-analyzer"));
    }
}
