// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::AppError;

use super::formatter::Formatter;
use crate::{config::Config, types::AnalysisResult};

/// Formatter for GitHub Actions output
pub struct GithubFormatter;

impl Formatter for GithubFormatter {
    fn format(&self, result: &AnalysisResult, _config: &Config) -> Result<String, AppError> {
        use std::fmt::Write;

        let summary = &result.summary;
        let mut output = String::new();

        let _ = writeln!(output, "prod_functions_changed={}", summary.prod_functions);
        let _ = writeln!(output, "prod_structs_changed={}", summary.prod_structs);
        let _ = writeln!(output, "prod_other_changed={}", summary.prod_other);
        let _ = writeln!(output, "test_units_changed={}", summary.test_units);
        let _ = writeln!(output, "prod_lines_added={}", summary.prod_lines_added);
        let _ = writeln!(output, "prod_lines_removed={}", summary.prod_lines_removed);
        let _ = writeln!(output, "test_lines_added={}", summary.test_lines_added);
        let _ = writeln!(output, "test_lines_removed={}", summary.test_lines_removed);
        let _ = writeln!(output, "weighted_score={}", summary.weighted_score);
        let _ = writeln!(output, "exceeds_limit={}", summary.exceeds_limit);

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnalysisScope, Summary};

    #[test]
    fn test_github_format() {
        let result = AnalysisResult::new(
            vec![],
            Summary {
                prod_functions: 5,
                prod_structs: 2,
                prod_other: 1,
                test_units: 10,
                prod_lines_added: 50,
                prod_lines_removed: 20,
                test_lines_added: 100,
                test_lines_removed: 30,
                weighted_score: 23,
                exceeds_limit: false,
            },
            AnalysisScope::new(),
        );

        let config = Config::default();
        let output = GithubFormatter
            .format(&result, &config)
            .expect("format should succeed");

        let expected = concat!(
            "prod_functions_changed=5\n",
            "prod_structs_changed=2\n",
            "prod_other_changed=1\n",
            "test_units_changed=10\n",
            "prod_lines_added=50\n",
            "prod_lines_removed=20\n",
            "test_lines_added=100\n",
            "test_lines_removed=30\n",
            "weighted_score=23\n",
            "exceeds_limit=false\n",
        );
        assert_eq!(output, expected);
    }
}
