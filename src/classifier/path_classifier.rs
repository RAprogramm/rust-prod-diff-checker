// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::Path;

/// Checks if path is in examples directory
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// `true` if path is in examples/
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_diff_analyzer::classifier::path_classifier::is_example_path;
///
/// assert!(is_example_path(Path::new("examples/demo.rs")));
/// assert!(!is_example_path(Path::new("src/lib.rs")));
/// ```
pub fn is_example_path(path: &Path) -> bool {
    path.to_string_lossy().contains("examples/")
}

/// Checks if path is in benches directory
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// `true` if path is in benches/
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_diff_analyzer::classifier::path_classifier::is_bench_path;
///
/// assert!(is_bench_path(Path::new("benches/bench.rs")));
/// assert!(!is_bench_path(Path::new("src/lib.rs")));
/// ```
pub fn is_bench_path(path: &Path) -> bool {
    path.to_string_lossy().contains("benches/")
}

/// Checks if path is in tests directory
///
/// # Arguments
///
/// * `path` - Path to check
///
/// # Returns
///
/// `true` if path is in tests/
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_diff_analyzer::classifier::path_classifier::is_test_path;
///
/// assert!(is_test_path(Path::new("tests/integration.rs")));
/// assert!(!is_test_path(Path::new("src/lib.rs")));
/// ```
pub fn is_test_path(path: &Path) -> bool {
    path.to_string_lossy().contains("tests/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_paths() {
        assert!(is_example_path(Path::new("examples/main.rs")));
        assert!(is_example_path(Path::new("crate/examples/demo.rs")));
        assert!(!is_example_path(Path::new("src/examples.rs")));
    }

    #[test]
    fn test_bench_paths() {
        assert!(is_bench_path(Path::new("benches/perf.rs")));
        assert!(!is_bench_path(Path::new("src/benches.rs")));
    }

    #[test]
    fn test_test_paths() {
        assert!(is_test_path(Path::new("tests/integration.rs")));
        assert!(!is_test_path(Path::new("src/tests.rs")));
    }
}
