// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use crate::{config::Config, types::SemanticUnit};

/// Checks if unit is a test function
///
/// # Arguments
///
/// * `unit` - Semantic unit to check
///
/// # Returns
///
/// `true` if unit has #[test] attribute
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     classifier::attr_classifier::is_test_unit,
///     types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
/// };
///
/// let unit = SemanticUnit::new(
///     SemanticUnitKind::Function,
///     "test_it".to_string(),
///     Visibility::Private,
///     LineSpan::new(1, 10),
///     vec!["test".to_string()],
/// );
///
/// assert!(is_test_unit(&unit));
/// ```
pub fn is_test_unit(unit: &SemanticUnit) -> bool {
    unit.has_attribute("test")
}

/// Checks if unit is a benchmark function
///
/// # Arguments
///
/// * `unit` - Semantic unit to check
///
/// # Returns
///
/// `true` if unit has #[bench] attribute
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     classifier::attr_classifier::is_bench_unit,
///     types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
/// };
///
/// let unit = SemanticUnit::new(
///     SemanticUnitKind::Function,
///     "bench_it".to_string(),
///     Visibility::Private,
///     LineSpan::new(1, 10),
///     vec!["bench".to_string()],
/// );
///
/// assert!(is_bench_unit(&unit));
/// ```
pub fn is_bench_unit(unit: &SemanticUnit) -> bool {
    unit.has_attribute("bench")
}

/// Checks if unit is inside a #[cfg(test)] module
///
/// # Arguments
///
/// * `unit` - Semantic unit to check
///
/// # Returns
///
/// `true` if unit has cfg_test marker
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     classifier::attr_classifier::is_in_test_module,
///     types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
/// };
///
/// let unit = SemanticUnit::new(
///     SemanticUnitKind::Function,
///     "helper".to_string(),
///     Visibility::Private,
///     LineSpan::new(1, 10),
///     vec!["cfg_test".to_string()],
/// );
///
/// assert!(is_in_test_module(&unit));
/// ```
pub fn is_in_test_module(unit: &SemanticUnit) -> bool {
    unit.has_attribute("cfg_test")
}

/// Checks if unit has a test-related feature attribute
///
/// # Arguments
///
/// * `unit` - Semantic unit to check
/// * `config` - Configuration with test features
///
/// # Returns
///
/// `true` if unit has a test feature attribute
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     classifier::attr_classifier::has_test_feature,
///     config::Config,
///     types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
/// };
///
/// let unit = SemanticUnit::new(
///     SemanticUnitKind::Function,
///     "mock_fn".to_string(),
///     Visibility::Private,
///     LineSpan::new(1, 10),
///     vec!["cfg".to_string()],
/// );
///
/// let config = Config::default();
/// let result = has_test_feature(&unit, &config);
/// ```
pub fn has_test_feature(unit: &SemanticUnit, config: &Config) -> bool {
    let test_features = config.test_features_set();

    for attr in &unit.attributes {
        if attr.starts_with("cfg") {
            for feature in &test_features {
                if attr.contains(feature) {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LineSpan, SemanticUnitKind, Visibility};

    fn make_unit(attrs: Vec<&str>) -> SemanticUnit {
        SemanticUnit::new(
            SemanticUnitKind::Function,
            "test".to_string(),
            Visibility::Private,
            LineSpan::new(1, 10),
            attrs.into_iter().map(String::from).collect(),
        )
    }

    #[test]
    fn test_is_test_unit() {
        assert!(is_test_unit(&make_unit(vec!["test"])));
        assert!(!is_test_unit(&make_unit(vec!["inline"])));
    }

    #[test]
    fn test_is_bench_unit() {
        assert!(is_bench_unit(&make_unit(vec!["bench"])));
        assert!(!is_bench_unit(&make_unit(vec!["test"])));
    }

    #[test]
    fn test_is_in_test_module() {
        assert!(is_in_test_module(&make_unit(vec!["cfg_test"])));
        assert!(!is_in_test_module(&make_unit(vec!["test"])));
    }
}
