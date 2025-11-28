// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use crate::{
    config::Config,
    types::{AnalysisResult, Change, ExclusionReason, SemanticUnitKind},
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
///     types::{AnalysisResult, AnalysisScope, Summary},
/// };
///
/// let result = AnalysisResult::new(vec![], Summary::default(), AnalysisScope::new());
/// let config = Config::default();
/// let output = format_comment(&result, &config);
/// assert!(output.contains("Rust Diff Analysis"));
/// ```
pub fn format_comment(result: &AnalysisResult, config: &Config) -> String {
    let summary = &result.summary;

    let mut output = String::new();

    output.push_str(COMMENT_MARKER);
    output.push('\n');
    output.push_str("## Rust Diff Analysis\n\n");

    // Verdict at the top - most important info first
    if summary.exceeds_limit {
        output.push_str("> [!CAUTION]\n");
        output.push_str("> **PR exceeds configured limits.** Consider splitting into smaller PRs.\n");

        let mut exceeded = Vec::new();
        if summary.total_prod_units() > config.limits.max_prod_units {
            exceeded.push(format!(
                "**{}** units (limit: {})",
                summary.total_prod_units(),
                config.limits.max_prod_units
            ));
        }
        if summary.weighted_score > config.limits.max_weighted_score {
            exceeded.push(format!(
                "**{}** weighted score (limit: {})",
                summary.weighted_score, config.limits.max_weighted_score
            ));
        }
        if let Some(max_lines) = config.limits.max_prod_lines {
            if summary.prod_lines_added > max_lines {
                exceeded.push(format!(
                    "**{}** lines added (limit: {})",
                    summary.prod_lines_added, max_lines
                ));
            }
        }
        if !exceeded.is_empty() {
            output.push_str(">\n");
            for item in &exceeded {
                output.push_str(&format!("> - {}\n", item));
            }
        }
    } else {
        output.push_str("> [!TIP]\n");
        output.push_str("> **PR size is within limits.** Good job keeping changes focused!\n");
    }

    // Limits section - collapsible
    output.push_str("\n<details>\n");
    output.push_str("<summary><strong>Limits</strong> — configured thresholds for this repository</summary>\n\n");
    output.push_str("*Each metric is compared against its configured maximum. ");
    output.push_str("If any limit is exceeded, the PR check fails.*\n\n");
    output.push_str("| Metric | Value | Limit | Status |\n");
    output.push_str("|--------|------:|------:|:------:|\n");

    let units_status = if summary.total_prod_units() > config.limits.max_prod_units {
        "❌"
    } else {
        "✅"
    };
    output.push_str(&format!(
        "| Production Units | {} | {} | {} |\n",
        summary.total_prod_units(),
        config.limits.max_prod_units,
        units_status
    ));

    let score_status = if summary.weighted_score > config.limits.max_weighted_score {
        "❌"
    } else {
        "✅"
    };
    output.push_str(&format!(
        "| Weighted Score | {} | {} | {} |\n",
        summary.weighted_score, config.limits.max_weighted_score, score_status
    ));

    if let Some(max_lines) = config.limits.max_prod_lines {
        let lines_status = if summary.prod_lines_added > max_lines {
            "❌"
        } else {
            "✅"
        };
        output.push_str(&format!(
            "| Lines Added | {} | {} | {} |\n",
            summary.prod_lines_added, max_lines, lines_status
        ));
    }

    output.push_str("\n**Understanding the metrics:**\n");
    output.push_str("- **Production Units**: Functions, structs, enums, traits, and other semantic code units in production code\n");
    output.push_str("- **Weighted Score**: Complexity score based on unit types (public APIs weigh more than private)\n");
    output.push_str("- **Lines Added**: Raw count of new lines in production code\n");
    output.push_str("\n</details>\n");

    // Summary section - collapsible
    output.push_str("\n<details>\n");
    output.push_str("<summary><strong>Summary</strong> — breakdown of changes by category</summary>\n\n");
    output.push_str("*Production code counts toward limits. Test code is tracked but doesn't affect limits.*\n\n");
    output.push_str("| Metric | Production | Test |\n");
    output.push_str("|--------|----------:|-----:|\n");
    output.push_str(&format!("| Functions | {} | - |\n", summary.prod_functions));
    output.push_str(&format!(
        "| Structs/Enums | {} | - |\n",
        summary.prod_structs
    ));
    output.push_str(&format!("| Other | {} | - |\n", summary.prod_other));
    output.push_str(&format!(
        "| Lines added | +{} | +{} |\n",
        summary.prod_lines_added, summary.test_lines_added
    ));
    output.push_str(&format!(
        "| Lines removed | -{} | -{} |\n",
        summary.prod_lines_removed, summary.test_lines_removed
    ));
    output.push_str(&format!(
        "| **Total units** | **{}** | {} |\n",
        summary.total_prod_units(),
        summary.test_units
    ));
    output.push_str("\n</details>\n");

    // Changed units - collapsible
    if config.output.include_details && !result.changes.is_empty() {
        let prod_changes: Vec<_> = result.production_changes().collect();
        let test_changes: Vec<_> = result.test_changes().collect();

        if !prod_changes.is_empty() {
            output.push_str("\n<details>\n");
            output.push_str(&format!(
                "<summary><strong>Production Changes</strong> — {} units modified</summary>\n\n",
                prod_changes.len()
            ));
            output.push_str("*Semantic units (functions, structs, etc.) that were added or modified in production code.*\n\n");
            output.push_str("| File | Unit | Type | Changes |\n");
            output.push_str("|------|------|:----:|--------:|\n");
            for change in prod_changes {
                output.push_str(&format_change_row(change));
            }
            output.push_str("\n</details>\n");
        }

        if !test_changes.is_empty() {
            output.push_str("\n<details>\n");
            output.push_str(&format!(
                "<summary><strong>Test Changes</strong> — {} units modified</summary>\n\n",
                test_changes.len()
            ));
            output.push_str("*Test code changes don't count toward PR size limits.*\n\n");
            output.push_str("| File | Unit | Type | Changes |\n");
            output.push_str("|------|------|:----:|--------:|\n");
            for change in test_changes {
                output.push_str(&format_change_row(change));
            }
            output.push_str("\n</details>\n");
        }
    }

    format_scope_section(&mut output, result);

    output.push_str("\n---\n");
    output.push_str(
        "<sub>[Rust Diff Analyzer](https://github.com/RAprogramm/rust-prod-diff-checker)</sub>\n",
    );

    output
}

