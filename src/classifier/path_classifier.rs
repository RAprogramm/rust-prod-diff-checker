// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::Path;

/// Checks whether a path matches a `/`-separated pattern by whole components
///
/// The pattern's components must appear consecutively in the path. A trailing
/// slash requires the match to be a directory, i.e. at least one more path
/// component must follow. Substrings never match partial components, so
/// `tests/` does not match `src/attests/mod.rs` and `src/gen` does not match
/// `src/generic.rs`.
///
/// # Arguments
///
/// * `path` - Path to check
/// * `pattern` - `/`-separated component pattern
///
/// # Returns
///
/// `true` if the pattern's components appear consecutively in the path
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_diff_analyzer::classifier::path_classifier::path_matches_pattern;
///
/// assert!(path_matches_pattern(
///     Path::new("tests/integration.rs"),
///     "tests/"
/// ));
/// assert!(path_matches_pattern(
///     Path::new("crate/tests/it.rs"),
///     "tests/"
/// ));
/// assert!(!path_matches_pattern(
///     Path::new("src/attests/mod.rs"),
///     "tests/"
/// ));
/// assert!(!path_matches_pattern(
///     Path::new("src/generic.rs"),
///     "src/gen"
/// ));
/// ```
pub fn path_matches_pattern(path: &Path, pattern: &str) -> bool {
    let requires_directory = pattern.ends_with('/');
    let pattern_components: Vec<&str> = pattern.split('/').filter(|c| !c.is_empty()).collect();
    if pattern_components.is_empty() {
        return false;
    }

    let components: Vec<String> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    let needed = pattern_components.len();
    if components.len() < needed {
        return false;
    }

    for start in 0..=(components.len() - needed) {
        let matches = components[start..start + needed]
            .iter()
            .zip(&pattern_components)
            .all(|(component, pattern_component)| component == pattern_component);
        if matches && (!requires_directory || start + needed < components.len()) {
            return true;
        }
    }

    false
}

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
    path_matches_pattern(path, "examples/")
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
    path_matches_pattern(path, "benches/")
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
    path_matches_pattern(path, "tests/")
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

    #[test]
    fn test_component_boundaries() {
        assert!(!is_test_path(Path::new("src/attests/mod.rs")));
        assert!(!is_test_path(Path::new("src/contests/x.rs")));
        assert!(!is_bench_path(Path::new("src/benches_util/x.rs")));
        assert!(is_test_path(Path::new("crate/tests/deep/it.rs")));
    }

    #[test]
    fn test_pattern_matching_rules() {
        assert!(path_matches_pattern(Path::new("src/gen/out.rs"), "src/gen"));
        assert!(!path_matches_pattern(
            Path::new("src/generic.rs"),
            "src/gen"
        ));
        assert!(path_matches_pattern(
            Path::new("src/gen/out.rs"),
            "src/gen/"
        ));
        assert!(!path_matches_pattern(Path::new("a/tests"), "tests/"));
        assert!(!path_matches_pattern(Path::new("src/lib.rs"), ""));
    }
}
