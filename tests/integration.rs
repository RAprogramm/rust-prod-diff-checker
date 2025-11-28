// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::Path;

use rust_diff_analyzer::{
    analysis::{extractor::extract_semantic_units_from_str, map_changes},
    classifier::classify_unit,
    config::Config,
    git::parse_diff,
    output::format_output,
    types::{AnalysisResult, AnalysisScope, CodeType, SemanticUnitKind, Summary},
};

#[test]
fn test_full_analysis_pipeline() {
    let diff = r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,10 @@
+pub fn new_feature() {
+    println!("feature");
+}
+
 pub fn existing() {
     println!("existing");
 }
"#;

    let source = r#"
pub fn new_feature() {
    println!("feature");
}

pub fn existing() {
    println!("existing");
}
"#;

    let config = Config::default();
    let diffs = parse_diff(diff).expect("diff parse failed");

    assert_eq!(diffs.len(), 1);
    assert_eq!(diffs[0].total_added(), 4);

    let result =
        map_changes(&diffs, &config, |_| Ok(source.to_string())).expect("map_changes failed");

    assert!(!result.changes.is_empty());

    let prod_changes: Vec<_> = result
        .changes
        .iter()
        .filter(|c| c.classification.is_production())
        .collect();

    assert!(!prod_changes.is_empty());
}