fn format_change_row(change: &Change) -> String {
    let kind = match change.unit.kind {
        SemanticUnitKind::Function => "function",
        SemanticUnitKind::Struct => "struct",
        SemanticUnitKind::Enum => "enum",
        SemanticUnitKind::Trait => "trait",
        SemanticUnitKind::Impl => "impl",
        SemanticUnitKind::Const => "const",
        SemanticUnitKind::Static => "static",
        SemanticUnitKind::TypeAlias => "type",
        SemanticUnitKind::Macro => "macro",
        SemanticUnitKind::Module => "module",
    };

    let span = &change.unit.span;
    let file_with_lines = format!(
        "`{}:{}-{}`",
        change.file_path.display(),
        span.start,
        span.end
    );

    let changes = format!("+{} -{}", change.lines_added, change.lines_removed);

    format!(
        "| {} | `{}` | {} | {} |\n",
        file_with_lines,
        change.unit.qualified_name(),
        kind,
        changes
    )
}

fn format_scope_section(output: &mut String, result: &AnalysisResult) {
    let scope = &result.scope;

    if scope.analyzed_files.is_empty()
        && scope.skipped_files.is_empty()
        && scope.exclusion_patterns.is_empty()
    {
        return;
    }

    output.push_str("\n<details>\n");
    output.push_str("<summary>Analysis Scope</summary>\n\n");

    if !scope.analyzed_files.is_empty() {
        output.push_str(&format!(
            "**Analyzed:** {} Rust files\n\n",
            scope.analyzed_files.len()
        ));
    }

    if !scope.exclusion_patterns.is_empty() {
        output.push_str("**Excluded patterns:**\n");
        for pattern in &scope.exclusion_patterns {
            output.push_str(&format!("- `{}`\n", pattern));
        }
        output.push('\n');
    }

    let non_rust = scope.non_rust_count();
    let ignored = scope.ignored_count();

    if non_rust > 0 || ignored > 0 {
        output.push_str("**Skipped files:**\n");
        if non_rust > 0 {
            output.push_str(&format!("- {} non-Rust files\n", non_rust));
        }
        if ignored > 0 {
            output.push_str(&format!("- {} files matched ignore patterns\n", ignored));
        }
        output.push('\n');
    }

    if !scope.skipped_files.is_empty() && scope.skipped_files.len() <= 10 {
        output.push_str("**Skipped file list:**\n");
        for skipped in &scope.skipped_files {
            let reason = match &skipped.reason {
                ExclusionReason::NonRust => "non-Rust".to_string(),
                ExclusionReason::IgnorePattern(p) => format!("pattern: {}", p),
            };
            output.push_str(&format!("- `{}` ({})\n", skipped.path.display(), reason));
        }
        output.push('\n');
    }

    output.push_str("</details>\n");
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
    use crate::types::{AnalysisScope, Summary};

    #[test]
    fn test_format_comment() {
        let result = AnalysisResult::new(vec![], Summary::default(), AnalysisScope::new());
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
        let result = AnalysisResult::new(vec![], summary, AnalysisScope::new());
        let config = Config::default();
        let output = format_comment(&result, &config);

        assert!(output.contains("[!CAUTION]"));
        assert!(output.contains("PR exceeds configured limits"));
    }

    #[test]
    fn test_get_comment_marker() {
        let marker = get_comment_marker();
        assert!(marker.contains("rust-diff-analyzer"));
    }
}
