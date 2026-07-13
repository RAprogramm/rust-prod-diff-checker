// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, path::Path};

use masterror::AppError;

use super::extractor::extract_semantic_units_from_str;
use crate::{
    classifier::classify_unit,
    config::Config,
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
/// Currently never returns an error: files that fail to read or parse are
/// recorded in the scope as skipped. The `Result` is kept for API stability.
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

        if diff.is_deleted {
            scope.add_skipped(diff.path.clone(), ExclusionReason::Deleted);
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

        let content = match file_reader(&diff.path) {
            Ok(content) => content,
            Err(e) => {
                scope.add_skipped(diff.path.clone(), ExclusionReason::ReadError(e.to_string()));
                continue;
            }
        };

        let units = match extract_semantic_units_from_str(&content, &diff.path) {
            Ok(units) => units,
            Err(e) => {
                scope.add_skipped(
                    diff.path.clone(),
                    ExclusionReason::ParseError(e.to_string()),
                );
                continue;
            }
        };

        scope.add_analyzed(diff.path.clone());

        let added_lines = diff.all_added_lines();
        let removed_positions = diff.all_removed_positions_in_new();

        let mut unit_changes: HashMap<usize, (usize, usize)> = HashMap::new();

        for line in &added_lines {
            if let Some(index) = find_containing_unit_index(&units, *line) {
                let entry = unit_changes.entry(index).or_insert((0, 0));
                entry.0 += 1;
            }
        }

        for line in &removed_positions {
            if let Some(index) = find_containing_unit_index(&units, *line) {
                let entry = unit_changes.entry(index).or_insert((0, 0));
                entry.1 += 1;
            }
        }

        for (index, unit) in units.iter().enumerate() {
            if let Some((added, removed)) = unit_changes.get(&index) {
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

fn find_containing_unit_index(units: &[SemanticUnit], line: usize) -> Option<usize> {
    let mut best_match: Option<usize> = None;

    for (index, unit) in units.iter().enumerate() {
        if unit.span.contains(line) {
            match best_match {
                None => best_match = Some(index),
                Some(current) => {
                    if unit.span.len() < units[current].span.len() {
                        best_match = Some(index);
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
    fn test_find_containing_unit_index() {
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

        assert_eq!(find_containing_unit_index(&units, 15), Some(1));
        assert_eq!(find_containing_unit_index(&units, 50), Some(0));
        assert_eq!(find_containing_unit_index(&units, 200), None);
    }

    #[test]
    fn test_removed_lines_attributed_in_new_coordinates() {
        use std::path::PathBuf;

        use crate::{
            config::Config,
            git::{FileDiff, Hunk, HunkLine},
        };

        let content = "\
fn first() {
    1;
}
fn second() {
    2;
    3;
}
";

        let mut hunk_top = Hunk::new(1, 10, 1, 0);
        for old in 1..=10 {
            hunk_top
                .lines
                .push(HunkLine::removed(old, format!("old prefix {}", old)));
        }

        let mut hunk_inner = Hunk::new(14, 4, 4, 3);
        hunk_inner
            .lines
            .push(HunkLine::context(14, 4, "fn second() {".to_string()));
        hunk_inner
            .lines
            .push(HunkLine::context(15, 5, "    2;".to_string()));
        hunk_inner
            .lines
            .push(HunkLine::removed(16, "    gone();".to_string()));
        hunk_inner
            .lines
            .push(HunkLine::context(17, 6, "    3;".to_string()));

        let mut diff = FileDiff::new(PathBuf::from("src/prod.rs"));
        diff.hunks = vec![hunk_top, hunk_inner];

        let config = Config::default();
        let result =
            map_changes(&[diff], &config, |_| Ok(content.to_string())).expect("map should work");

        let second = result
            .changes
            .iter()
            .find(|c| c.unit.name == "second")
            .expect("removal inside second must be attributed to second");
        assert_eq!(second.lines_removed, 1);

        let first = result
            .changes
            .iter()
            .find(|c| c.unit.name == "first")
            .expect("prefix removals must be attributed to the adjacent unit");
        assert_eq!(first.lines_removed, 10);
    }

    #[test]
    fn test_struct_and_impl_with_same_name_not_double_counted() {
        use std::path::PathBuf;

        use crate::{
            config::Config,
            git::{FileDiff, Hunk, HunkLine},
        };

        let content = "\
pub struct Foo {
    pub value: u32,
}

impl Foo {
    pub fn get(&self) -> u32 {
        self.value
    }
}
";

        let mut hunk = Hunk::new(1, 2, 1, 3);
        hunk.lines
            .push(HunkLine::context(1, 1, "pub struct Foo {".to_string()));
        hunk.lines
            .push(HunkLine::added(2, "    pub value: u32,".to_string()));
        hunk.lines.push(HunkLine::context(2, 3, "}".to_string()));

        let mut diff = FileDiff::new(PathBuf::from("src/prod.rs"));
        diff.hunks = vec![hunk];

        let config = Config::default();
        let result =
            map_changes(&[diff], &config, |_| Ok(content.to_string())).expect("map should work");

        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.changes[0].unit.name, "Foo");
        assert_eq!(result.changes[0].lines_added, 1);
    }

    #[test]
    fn test_unreadable_file_is_skipped_not_fatal() {
        use std::{io, path::PathBuf};

        use crate::{config::Config, git::FileDiff};

        let diff = FileDiff::new(PathBuf::from("src/missing.rs"));
        let config = Config::default();

        let result = map_changes(&[diff], &config, |_| {
            Err(io::Error::new(io::ErrorKind::NotFound, "no such file"))
        })
        .expect("read failure must not abort analysis");

        assert!(result.changes.is_empty());
        assert!(result.scope.analyzed_files.is_empty());
        assert_eq!(result.scope.error_count(), 1);
        assert!(matches!(
            result.scope.skipped_files[0].reason,
            ExclusionReason::ReadError(_)
        ));
    }

    #[test]
    fn test_unparsable_file_is_skipped_not_fatal() {
        use std::path::PathBuf;

        use crate::{config::Config, git::FileDiff};

        let diff = FileDiff::new(PathBuf::from("src/broken.rs"));
        let config = Config::default();

        let result = map_changes(&[diff], &config, |_| Ok("fn broken( {{{".to_string()))
            .expect("parse failure must not abort analysis");

        assert!(result.changes.is_empty());
        assert_eq!(result.scope.error_count(), 1);
        assert!(matches!(
            result.scope.skipped_files[0].reason,
            ExclusionReason::ParseError(_)
        ));
    }

    #[test]
    fn test_deleted_file_is_skipped_without_reading() {
        use std::{io, path::PathBuf};

        use crate::{config::Config, git::FileDiff};

        let mut diff = FileDiff::new(PathBuf::from("src/gone.rs"));
        diff.is_deleted = true;
        let config = Config::default();

        let result = map_changes(&[diff], &config, |path| {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("must not be read: {}", path.display()),
            ))
        })
        .expect("deleted file must not abort analysis");

        assert!(result.changes.is_empty());
        assert_eq!(result.scope.skipped_files.len(), 1);
        assert_eq!(
            result.scope.skipped_files[0].reason,
            ExclusionReason::Deleted
        );
    }
}
