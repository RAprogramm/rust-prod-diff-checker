// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::AppError;
use serde::Serialize;

use super::formatter::Formatter;
use crate::{config::Config, error::OutputError, types::AnalysisResult};

/// Formatter for JSON output
pub struct JsonFormatter;

#[derive(Serialize)]
struct JsonOutput<'a> {
    summary: &'a crate::types::Summary,
    changes: Vec<JsonChange<'a>>,
}

#[derive(Serialize)]
struct JsonChange<'a> {
    file: String,
    unit: &'a str,
    kind: &'a str,
    visibility: &'a str,
    classification: &'a str,
    lines_added: usize,
    lines_removed: usize,
}

impl Formatter for JsonFormatter {
    fn format(&self, result: &AnalysisResult, config: &Config) -> Result<String, AppError> {
        let changes: Vec<JsonChange> = if config.output.include_details {
            result
                .changes
                .iter()
                .map(|c| JsonChange {
                    file: c.file_path.to_string_lossy().to_string(),
                    unit: &c.unit.name,
                    kind: c.unit.kind.as_str(),
                    visibility: c.unit.visibility.as_str(),
                    classification: c.classification.as_str(),
                    lines_added: c.lines_added,
                    lines_removed: c.lines_removed,
                })
                .collect()
        } else {
            vec![]
        };

        let output = JsonOutput {
            summary: &result.summary,
            changes,
        };

        serde_json::to_string_pretty(&output).map_err(|e| {
            AppError::from(OutputError {
                format: "json".to_string(),
                message: e.to_string(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnalysisScope, Summary};

    #[test]
    fn test_json_format() {
        let result = AnalysisResult::new(
            vec![],
            Summary {
                prod_functions: 3,
                prod_structs: 1,
                prod_other: 0,
                test_units: 5,
                prod_lines_added: 30,
                prod_lines_removed: 10,
                test_lines_added: 50,
                test_lines_removed: 20,
                weighted_score: 15,
                exceeds_limit: false,
            },
            AnalysisScope::new(),
        );

        let config = Config::default();
        let output = JsonFormatter
            .format(&result, &config)
            .expect("format should succeed");

        assert!(output.contains("\"prod_functions\": 3"));
        assert!(output.contains("\"weighted_score\": 15"));
    }
}
