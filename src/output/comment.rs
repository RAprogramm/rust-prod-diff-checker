// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::fmt::Write;

use crate::{
    classifier::rules::exceeded_per_type_limits,
    config::Config,
    types::{AnalysisResult, Change, ExclusionReason},
};

/// Escapes text for a markdown table cell rendered as inline code
///
/// A pipe would break the table row and a backtick would terminate the code
/// span, so both are replaced with safe stand-ins.
fn escape_cell(text: &str) -> String {
    text.replace('|', "\\|").replace('`', "'")
}

/// Returns the status icon for a limit comparison
fn status_icon(exceeded: bool) -> &'static str {
    if exceeded { "❌" } else { "✅" }
}

const COMMENT_MARKER: &str = "<!-- rust-diff-analyzer-comment -->";

const MAX_SKIPPED_LISTED: usize = 10;

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

    if summary.exceeds_limit {
        output.push_str("> [!CAUTION]\n");
        output.push_str(
            "> **PR exceeds configured limits.** Consider splitting into smaller PRs.\n",
        );

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
        if let Some(max_lines) = config.limits.max_prod_lines
            && summary.prod_lines_added > max_lines
        {
            exceeded.push(format!(
                "**{}** lines added (limit: {})",
                summary.prod_lines_added, max_lines
            ));
        }
        for (kind, count, limit) in exceeded_per_type_limits(&result.changes, config) {
            exceeded.push(format!(
                "**{}** changed units of type `{}` (limit: {})",
                count, kind, limit
            ));
        }
        if !exceeded.is_empty() {
            output.push_str(">\n");
            for item in &exceeded {
                let _ = writeln!(output, "> - {}", item);
            }
        }
    } else {
        output.push_str("> [!TIP]\n");
        output.push_str("> **PR size is within limits.** Good job keeping changes focused!\n");
    }

    output.push_str("\n<details>\n");
    output.push_str(
        "<summary><strong>Limits</strong> — configured thresholds for this \
         repository</summary>\n\n",
    );
    output.push_str("> *Each metric is compared against its configured maximum. ");
    output.push_str("If any limit is exceeded, the PR check fails.*\n\n");
    output.push_str("| Metric | Value | Limit | Status |\n");
    output.push_str("|--------|------:|------:|:------:|\n");

    let _ = writeln!(
        output,
        "| Production Units | {} | {} | {} |",
        summary.total_prod_units(),
        config.limits.max_prod_units,
        status_icon(summary.total_prod_units() > config.limits.max_prod_units)
    );

    let _ = writeln!(
        output,
        "| Weighted Score | {} | {} | {} |",
        summary.weighted_score,
        config.limits.max_weighted_score,
        status_icon(summary.weighted_score > config.limits.max_weighted_score)
    );

    if let Some(max_lines) = config.limits.max_prod_lines {
        let _ = writeln!(
            output,
            "| Lines Added | {} | {} | {} |",
            summary.prod_lines_added,
            max_lines,
            status_icon(summary.prod_lines_added > max_lines)
        );
    }

    output.push_str("\n**Understanding the metrics:**\n");
    output.push_str(
        "- **Production Units**: Functions, structs, enums, traits, and other semantic code \
         units in production code\n",
    );
    output.push_str(
        "- **Weighted Score**: Complexity score based on unit types (public APIs weigh more than \
         private)\n",
    );
    output.push_str("- **Lines Added**: Raw count of new lines in production code\n");
    output.push_str("\n</details>\n");

    output.push_str("\n<details>\n");
    output.push_str(
        "<summary><strong>Summary</strong> — breakdown of changes by category</summary>\n\n",
    );
    output.push_str(
        "> *Production code counts toward limits. Test code is tracked but doesn't affect \
         limits.*\n\n",
    );
    output.push_str("| Metric | Production | Test |\n");
    output.push_str("|--------|----------:|-----:|\n");
    let _ = writeln!(output, "| Functions | {} | - |", summary.prod_functions);
    let _ = writeln!(output, "| Structs/Enums | {} | - |", summary.prod_structs);
    let _ = writeln!(output, "| Other | {} | - |", summary.prod_other);
    let _ = writeln!(
        output,
        "| Lines added | +{} | +{} |",
        summary.prod_lines_added, summary.test_lines_added
    );
    let _ = writeln!(
        output,
        "| Lines removed | -{} | -{} |",
        summary.prod_lines_removed, summary.test_lines_removed
    );
    let _ = writeln!(
        output,
        "| **Total units** | **{}** | {} |",
        summary.total_prod_units(),
        summary.test_units
    );
    output.push_str("\n</details>\n");

    if config.output.include_details && !result.changes.is_empty() {
        let prod_changes: Vec<_> = result.production_changes().collect();
        let test_changes: Vec<_> = result.test_changes().collect();

        if !prod_changes.is_empty() {
            output.push_str("\n<details>\n");
            let _ = writeln!(
                output,
                "<summary><strong>Production Changes</strong> — {} units modified</summary>\n",
                prod_changes.len()
            );
            output.push_str(
                "> *Semantic units (functions, structs, etc.) that were added or modified in \
                 production code.*\n\n",
            );
            output.push_str("| File | Unit | Type | Changes |\n");
            output.push_str("|------|------|:----:|--------:|\n");
            for change in prod_changes {
                write_change_row(&mut output, change);
            }
            output.push_str("\n</details>\n");
        }

        if !test_changes.is_empty() {
            output.push_str("\n<details>\n");
            let _ = writeln!(
                output,
                "<summary><strong>Test Changes</strong> — {} units modified</summary>\n",
                test_changes.len()
            );
            output.push_str("> *Test code changes don't count toward PR size limits.*\n\n");
            output.push_str("| File | Unit | Type | Changes |\n");
            output.push_str("|------|------|:----:|--------:|\n");
            for change in test_changes {
                write_change_row(&mut output, change);
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

fn write_change_row(output: &mut String, change: &Change) {
    let span = &change.unit.span;
    let _ = writeln!(
        output,
        "| `{}:{}-{}` | `{}` | {} | +{} -{} |",
        escape_cell(&change.file_path.display().to_string()),
        span.start,
        span.end,
        escape_cell(&change.unit.qualified_name()),
        change.unit.kind.as_str(),
        change.lines_added,
        change.lines_removed
    );
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
        let _ = writeln!(
            output,
            "**Analyzed:** {} Rust files\n",
            scope.analyzed_files.len()
        );
    }

    if !scope.exclusion_patterns.is_empty() {
        output.push_str("**Excluded patterns:**\n");
        for pattern in &scope.exclusion_patterns {
            let _ = writeln!(output, "- `{}`", escape_cell(pattern));
        }
        output.push('\n');
    }

    let non_rust = scope.non_rust_count();
    let ignored = scope.ignored_count();
    let deleted = scope.deleted_count();
    let errored = scope.error_count();

    if non_rust > 0 || ignored > 0 || deleted > 0 || errored > 0 {
        output.push_str("**Skipped files:**\n");
        if non_rust > 0 {
            let _ = writeln!(output, "- {} non-Rust files", non_rust);
        }
        if ignored > 0 {
            let _ = writeln!(output, "- {} files matched ignore patterns", ignored);
        }
        if deleted > 0 {
            let _ = writeln!(output, "- {} deleted files", deleted);
        }
        if errored > 0 {
            let _ = writeln!(output, "- {} files with read/parse errors", errored);
        }
        output.push('\n');
    }

    if !scope.skipped_files.is_empty() {
        output.push_str("**Skipped file list:**\n");
        for skipped in scope.skipped_files.iter().take(MAX_SKIPPED_LISTED) {
            let reason = match &skipped.reason {
                ExclusionReason::NonRust => "non-Rust".to_string(),
                ExclusionReason::IgnorePattern(p) => format!("pattern: {}", escape_cell(p)),
                ExclusionReason::Deleted => "deleted".to_string(),
                ExclusionReason::ReadError(e) => format!("read error: {}", escape_cell(e)),
                ExclusionReason::ParseError(e) => format!("parse error: {}", escape_cell(e)),
            };
            let _ = writeln!(
                output,
                "- `{}` ({})",
                escape_cell(&skipped.path.display().to_string()),
                reason
            );
        }
        if scope.skipped_files.len() > MAX_SKIPPED_LISTED {
            let _ = writeln!(
                output,
                "- …and {} more",
                scope.skipped_files.len() - MAX_SKIPPED_LISTED
            );
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

    #[test]
    fn test_escape_cell() {
        assert_eq!(escape_cell("plain/path.rs"), "plain/path.rs");
        assert_eq!(escape_cell("a|b.rs"), "a\\|b.rs");
        assert_eq!(escape_cell("a`b.rs"), "a'b.rs");
    }

    #[test]
    fn test_per_type_limit_reason_listed() {
        use std::path::PathBuf;

        use crate::{
            config::PerTypeLimits,
            types::{Change, CodeType, LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
        };

        let make_change = || {
            Change::new(
                PathBuf::from("src/lib.rs"),
                SemanticUnit::new(
                    SemanticUnitKind::Function,
                    "f".to_string(),
                    Visibility::Public,
                    LineSpan::new(1, 3),
                    vec![],
                ),
                CodeType::Production,
                1,
                0,
            )
        };

        let mut config = Config::default();
        config.limits.per_type = Some(PerTypeLimits {
            functions: Some(1),
            ..PerTypeLimits::default()
        });

        let summary = Summary {
            exceeds_limit: true,
            ..Default::default()
        };
        let result = AnalysisResult::new(
            vec![make_change(), make_change()],
            summary,
            AnalysisScope::new(),
        );
        let output = format_comment(&result, &config);

        assert!(output.contains("changed units of type `function` (limit: 1)"));
    }

    #[test]
    fn test_skipped_file_list_truncated() {
        use std::path::PathBuf;

        let mut scope = AnalysisScope::new();
        for i in 0..15 {
            scope.add_skipped(PathBuf::from(format!("file{}.txt", i)), {
                ExclusionReason::NonRust
            });
        }
        let result = AnalysisResult::new(vec![], Summary::default(), scope);
        let config = Config::default();
        let output = format_comment(&result, &config);

        assert!(output.contains("…and 5 more"));
        assert!(output.contains("`file0.txt`"));
        assert!(!output.contains("`file14.txt`"));
    }
}