#[test]
fn test_test_code_classification() {
    let code = r#"
pub fn production_function() {
    println!("prod");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper() {
        println!("helper");
    }

    #[test]
    fn test_something() {
        test_helper();
        assert!(true);
    }
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    let config = Config::default();

    let prod_fn = units
        .iter()
        .find(|u| u.name == "production_function")
        .expect("production_function not found");
    let classification = classify_unit(prod_fn, Path::new("src/lib.rs"), &config);
    assert_eq!(classification, CodeType::Production);

    let test_fn = units
        .iter()
        .find(|u| u.name == "test_something")
        .expect("test_something not found");
    let classification = classify_unit(test_fn, Path::new("src/lib.rs"), &config);
    assert_eq!(classification, CodeType::Test);

    let helper_fn = units
        .iter()
        .find(|u| u.name == "test_helper")
        .expect("test_helper not found");
    let classification = classify_unit(helper_fn, Path::new("src/lib.rs"), &config);
    assert_eq!(classification, CodeType::TestUtility);
}

#[test]
fn test_path_based_classification() {
    let code = "pub fn benchmark_function() {}";
    let units = extract_semantic_units_from_str(code, Path::new("benches/perf.rs"))
        .expect("extraction failed");

    let config = Config::default();
    let unit = &units[0];
    let classification = classify_unit(unit, Path::new("benches/perf.rs"), &config);
    assert_eq!(classification, CodeType::Benchmark);

    let classification = classify_unit(unit, Path::new("tests/integration.rs"), &config);
    assert_eq!(classification, CodeType::Test);

    let classification = classify_unit(unit, Path::new("examples/demo.rs"), &config);
    assert_eq!(classification, CodeType::Example);

    let classification = classify_unit(unit, Path::new("build.rs"), &config);
    assert_eq!(classification, CodeType::BuildScript);
}

#[test]
fn test_multiple_files_diff() {
    let diff = r#"diff --git a/src/a.rs b/src/a.rs
--- a/src/a.rs
+++ b/src/a.rs
@@ -1,1 +1,5 @@
+pub fn func_a() {
+    println!("a");
+}
+
 pub fn old_a() {}
diff --git a/src/b.rs b/src/b.rs
--- a/src/b.rs
+++ b/src/b.rs
@@ -1,1 +1,5 @@
+pub fn func_b() {
+    println!("b");
+}
+
 pub fn old_b() {}
"#;

    let diffs = parse_diff(diff).expect("diff parse failed");
    assert_eq!(diffs.len(), 2);
    assert_eq!(diffs[0].total_added(), 4);
    assert_eq!(diffs[1].total_added(), 4);
}

#[test]
fn test_output_formatting() {
    let result = AnalysisResult::new(
        vec![],
        Summary {
            prod_functions: 3,
            prod_structs: 1,
            prod_other: 2,
            test_units: 5,
            prod_lines_added: 30,
            prod_lines_removed: 10,
            test_lines_added: 50,
            test_lines_removed: 20,
            weighted_score: 15,
            exceeds_limit: false,
        },
        AnalysisScope::new(),
    );

    let config = Config::default();
    let output = format_output(&result, &config).expect("format failed");

    assert!(output.contains("prod_functions_changed=3"));
    assert!(output.contains("prod_structs_changed=1"));
    assert!(output.contains("prod_other_changed=2"));
    assert!(output.contains("test_units_changed=5"));
    assert!(output.contains("prod_lines_added=30"));
    assert!(output.contains("prod_lines_removed=10"));
    assert!(output.contains("weighted_score=15"));
    assert!(output.contains("exceeds_limit=false"));
}

#[test]
fn test_complex_rust_structures() {
    let code = r#"
pub struct Config {
    pub name: String,
    pub value: i32,
}

pub enum Status {
    Active,
    Inactive,
    Pending,
}

pub trait Processor {
    fn process(&self) -> Result<(), Error>;
    fn validate(&self) -> bool;
}

impl Config {
    pub fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }

    pub fn is_valid(&self) -> bool {
        self.value > 0
    }
}

impl Processor for Config {
    fn process(&self) -> Result<(), Error> {
        Ok(())
    }

    fn validate(&self) -> bool {
        self.is_valid()
    }
}

pub type ConfigResult = Result<Config, Error>;

pub const MAX_VALUE: i32 = 1000;

pub static INSTANCE_COUNT: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

macro_rules! config_helper {
    ($name:expr) => {
        Config::new($name.to_string(), 0)
    };
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    let has_struct = units
        .iter()
        .any(|u| u.name == "Config" && matches!(u.kind, SemanticUnitKind::Struct));
    assert!(has_struct, "Config struct not found");

    let has_enum = units
        .iter()
        .any(|u| u.name == "Status" && matches!(u.kind, SemanticUnitKind::Enum));
    assert!(has_enum, "Status enum not found");

    let has_trait = units
        .iter()
        .any(|u| u.name == "Processor" && matches!(u.kind, SemanticUnitKind::Trait));
    assert!(has_trait, "Processor trait not found");

    let has_impl = units
        .iter()
        .any(|u| u.name == "Config" && matches!(u.kind, SemanticUnitKind::Impl));
    assert!(has_impl, "Config impl not found");

    let has_type_alias = units
        .iter()
        .any(|u| u.name == "ConfigResult" && matches!(u.kind, SemanticUnitKind::TypeAlias));
    assert!(has_type_alias, "ConfigResult type alias not found");

    let has_const = units
        .iter()
        .any(|u| u.name == "MAX_VALUE" && matches!(u.kind, SemanticUnitKind::Const));
    assert!(has_const, "MAX_VALUE const not found");

    let has_static = units
        .iter()
        .any(|u| u.name == "INSTANCE_COUNT" && matches!(u.kind, SemanticUnitKind::Static));
    assert!(has_static, "INSTANCE_COUNT static not found");

    let has_macro = units
        .iter()
        .any(|u| u.name == "config_helper" && matches!(u.kind, SemanticUnitKind::Macro));
    assert!(has_macro, "config_helper macro not found");
}

#[test]
fn test_nested_modules() {
    let code = r#"
pub mod outer {
    pub fn outer_func() {}

    pub mod inner {
        pub fn inner_func() {}

        #[cfg(test)]
        mod tests {
            #[test]
            fn test_inner() {}
        }
    }
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "outer"));
    assert!(units.iter().any(|u| u.name == "inner"));
    assert!(units.iter().any(|u| u.name == "outer_func"));
    assert!(units.iter().any(|u| u.name == "inner_func"));
    assert!(units.iter().any(|u| u.name == "test_inner"));
}

