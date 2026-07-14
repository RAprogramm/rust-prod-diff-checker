// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use crate::{
    config::Config,
    types::{Change, SemanticUnit, SemanticUnitKind},
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

/// Returns every production unit kind exceeding its per-type limit
///
/// # Arguments
///
/// * `changes` - Analyzed changes
/// * `config` - Configuration with optional per-type limits
///
/// # Returns
///
/// Tuples of `(kind name, count, limit)` for each exceeded limit; empty when
/// no per-type limits are configured or none are exceeded
///
/// # Examples
///
/// ```
/// use rust_diff_analyzer::{classifier::rules::exceeded_per_type_limits, config::Config};
///
/// let config = Config::default();
/// assert!(exceeded_per_type_limits(&[], &config).is_empty());
/// ```
pub fn exceeded_per_type_limits(
    changes: &[Change],
    config: &Config,
) -> Vec<(&'static str, usize, usize)> {
    let per_type = match &config.limits.per_type {
        Some(limits) => limits,
        None => return Vec::new(),
    };

    let kinds = [
        SemanticUnitKind::Function,
        SemanticUnitKind::Struct,
        SemanticUnitKind::Enum,
        SemanticUnitKind::Trait,
        SemanticUnitKind::Impl,
        SemanticUnitKind::Const,
        SemanticUnitKind::Static,
        SemanticUnitKind::TypeAlias,
        SemanticUnitKind::Macro,
        SemanticUnitKind::Module,
    ];
    let limits = [
        per_type.functions,
        per_type.structs,
        per_type.enums,
        per_type.traits,
        per_type.impl_blocks,
        per_type.consts,
        per_type.statics,
        per_type.type_aliases,
        per_type.macros,
        per_type.modules,
    ];

    let mut counts = [0usize; 10];
    for change in changes {
        if !change.classification.is_production() {
            continue;
        }
        if let Some(index) = kinds.iter().position(|k| *k == change.unit.kind) {
            counts[index] += 1;
        }
    }

    kinds
        .iter()
        .zip(limits)
        .zip(counts)
        .filter_map(|((kind, limit), count)| {
            limit.and_then(|limit| (count > limit).then_some((kind.as_str(), count, limit)))
        })
        .collect()
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

    #[test]
    fn test_exceeded_per_type_limits() {
        use std::path::PathBuf;

        use crate::{
            config::PerTypeLimits,
            types::{Change, CodeType},
        };

        let make_change = |kind: SemanticUnitKind| {
            Change::new(
                PathBuf::from("src/lib.rs"),
                SemanticUnit::new(
                    kind,
                    "unit".to_string(),
                    Visibility::Public,
                    LineSpan::new(1, 5),
                    vec![],
                ),
                CodeType::Production,
                3,
                0,
            )
        };

        let mut config = Config::default();
        config.limits.per_type = Some(PerTypeLimits {
            functions: Some(1),
            ..PerTypeLimits::default()
        });

        let changes = vec![
            make_change(SemanticUnitKind::Function),
            make_change(SemanticUnitKind::Function),
            make_change(SemanticUnitKind::Struct),
        ];

        let exceeded = exceeded_per_type_limits(&changes, &config);
        assert_eq!(exceeded, vec![("function", 2, 1)]);

        config.limits.per_type = None;
        assert!(exceeded_per_type_limits(&changes, &config).is_empty());
    }
}
