// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, fs, path::Path};

use masterror::AppError;
use serde::{Deserialize, Serialize};

use crate::error::{ConfigError, ConfigValidationError, FileReadError};

/// Classification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationConfig {
    /// Features that indicate test code
    #[serde(default = "default_test_features")]
    pub test_features: Vec<String>,
    /// Paths that contain test code
    #[serde(default = "default_test_paths")]
    pub test_paths: Vec<String>,
    /// Paths to ignore completely
    #[serde(default)]
    pub ignore_paths: Vec<String>,
}

impl Default for ClassificationConfig {
    fn default() -> Self {
        Self {
            test_features: default_test_features(),
            test_paths: default_test_paths(),
            ignore_paths: Vec::new(),
        }
    }
}

fn default_test_features() -> Vec<String> {
    vec![
        "test-utils".to_string(),
        "testing".to_string(),
        "mock".to_string(),
    ]
}

fn default_test_paths() -> Vec<String> {
    vec![
        "tests/".to_string(),
        "benches/".to_string(),
        "examples/".to_string(),
    ]
}

/// Weight configuration for scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightsConfig {
    /// Weight for public functions
    #[serde(default = "default_public_function_weight")]
    pub public_function: usize,
    /// Weight for private functions
    #[serde(default = "default_private_function_weight")]
    pub private_function: usize,
    /// Weight for public structs
    #[serde(default = "default_public_struct_weight")]
    pub public_struct: usize,
    /// Weight for private structs
    #[serde(default = "default_private_struct_weight")]
    pub private_struct: usize,
    /// Weight for impl blocks
    #[serde(default = "default_impl_weight")]
    pub impl_block: usize,
    /// Weight for trait definitions
    #[serde(default = "default_trait_weight")]
    pub trait_definition: usize,
    /// Weight for const/static items
    #[serde(default = "default_const_weight")]
    pub const_static: usize,
}

impl Default for WeightsConfig {
    fn default() -> Self {
        Self {
            public_function: default_public_function_weight(),
            private_function: default_private_function_weight(),
            public_struct: default_public_struct_weight(),
            private_struct: default_private_struct_weight(),
            impl_block: default_impl_weight(),
            trait_definition: default_trait_weight(),
            const_static: default_const_weight(),
        }
    }
}

fn default_public_function_weight() -> usize {
    3
}

fn default_private_function_weight() -> usize {
    1
}

fn default_public_struct_weight() -> usize {
    3
}

fn default_private_struct_weight() -> usize {
    1
}

fn default_impl_weight() -> usize {
    2
}

fn default_trait_weight() -> usize {
    4
}

fn default_const_weight() -> usize {
    1
}

/// Per-type limit configuration
///
/// All fields are optional. When set, the analyzer will check that the number
/// of changed units of each type does not exceed the specified limit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerTypeLimits {
    /// Maximum number of functions
    pub functions: Option<usize>,
    /// Maximum number of structs
    pub structs: Option<usize>,
    /// Maximum number of enums
    pub enums: Option<usize>,
    /// Maximum number of traits
    pub traits: Option<usize>,
    /// Maximum number of impl blocks
    pub impl_blocks: Option<usize>,
    /// Maximum number of constants
    pub consts: Option<usize>,
    /// Maximum number of statics
    pub statics: Option<usize>,
    /// Maximum number of type aliases
    pub type_aliases: Option<usize>,
    /// Maximum number of macros
    pub macros: Option<usize>,
    /// Maximum number of modules
    pub modules: Option<usize>,
}

/// Limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    /// Maximum number of production units allowed
    #[serde(default = "default_max_prod_units")]
    pub max_prod_units: usize,
    /// Maximum weighted score allowed
    #[serde(default = "default_max_weighted_score")]
    pub max_weighted_score: usize,
    /// Maximum number of production lines added
    #[serde(default)]
    pub max_prod_lines: Option<usize>,
    /// Per-type limits for fine-grained control
    #[serde(default)]
    pub per_type: Option<PerTypeLimits>,
    /// Whether to fail when limits are exceeded
    #[serde(default = "default_fail_on_exceed")]
    pub fail_on_exceed: bool,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_prod_units: default_max_prod_units(),
            max_weighted_score: default_max_weighted_score(),
            max_prod_lines: None,
            per_type: None,
            fail_on_exceed: default_fail_on_exceed(),
        }
    }
}

fn default_max_prod_units() -> usize {
    30
}

fn default_max_weighted_score() -> usize {
    100
}

fn default_fail_on_exceed() -> bool {
    true
}

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// GitHub Actions output format
    #[default]
    Github,
    /// JSON output format
    Json,
    /// Human-readable output format
    Human,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output format to use
    #[serde(default)]
    pub format: OutputFormat,
    /// Whether to include detailed change information
    #[serde(default = "default_include_details")]
    pub include_details: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::default(),
            include_details: default_include_details(),
        }
    }
}

fn default_include_details() -> bool {
    true
}

/// Main configuration structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Classification settings
    #[serde(default)]
    pub classification: ClassificationConfig,
    /// Weight settings
    #[serde(default)]
    pub weights: WeightsConfig,
    /// Limit settings
    #[serde(default)]
    pub limits: LimitsConfig,
    /// Output settings
    #[serde(default)]
    pub output: OutputConfig,
}

impl Config {
    /// Loads configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// Loaded configuration or error
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
    /// use rust_diff_analyzer::Config;
    ///
    /// let config = Config::from_file(Path::new(".rust-diff-analyzer.toml"));
    /// ```
    pub fn from_file(path: &Path) -> Result<Self, AppError> {
        let content =
            fs::read_to_string(path).map_err(|e| AppError::from(FileReadError::new(path, e)))?;

        toml::from_str(&content).map_err(|e| AppError::from(ConfigError::new(path, e.to_string())))
    }