#[test]
fn test_async_functions() {
    let code = r#"
pub async fn async_operation() -> Result<(), Error> {
    Ok(())
}

pub async fn fetch_data(url: &str) -> String {
    String::new()
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    assert_eq!(units.len(), 2);
    assert!(units.iter().any(|u| u.name == "async_operation"));
    assert!(units.iter().any(|u| u.name == "fetch_data"));
}

#[test]
fn test_generic_types() {
    let code = r#"
pub struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}

pub fn process<T: Clone>(item: T) -> T {
    item.clone()
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    assert!(units.iter().any(|u| u.name == "Container"));
    assert!(units.iter().any(|u| u.name == "new"));
    assert!(units.iter().any(|u| u.name == "get"));
    assert!(units.iter().any(|u| u.name == "process"));
}

#[test]
fn test_visibility_levels() {
    let code = r#"
pub fn public_func() {}

pub(crate) fn crate_func() {}

pub(super) fn super_func() {}

fn private_func() {}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    let public = units.iter().find(|u| u.name == "public_func").unwrap();
    assert!(public.visibility.is_public());

    let crate_vis = units.iter().find(|u| u.name == "crate_func").unwrap();
    assert!(!crate_vis.visibility.is_public());

    let private = units.iter().find(|u| u.name == "private_func").unwrap();
    assert!(!private.visibility.is_public());
}

#[test]
fn test_empty_diff() {
    let diff = "";
    let diffs = parse_diff(diff).expect("parse failed");
    assert!(diffs.is_empty());
}

#[test]
fn test_non_rust_files_ignored() {
    let diff = r#"diff --git a/README.md b/README.md
--- a/README.md
+++ b/README.md
@@ -1,1 +1,2 @@
 # Project
+New line
diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,1 +1,2 @@
 fn main() {}
+fn new() {}
"#;

    let diffs = parse_diff(diff).expect("parse failed");
    assert_eq!(diffs.len(), 2);

    let rust_files: Vec<_> = diffs.iter().filter(|d| d.is_rust_file()).collect();
    assert_eq!(rust_files.len(), 1);
}

#[test]
fn test_config_limits() {
    let result = AnalysisResult::new(
        vec![],
        Summary {
            prod_functions: 50,
            prod_structs: 10,
            prod_other: 5,
            test_units: 20,
            prod_lines_added: 500,
            prod_lines_removed: 100,
            test_lines_added: 200,
            test_lines_removed: 50,
            weighted_score: 200,
            exceeds_limit: true,
        },
        AnalysisScope::new(),
    );

    assert!(result.summary.exceeds_limit);
    assert_eq!(result.summary.total_prod_units(), 65);
}

#[test]
fn test_trait_impl_for_type() {
    let code = r#"
pub trait Display {
    fn display(&self) -> String;
}

pub struct Item {
    name: String,
}

impl Display for Item {
    fn display(&self) -> String {
        self.name.clone()
    }
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    let trait_impl = units
        .iter()
        .find(|u| u.name.contains("Display for Item"))
        .expect("trait impl not found");
    assert!(matches!(trait_impl.kind, SemanticUnitKind::Impl));
}

#[test]
fn test_multiple_attributes() {
    let code = r#"
#[derive(Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub name: String,
}

#[inline]
#[must_use]
pub fn compute() -> i32 {
    42
}
"#;

    let units =
        extract_semantic_units_from_str(code, Path::new("src/lib.rs")).expect("extraction failed");

    let config = units.iter().find(|u| u.name == "Config").unwrap();
    assert!(config.has_attribute("derive"));

    let compute = units.iter().find(|u| u.name == "compute").unwrap();
    assert!(compute.has_attribute("inline"));
    assert!(compute.has_attribute("must_use"));
}
