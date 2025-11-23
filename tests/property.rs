// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::Path;

use proptest::prelude::*;
use rust_diff_analyzer::{
    analysis::extractor::extract_semantic_units_from_str,
    config::Config,
    git::parse_diff,
    types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility},
};

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn valid_identifier() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{2,30}"
        .prop_filter_map("filter keywords", |s| {
            if RUST_KEYWORDS.contains(&s.as_str()) {
                None
            } else {
                Some(s)
            }
        })
}

proptest! {
    #[test]
    fn test_linespan_contains_within_bounds(start in 1usize..1000, len in 1usize..100) {
        let end = start + len;
        let span = LineSpan::new(start, end);

        for line in start..=end {
            prop_assert!(span.contains(line));
        }

        prop_assert!(!span.contains(start - 1));
        prop_assert!(!span.contains(end + 1));
    }

    #[test]
    fn test_linespan_len_calculation(start in 1usize..1000, len in 1usize..100) {
        let end = start + len;
        let span = LineSpan::new(start, end);
        prop_assert_eq!(span.len(), len + 1);
    }

    #[test]
    fn test_valid_function_always_parses(name in valid_identifier()) {
        let code = format!("pub fn {}() {{}}", name);
        let result = extract_semantic_units_from_str(&code, Path::new("test.rs"));
        prop_assert!(result.is_ok());

        let units = result.ok();
        prop_assert!(units.is_some_and(|u| u.len() == 1));
    }

    #[test]
    fn test_valid_struct_always_parses(name in valid_identifier()) {
        let code = format!("pub struct {} {{}}", name);
        let result = extract_semantic_units_from_str(&code, Path::new("test.rs"));
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_valid_enum_always_parses(name in valid_identifier()) {
        let code = format!("pub enum {} {{ A, B }}", name);
        let result = extract_semantic_units_from_str(&code, Path::new("test.rs"));
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_diff_line_numbers_are_positive(
        old_start in 1usize..1000,
        old_count in 1usize..100,
        new_start in 1usize..1000,
        new_count in 1usize..100
    ) {
        let diff = format!(
            r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -{},{} +{},{} @@
 context line
"#,
            old_start, old_count, new_start, new_count
        );

        let result = parse_diff(&diff);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_config_default_is_always_valid(_seed in 0u64..1000) {
        let config = Config::default();
        prop_assert!(config.validate().is_ok());
    }

    #[test]
    fn test_multiple_functions_parse_correctly(count in 1usize..20) {
        let mut code = String::new();
        for i in 0..count {
            code.push_str(&format!("pub fn func_{}() {{}}\n", i));
        }

        let result = extract_semantic_units_from_str(&code, Path::new("test.rs"));
        prop_assert!(result.is_ok());

        let units = result.ok();
        prop_assert!(units.is_some_and(|u| u.len() == count));
    }

    #[test]
    fn test_semantic_unit_creation(
        name in valid_identifier(),
        start in 1usize..1000,
        len in 1usize..100
    ) {
        let span = LineSpan::new(start, start + len);
        let unit = SemanticUnit::new(
            SemanticUnitKind::Function,
            name.clone(),
            Visibility::Public,
            span,
            vec![],
        );

        prop_assert_eq!(unit.name, name);
        prop_assert_eq!(unit.kind, SemanticUnitKind::Function);
        prop_assert_eq!(unit.visibility, Visibility::Public);
    }

    #[test]
    fn test_diff_preserves_file_count(file_count in 1usize..10) {
        let mut diff = String::new();

        for i in 0..file_count {
            diff.push_str(&format!(
                r#"diff --git a/src/file_{}.rs b/src/file_{}.rs
--- a/src/file_{}.rs
+++ b/src/file_{}.rs
@@ -1,1 +1,2 @@
 fn existing() {{}}
+fn added() {{}}
"#,
                i, i, i, i
            ));
        }

        let result = parse_diff(&diff);
        prop_assert!(result.is_ok());

        let files = result.ok();
        prop_assert!(files.is_some_and(|f| f.len() == file_count));
    }
}
