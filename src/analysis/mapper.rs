// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, path::Path};

use masterror::AppError;

use super::extractor::extract_semantic_units_from_str;
use crate::{
    classifier::classify_unit,
    config::Config,
    error::FileReadError,
    git::FileDiff,
    types::{AnalysisScope, Change, ExclusionReason, SemanticUnit},
};

/// Result of mapping changes including scope information
pub struct MapResult {
    /// List of changes
    pub changes: Vec<Change>,
    /// Analysis scope
    pub scope: AnalysisScope,
}

/// Maps diff changes to semantic units
///
/// # Arguments
///
/// * `diffs` - Vector of file diffs
/// * `config` - Configuration
/// * `file_reader` - Function to read file contents
///
/// # Returns
///
/// MapResult with changes and scope or error
///
/// # Errors
///
/// Returns error if file reading or parsing fails
///
/// # Examples
///
/// ```no_run
/// use std::fs;
///
/// use rust_diff_analyzer::{analysis::map_changes, config::Config};
///
/// let diffs = vec![];
/// let config = Config::default();
/// let result = map_changes(&diffs, &config, |p| fs::read_to_string(p));
/// ```
pub fn map_changes<F>(
    diffs: &[FileDiff],
    config: &Config,
    file_reader: F,
) -> Result<MapResult, AppError>
where
    F: Fn(&Path) -> Result<String, std::io::Error>,
{
    let mut changes = Vec::new();
    let mut scope = AnalysisScope::new();

    scope.set_patterns(config.classification.ignore_paths.clone());

    for diff in diffs {
        if !diff.is_rust_file() {
            scope.add_skipped(diff.path.clone(), ExclusionReason::NonRust);
            continue;
        }

        if config.should_ignore(&diff.path) {
            let pattern = config
                .classification
                .ignore_paths
                .iter()
                .find(|p| diff.path.to_string_lossy().contains(p.as_str()))
                .cloned()
                .unwrap_or_default();
            scope.add_skipped(diff.path.clone(), ExclusionReason::IgnorePattern(pattern));
            continue;
        }

        scope.add_analyzed(diff.path.clone());

        let content = file_reader(&diff.path)
            .map_err(|e| AppError::from(FileReadError::new(diff.path.clone(), e)))?;

        let units = extract_semantic_units_from_str(&content, &diff.path)?;

        let added_lines = diff.all_added_lines();
        let removed_lines = diff.all_removed_lines();

        let mut unit_changes: HashMap<String, (usize, usize)> = HashMap::new();

        for line in &added_lines {
            if let Some(unit) = find_containing_unit(&units, *line) {
                let entry = unit_changes.entry(unit.qualified_name()).or_insert((0, 0));
                entry.0 += 1;
            }
        }

        for line in &removed_lines {
            if let Some(unit) = find_containing_unit(&units, *line) {
                let entry = unit_changes.entry(unit.qualified_name()).or_insert((0, 0));
                entry.1 += 1;
            }
        }

        for unit in &units {
            if let Some((added, removed)) = unit_changes.get(&unit.qualified_name()) {
                let classification = classify_unit(unit, &diff.path, config);

                changes.push(Change::new(
                    diff.path.clone(),
                    unit.clone(),
                    classification,
                    *added,
                    *removed,
                ));
            }
        }
    }

    Ok(MapResult { changes, scope })
}

fn find_containing_unit(units: &[SemanticUnit], line: usize) -> Option<&SemanticUnit> {
    let mut best_match: Option<&SemanticUnit> = None;

    for unit in units {
        if unit.span.contains(line) {
            match best_match {
                None => best_match = Some(unit),
                Some(current) => {
                    if unit.span.len() < current.span.len() {
                        best_match = Some(unit);
                    }
                }
            }
        }
    }

    best_match
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LineSpan, SemanticUnitKind, Visibility};

    #[test]
    fn test_find_containing_unit() {
        let units = vec![
            SemanticUnit::new(
                SemanticUnitKind::Module,
                "module".to_string(),
                Visibility::Private,
                LineSpan::new(1, 100),
                vec![],
            ),
            SemanticUnit::new(
                SemanticUnitKind::Function,
                "func".to_string(),
                Visibility::Public,
                LineSpan::new(10, 20),
                vec![],
            ),
        ];

        let result = find_containing_unit(&units, 15);
        assert!(result.is_some());
        assert_eq!(result.expect("should find unit").name, "func");

        let result = find_containing_unit(&units, 50);
        assert!(result.is_some());
        assert_eq!(result.expect("should find unit").name, "module");

        let result = find_containing_unit(&units, 200);
        assert!(result.is_none());
    }
}