    /// Validates configuration values
    ///
    /// # Returns
    ///
    /// Ok if valid, error otherwise
    ///
    /// # Errors
    ///
    /// Returns error if any configuration value is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::Config;
    ///
    /// let config = Config::default();
    /// assert!(config.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), AppError> {
        if self.limits.max_prod_units == 0 {
            return Err(ConfigValidationError {
                field: "limits.max_prod_units".to_string(),
                message: "must be greater than 0".to_string(),
            }
            .into());
        }

        if self.limits.max_weighted_score == 0 {
            return Err(ConfigValidationError {
                field: "limits.max_weighted_score".to_string(),
                message: "must be greater than 0".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Returns set of test feature names
    ///
    /// # Returns
    ///
    /// HashSet of test feature names
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::Config;
    ///
    /// let config = Config::default();
    /// let features = config.test_features_set();
    /// assert!(features.contains("test-utils"));
    /// ```
    pub fn test_features_set(&self) -> HashSet<&str> {
        self.classification
            .test_features
            .iter()
            .map(|s| s.as_str())
            .collect()
    }

    /// Checks if a path should be ignored
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Returns
    ///
    /// `true` if path should be ignored
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use rust_diff_analyzer::Config;
    ///
    /// let config = Config::default();
    /// assert!(!config.should_ignore(Path::new("src/lib.rs")));
    /// ```
    pub fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.classification
            .ignore_paths
            .iter()
            .any(|p| path_str.contains(p))
    }

    /// Checks if a path is in a test directory
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Returns
    ///
    /// `true` if path is in a test directory
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use rust_diff_analyzer::Config;
    ///
    /// let config = Config::default();
    /// assert!(config.is_test_path(Path::new("tests/integration.rs")));
    /// assert!(!config.is_test_path(Path::new("src/lib.rs")));
    /// ```
    pub fn is_test_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.classification
            .test_paths
            .iter()
            .any(|p| path_str.contains(p))
    }

    /// Checks if path is a build script
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Returns
    ///
    /// `true` if path is build.rs
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use rust_diff_analyzer::Config;
    ///
    /// let config = Config::default();
    /// assert!(config.is_build_script(Path::new("build.rs")));
    /// assert!(!config.is_build_script(Path::new("src/lib.rs")));
    /// ```
    pub fn is_build_script(&self, path: &Path) -> bool {
        path.file_name().map(|n| n == "build.rs").unwrap_or(false)
    }
}

/// Builder for creating configurations programmatically
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Creates a new configuration builder
    ///
    /// # Returns
    ///
    /// A new ConfigBuilder with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let builder = ConfigBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the output format
    ///
    /// # Arguments
    ///
    /// * `format` - Output format to use
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::{ConfigBuilder, OutputFormat};
    ///
    /// let config = ConfigBuilder::new()
    ///     .output_format(OutputFormat::Json)
    ///     .build();
    /// ```
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.config.output.format = format;
        self
    }

    /// Sets the maximum production units limit
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of production units
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new().max_prod_units(50).build();
    /// ```
    pub fn max_prod_units(mut self, limit: usize) -> Self {
        self.config.limits.max_prod_units = limit;
        self
    }

    /// Sets the maximum weighted score limit
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum weighted score
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new().max_weighted_score(200).build();
    /// ```
    pub fn max_weighted_score(mut self, limit: usize) -> Self {
        self.config.limits.max_weighted_score = limit;
        self
    }

    /// Sets whether to fail on exceeded limits
    ///
    /// # Arguments
    ///
    /// * `fail` - Whether to fail on exceeded limits
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new().fail_on_exceed(false).build();
    /// ```
    pub fn fail_on_exceed(mut self, fail: bool) -> Self {
        self.config.limits.fail_on_exceed = fail;
        self
    }

    /// Sets the maximum production lines limit
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of production lines added
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new().max_prod_lines(200).build();
    /// ```
    pub fn max_prod_lines(mut self, limit: usize) -> Self {
        self.config.limits.max_prod_lines = Some(limit);
        self
    }

    /// Sets per-type limits
    ///
    /// # Arguments
    ///
    /// * `limits` - Per-type limit configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::{ConfigBuilder, PerTypeLimits};
    ///
    /// let per_type = PerTypeLimits {
    ///     functions: Some(5),
    ///     structs: Some(3),
    ///     ..Default::default()
    /// };
    /// let config = ConfigBuilder::new().per_type_limits(per_type).build();
    /// ```
    pub fn per_type_limits(mut self, limits: PerTypeLimits) -> Self {
        self.config.limits.per_type = Some(limits);
        self
    }

    /// Adds a test feature
    ///
    /// # Arguments
    ///
    /// * `feature` - Feature name to add
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new()
    ///     .add_test_feature("my-test-feature")
    ///     .build();
    /// ```
    pub fn add_test_feature(mut self, feature: &str) -> Self {
        self.config
            .classification
            .test_features
            .push(feature.to_string());
        self
    }

    /// Adds a path to ignore
    ///
    /// # Arguments
    ///
    /// * `path` - Path pattern to ignore
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new().add_ignore_path("fixtures/").build();
    /// ```
    pub fn add_ignore_path(mut self, path: &str) -> Self {
        self.config
            .classification
            .ignore_paths
            .push(path.to_string());
        self
    }

    /// Builds the configuration
    ///
    /// # Returns
    ///
    /// The built Config
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::new().build();
    /// ```
    pub fn build(self) -> Config {
        self.config
    }
}
