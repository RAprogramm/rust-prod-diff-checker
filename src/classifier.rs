// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

pub mod attr_classifier;
pub mod path_classifier;
pub mod rules;

use std::path::Path;

use crate::{
    config::Config,
    types::{CodeType, SemanticUnit},
};

/// Classifies a semantic unit as production or test code
///
/// # Arguments
///
/// * `unit` - The semantic unit to classify
/// * `path` - Path to the file containing the unit
/// * `config` - Configuration
///
/// # Returns
///
/// Classification of the code
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_diff_analyzer::{
///     classifier::classify_unit,
///     config::Config,
///     types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
/// };
///
/// let unit = SemanticUnit::new(
///     SemanticUnitKind::Function,
///     "test_something".to_string(),
///     Visibility::Private,
///     LineSpan::new(1, 10),
///     vec!["test".to_string()],
/// );
///
/// let config = Config::default();
/// let classification = classify_unit(&unit, Path::new("src/lib.rs"), &config);
/// assert!(classification == rust_diff_analyzer::types::CodeType::Test);
/// ```
pub fn classify_unit(unit: &SemanticUnit, path: &Path, config: &Config) -> CodeType {
    if config.is_build_script(path) {
        return CodeType::BuildScript;
    }

    if path_classifier::is_example_path(path) {
        return CodeType::Example;
    }

    if path_classifier::is_bench_path(path) {
        return CodeType::Benchmark;
    }

    if config.is_test_path(path) {
        return CodeType::Test;
    }

    if attr_classifier::is_bench_unit(unit) {
        return CodeType::Benchmark;
    }

    if attr_classifier::is_test_unit(unit) {
        return CodeType::Test;
    }

    if attr_classifier::is_in_test_module(unit) {
        return CodeType::TestUtility;
    }

    if attr_classifier::has_test_feature(unit, config) {
        return CodeType::TestUtility;
    }

    CodeType::Production
}
