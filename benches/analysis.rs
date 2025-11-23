// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{hint::black_box, path::Path};

use criterion::{Criterion, criterion_group, criterion_main};
use rust_diff_analyzer::{analysis::extractor::extract_semantic_units_from_str, git::parse_diff};

fn bench_parse_diff(c: &mut Criterion) {
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

    c.bench_function("parse_diff", |b| b.iter(|| parse_diff(black_box(diff))));
}

fn bench_extract_units(c: &mut Criterion) {
    let code = r#"
pub struct Config {
    pub name: String,
    pub value: i32,
}

pub enum Status {
    Active,
    Inactive,
}

pub trait Processor {
    fn process(&self) -> Result<(), Error>;
}

impl Config {
    pub fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }
}

pub fn process_data(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new("test".to_string(), 42);
        assert_eq!(config.value, 42);
    }
}
"#;

    c.bench_function("extract_semantic_units", |b| {
        b.iter(|| {
            extract_semantic_units_from_str(black_box(code), black_box(Path::new("src/lib.rs")))
        })
    });
}

fn bench_large_diff(c: &mut Criterion) {
    let mut diff = String::from(
        r#"diff --git a/src/lib.rs b/src/lib.rs
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,1 +1,100 @@
"#,
    );

    for i in 0..100 {
        diff.push_str(&format!("+pub fn func_{}() {{}}\n", i));
    }

    c.bench_function("parse_large_diff", |b| {
        b.iter(|| parse_diff(black_box(&diff)))
    });
}

criterion_group!(
    benches,
    bench_parse_diff,
    bench_extract_units,
    bench_large_diff
);
criterion_main!(benches);
