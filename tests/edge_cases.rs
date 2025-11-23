// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::Path;

use rust_diff_analyzer::analysis::extractor::extract_semantic_units_from_str;
use rust_diff_analyzer::classifier::classify_unit;
use rust_diff_analyzer::config::{Config, ConfigBuilder};
use rust_diff_analyzer::git::parse_diff;
use rust_diff_analyzer::types::CodeType;

#[test]
fn test_diff_with_only_removals() {
    let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,5 +1,2 @@
 fn main() {}
-fn removed_func() {
-    println!("removed");
-}
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    assert_eq!(diffs.len(), 1);
    assert_eq!(diffs[0].total_removed(), 3);
    assert_eq!(diffs[0].total_added(), 0);
}

#[test]
fn test_diff_with_context_only() {
    let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,3 @@
 fn main() {
-    old_code();
+    new_code();
 }
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    assert_eq!(diffs[0].total_added(), 1);
    assert_eq!(diffs[0].total_removed(), 1);
}

#[test]
fn test_renamed_file() {
    let diff = r#"diff --git a/src/old.rs b/src/new.rs
similarity index 90%
rename from src/old.rs
rename to src/new.rs
--- a/src/old.rs
+++ b/src/new.rs
@@ -1,1 +1,2 @@
 fn main() {}
+fn added() {}
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    assert_eq!(diffs.len(), 1);
}

#[test]
fn test_binary_file_skipped() {
    let diff = r#"diff --git a/image.png b/image.png
Binary files differ
diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,1 +1,2 @@
 fn main() {}
+fn new() {}
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    let rust_files: Vec<_> = diffs.iter().filter(|d| d.is_rust_file()).collect();
    assert_eq!(rust_files.len(), 1);
}

#[test]
fn test_empty_function() {
    let code = "pub fn empty() {}";
    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "empty");
}

#[test]
fn test_function_with_many_arguments() {
    let code = r#"
pub fn many_args(
    a: i32,
    b: String,
    c: Vec<u8>,
    d: Option<bool>,
    e: Result<(), Error>,
) -> i32 {
    a
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "many_args");
}

#[test]
fn test_deeply_nested_code() {
    let code = r#"
pub mod a {
    pub mod b {
        pub mod c {
            pub mod d {
                pub fn deep_func() {}
            }
        }
    }
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "deep_func"));
}

#[test]
fn test_impl_with_lifetimes() {
    let code = r#"
pub struct Borrowed<'a> {
    data: &'a str,
}

impl<'a> Borrowed<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }

    pub fn get(&self) -> &str {
        self.data
    }
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "Borrowed"));
    assert!(units.iter().any(|u| u.name == "new"));
    assert!(units.iter().any(|u| u.name == "get"));
}

#[test]
fn test_where_clauses() {
    let code = r#"
pub fn complex_generic<T, U>(a: T, b: U) -> T
where
    T: Clone + Send + Sync,
    U: Into<T>,
{
    a.clone()
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "complex_generic");
}

#[test]
fn test_unsafe_code() {
    let code = r#"
pub unsafe fn dangerous() {
    std::ptr::null::<i32>();
}

pub fn safe_wrapper() {
    unsafe {
        dangerous();
    }
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "dangerous"));
    assert!(units.iter().any(|u| u.name == "safe_wrapper"));
}

#[test]
fn test_extern_functions() {
    let code = r#"
extern "C" {
    fn external_func();
}

pub extern "C" fn exported_func() -> i32 {
    42
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "exported_func"));
}

#[test]
fn test_custom_test_features() {
    let config = ConfigBuilder::new()
        .add_test_feature("test-utils")
        .add_test_feature("mock")
        .build();

    let features = config.test_features_set();
    assert!(features.contains("test-utils"));
    assert!(features.contains("mock"));
}

#[test]
fn test_ignore_paths() {
    let config = ConfigBuilder::new()
        .add_ignore_path("fixtures/")
        .add_ignore_path("snapshots/")
        .build();

    assert!(config.should_ignore(Path::new("fixtures/test.rs")));
    assert!(config.should_ignore(Path::new("tests/snapshots/data.rs")));
    assert!(!config.should_ignore(Path::new("src/lib.rs")));
}

#[test]
fn test_doc_attributes() {
    let code = r#"
/// Documentation for function
///
/// # Examples
///
/// ```
/// let x = documented();
/// ```
pub fn documented() -> i32 {
    42
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert_eq!(units.len(), 1);
}

#[test]
fn test_conditional_compilation() {
    let code = r#"
#[cfg(target_os = "linux")]
pub fn linux_only() {}

#[cfg(target_os = "windows")]
pub fn windows_only() {}

#[cfg(all(feature = "full", not(feature = "minimal")))]
pub fn full_feature() {}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "linux_only"));
    assert!(units.iter().any(|u| u.name == "windows_only"));
    assert!(units.iter().any(|u| u.name == "full_feature"));
}

#[test]
fn test_associated_types() {
    let code = r#"
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "Iterator"));
    assert!(units.iter().any(|u| u.name == "next"));
}

#[test]
fn test_default_trait_impl() {
    let code = r#"
pub trait Default {
    fn default() -> Self;

    fn is_default(&self) -> bool {
        false
    }
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "default"));
    assert!(units.iter().any(|u| u.name == "is_default"));
}

#[test]
fn test_tuple_structs() {
    let code = r#"
pub struct Point(pub i32, pub i32);

pub struct Wrapper<T>(T);
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "Point"));
    assert!(units.iter().any(|u| u.name == "Wrapper"));
}

#[test]
fn test_unit_struct() {
    let code = "pub struct Empty;";

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "Empty");
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();
    assert!(config.validate().is_ok());

    config.limits.max_prod_units = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_hunk_with_no_context() {
    let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1 +1,3 @@
+fn new1() {}
+fn new2() {}
 fn existing() {}
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    assert_eq!(diffs[0].total_added(), 2);
}

#[test]
fn test_multiple_hunks_same_file() {
    let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,2 +1,3 @@
 fn first() {}
+fn added_first() {}
 fn second() {}
@@ -10,2 +11,3 @@
 fn tenth() {}
+fn added_tenth() {}
 fn eleventh() {}
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    assert_eq!(diffs[0].hunks.len(), 2);
    assert_eq!(diffs[0].total_added(), 2);
}

#[test]
fn test_bench_attribute() {
    let code = r#"
#[bench]
fn benchmark_operation(b: &mut Bencher) {
    b.iter(|| {
        operation()
    });
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    let bench = units.iter().find(|u| u.name == "benchmark_operation").unwrap();
    assert!(bench.has_attribute("bench"));

    let config = Config::default();
    let classification = classify_unit(bench, Path::new("src/lib.rs"), &config);
    assert_eq!(classification, CodeType::Benchmark);
}

#[test]
fn test_inner_attributes() {
    let code = r#"
#![allow(dead_code)]

pub fn allowed_dead() {}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "allowed_dead"));
}

#[test]
fn test_closure_in_function() {
    let code = r#"
pub fn with_closure() {
    let closure = |x: i32| x + 1;
    closure(5);
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "with_closure");
}

#[test]
fn test_const_generics() {
    let code = r#"
pub struct Array<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Array<N> {
    pub fn new() -> Self {
        Self { data: [0; N] }
    }
}
"#;

    let units = extract_semantic_units_from_str(code, Path::new("src/lib.rs"))
        .expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "Array"));
    assert!(units.iter().any(|u| u.name == "new"));
}
