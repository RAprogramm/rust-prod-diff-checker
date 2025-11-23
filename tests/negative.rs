// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::Path;

use rust_diff_analyzer::{
    analysis::extractor::extract_semantic_units_from_str, config::Config, git::parse_diff,
};

#[test]
fn test_invalid_rust_syntax() {
    let invalid_code = "fn broken( {}";
    let result = extract_semantic_units_from_str(invalid_code, Path::new("test.rs"));
    assert!(result.is_err());
}

#[test]
fn test_empty_diff() {
    let empty_diff = "";
    let result = parse_diff(empty_diff);
    assert!(result.is_ok());
    assert!(result.as_ref().is_ok_and(|files| files.is_empty()));
}

#[test]
fn test_malformed_diff_header() {
    let bad_diff = "diff --git";
    let result = parse_diff(bad_diff);
    assert!(result.is_err());
}

#[test]
fn test_invalid_hunk_header() {
    let bad_diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ invalid @@
"#;
    let result = parse_diff(bad_diff);
    assert!(result.is_err());
}

#[test]
fn test_config_validation_zero_max_units() {
    let mut config = Config::default();
    config.limits.max_prod_units = 0;
    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_config_validation_zero_max_score() {
    let mut config = Config::default();
    config.limits.max_weighted_score = 0;
    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_incomplete_function() {
    let code = "pub fn incomplete(";
    let result = extract_semantic_units_from_str(code, Path::new("test.rs"));
    assert!(result.is_err());
}

#[test]
fn test_missing_closing_brace() {
    let code = r#"
        pub fn test() {
            let x = 1;
    "#;
    let result = extract_semantic_units_from_str(code, Path::new("test.rs"));
    assert!(result.is_err());
}

#[test]
fn test_invalid_attribute() {
    let code = r#"
        #[invalid syntax here
        pub fn test() {}
    "#;
    let result = extract_semantic_units_from_str(code, Path::new("test.rs"));
    assert!(result.is_err());
}

#[test]
fn test_diff_with_binary_markers() {
    let diff = r#"diff --git a/image.png b/image.png
Binary files differ
"#;
    let result = parse_diff(diff);
    assert!(result.is_ok());
}

#[test]
fn test_diff_with_no_newline_at_eof() {
    let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,1 +1,2 @@
 fn main() {}
+fn test() {}
\ No newline at end of file
"#;
    let result = parse_diff(diff);
    assert!(result.is_ok());
}

#[test]
fn test_deeply_nested_invalid_syntax() {
    let code = r#"
        mod outer {
            mod inner {
                pub fn valid() {}
                pub fn invalid( {
                }
            }
        }
    "#;
    let result = extract_semantic_units_from_str(code, Path::new("test.rs"));
    assert!(result.is_err());
}

#[test]
fn test_unicode_in_identifiers() {
    let code = r#"
        pub fn функция() {}
        pub struct Структура {}
    "#;
    let result = extract_semantic_units_from_str(code, Path::new("test.rs"));
    assert!(result.is_ok());
}

#[test]
fn test_extremely_long_function_name() {
    let name = "a".repeat(1000);
    let code = format!("pub fn {}() {{}}", name);
    let result = extract_semantic_units_from_str(&code, Path::new("test.rs"));
    assert!(result.is_ok());
}

#[test]
fn test_diff_with_renamed_file() {
    let diff = r#"diff --git a/old.rs b/new.rs
similarity index 90%
rename from old.rs
rename to new.rs
--- a/old.rs
+++ b/new.rs
@@ -1,1 +1,2 @@
 fn main() {}
+fn added() {}
"#;
    let result = parse_diff(diff);
    assert!(result.is_ok());
    let files = result.as_ref().ok();
    assert!(files.is_some_and(|f| !f.is_empty()));
}
