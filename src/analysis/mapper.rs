use std::collections::HashMap;
use std::path::Path;

use crate::classifier::classify_unit;
use crate::config::Config;
use crate::error::AppError;
use crate::git::FileDiff;
use crate::types::{Change, SemanticUnit};

use super::extractor::extract_semantic_units_from_str;

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
/// Vector of changes or error
///
/// # Errors
///
/// Returns error if file reading or parsing fails
///
/// # Examples
///
/// ```no_run
/// use rust_diff_analyzer::analysis::map_changes;
/// use rust_diff_analyzer::config::Config;
/// use std::fs;
///
/// let diffs = vec![];
/// let config = Config::default();
/// let changes = map_changes(&diffs, &config, |p| fs::read_to_string(p));
/// ```
pub fn map_changes<F>(
    diffs: &[FileDiff],
    config: &Config,
    file_reader: F,
) -> Result<Vec<Change>, AppError>
where
    F: Fn(&Path) -> Result<String, std::io::Error>,
{
    let mut changes = Vec::new();

    for diff in diffs {
        if !diff.is_rust_file() {
            continue;
        }

        if config.should_ignore(&diff.path) {
            continue;
        }

        let content = file_reader(&diff.path).map_err(|e| AppError::FileRead {
            path: diff.path.clone(),
            source: e,
        })?;

        let units = extract_semantic_units_from_str(&content, &diff.path)?;

        let added_lines = diff.all_added_lines();
        let removed_lines = diff.all_removed_lines();

        let mut unit_changes: HashMap<String, (usize, usize)> = HashMap::new();

        for line in &added_lines {
            if let Some(unit) = find_containing_unit(&units, *line) {
                let entry = unit_changes
                    .entry(unit.name.clone())
                    .or_insert((0, 0));
                entry.0 += 1;
            }
        }

        for line in &removed_lines {
            if let Some(unit) = find_containing_unit(&units, *line) {
                let entry = unit_changes
                    .entry(unit.name.clone())
                    .or_insert((0, 0));
                entry.1 += 1;
            }
        }

        for unit in &units {
            if let Some((added, removed)) = unit_changes.get(&unit.name) {
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

    Ok(changes)
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
