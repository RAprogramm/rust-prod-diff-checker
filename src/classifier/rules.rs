// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use crate::{
    config::Config,
    types::{SemanticUnit, SemanticUnitKind},
};

/// Calculates the weight score for a semantic unit
///
/// # Arguments
///
/// * `unit` - Semantic unit to calculate weight for
/// * `config` - Configuration with weight settings
///
/// # Returns
///
/// Weight score for the unit
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{
///     classifier::rules::calculate_weight,
///     config::Config,
///     types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
/// };
///
/// let unit = SemanticUnit::new(
///     SemanticUnitKind::Function,
///     "public_fn".to_string(),
///     Visibility::Public,
///     LineSpan::new(1, 10),
///     vec![],
/// );
///
/// let config = Config::default();
/// let weight = calculate_weight(&unit, &config);
/// assert_eq!(weight, 3); // default public function weight
/// ```
pub fn calculate_weight(unit: &SemanticUnit, config: &Config) -> usize {
    let weights = &config.weights;

    match unit.kind {
        SemanticUnitKind::Function => {
            if unit.visibility.is_public() {
                weights.public_function
            } else {
                weights.private_function
            }
        }
        SemanticUnitKind::Struct | SemanticUnitKind::Enum => {
            if unit.visibility.is_public() {
                weights.public_struct
            } else {
                weights.private_struct
            }
        }
        SemanticUnitKind::Impl => weights.impl_block,
        SemanticUnitKind::Trait => weights.trait_definition,
        SemanticUnitKind::Const | SemanticUnitKind::Static => weights.const_static,
        SemanticUnitKind::TypeAlias => weights.const_static,
        SemanticUnitKind::Macro => weights.private_function,
        SemanticUnitKind::Module => weights.const_static,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LineSpan, Visibility};

    #[test]
    fn test_weight_calculation() {
        let config = Config::default();

        let pub_fn = SemanticUnit::new(
            SemanticUnitKind::Function,
            "public".to_string(),
            Visibility::Public,
            LineSpan::new(1, 10),
            vec![],
        );
        assert_eq!(calculate_weight(&pub_fn, &config), 3);

        let priv_fn = SemanticUnit::new(
            SemanticUnitKind::Function,
            "private".to_string(),
            Visibility::Private,
            LineSpan::new(1, 10),
            vec![],
        );
        assert_eq!(calculate_weight(&priv_fn, &config), 1);

        let trait_def = SemanticUnit::new(
            SemanticUnitKind::Trait,
            "MyTrait".to_string(),
            Visibility::Public,
            LineSpan::new(1, 10),
            vec![],
        );
        assert_eq!(calculate_weight(&trait_def, &config), 4);
    }
}
