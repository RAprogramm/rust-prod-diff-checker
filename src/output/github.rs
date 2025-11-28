// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::AppError;

use super::formatter::Formatter;
use crate::{config::Config, types::AnalysisResult};

/// Formatter for GitHub Actions output
pub struct GithubFormatter;

impl Formatter for GithubFormatter {
    fn format(&self, result: &AnalysisResult, _config: &Config) -> Result<String, AppError> {
        let mut output = String::new();

        output.push_str(&format!(
            "prod_functions_changed={}\n",
            result.summary.prod_functions
        ));
        output.push_str(&format!(
            "prod_structs_changed={}\n",
            result.summary.prod_structs
        ));
        output.push_str(&format!(
            "prod_other_changed={}\n",
            result.summary.prod_other
        ));
        output.push_str(&format!(
            "test_units_changed={}\n",
            result.summary.test_units
        ));
        output.push_str(&format!(
            "prod_lines_added={}\n",
            result.summary.prod_lines_added
        ));
        output.push_str(&format!(
            "prod_lines_removed={}\n",
            result.summary.prod_lines_removed
        ));
        output.push_str(&format!(
            "test_lines_added={}\n",
            result.summary.test_lines_added
        ));
        output.push_str(&format!(
            "test_lines_removed={}\n",
            result.summary.test_lines_removed
        ));
        output.push_str(&format!(
            "weighted_score={}\n",
            result.summary.weighted_score
        ));
        output.push_str(&format!("exceeds_limit={}\n", result.summary.exceeds_limit));

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

        assert!(output.contains("prod_functions_changed=5"));
        assert!(output.contains("prod_structs_changed=2"));
        assert!(output.contains("prod_lines_added=50"));
        assert!(output.contains("prod_lines_removed=20"));
        assert!(output.contains("test_lines_added=100"));
        assert!(output.contains("weighted_score=23"));
        assert!(output.contains("exceeds_limit=false"));
    }
}
