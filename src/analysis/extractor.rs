// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{fs, path::Path};

use masterror::AppError;

use super::ast_visitor::SemanticUnitVisitor;
use crate::{
    error::{FileReadError, ParseError},
    types::SemanticUnit,
};

/// Extracts semantic units from a Rust source file
///
/// # Arguments
///
/// * `path` - Path to the Rust source file
///
/// # Returns
///
/// Vector of semantic units or error
///
/// # Errors
///
/// Returns error if file cannot be read or parsed
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// use rust_diff_analyzer::analysis::extract_semantic_units;
///
/// let units = extract_semantic_units(Path::new("src/lib.rs"));
/// ```
pub fn extract_semantic_units(path: &Path) -> Result<Vec<SemanticUnit>, AppError> {
    let content =
        fs::read_to_string(path).map_err(|e| AppError::from(FileReadError::new(path, e)))?;

    extract_semantic_units_from_str(&content, path)
}

/// Extracts semantic units from Rust source code string
///
/// # Arguments
///
/// * `content` - Rust source code as string
/// * `path` - Path for error reporting
///
/// # Returns
///
/// Vector of semantic units or error
///
/// # Errors
///
/// Returns error if code cannot be parsed
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_diff_analyzer::analysis::extractor::extract_semantic_units_from_str;
///
/// let code = "fn main() {}";
/// let units = extract_semantic_units_from_str(code, Path::new("main.rs")).unwrap();
/// assert_eq!(units.len(), 1);
/// ```
pub fn extract_semantic_units_from_str(
    content: &str,
    path: &Path,
) -> Result<Vec<SemanticUnit>, AppError> {
    let file = syn::parse_file(content)
        .map_err(|e| AppError::from(ParseError::new(path, e.to_string())))?;

    Ok(SemanticUnitVisitor::extract(&file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_from_str() {
        let code = r#"
            pub fn main() {}
            struct Config {}
            impl Config {
                pub fn new() -> Self { Config {} }
            }
        "#;

        let units = extract_semantic_units_from_str(code, Path::new("test.rs"))
            .expect("extraction should succeed");

        assert!(units.len() >= 3);
    }

    #[test]
    fn test_parse_error() {
        let bad_code = "fn broken( {}";
        let result = extract_semantic_units_from_str(bad_code, Path::new("bad.rs"));
        assert!(result.is_err());
    }
}
