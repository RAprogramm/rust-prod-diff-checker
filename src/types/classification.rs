use serde::{Deserialize, Serialize};

/// Classification of code type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodeType {
    /// Production code
    Production,
    /// Test code (marked with #[test] or in #[cfg(test)])
    Test,
    /// Test utility code (in test module but not a test)
    TestUtility,
    /// Benchmark code
    Benchmark,
    /// Example code
    Example,
    /// Build script code
    BuildScript,
}

impl CodeType {
    /// Returns string representation of code type
    ///
    /// # Returns
    ///
    /// A static string slice representing the code type
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::CodeType;
    ///
    /// assert_eq!(CodeType::Production.as_str(), "production");
    /// assert_eq!(CodeType::Test.as_str(), "test");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Production => "production",
            Self::Test => "test",
            Self::TestUtility => "test_utility",
            Self::Benchmark => "benchmark",
            Self::Example => "example",
            Self::BuildScript => "build_script",
        }
    }

    /// Checks if this is production code
    ///
    /// # Returns
    ///
    /// `true` if code type is Production
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::CodeType;
    ///
    /// assert!(CodeType::Production.is_production());
    /// assert!(!CodeType::Test.is_production());
    /// ```
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    /// Checks if this is any type of test code
    ///
    /// # Returns
    ///
    /// `true` if code type is Test, TestUtility, or Benchmark
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::CodeType;
    ///
    /// assert!(CodeType::Test.is_test_related());
    /// assert!(CodeType::TestUtility.is_test_related());
    /// assert!(CodeType::Benchmark.is_test_related());
    /// assert!(!CodeType::Production.is_test_related());
    /// ```
    pub fn is_test_related(&self) -> bool {
        matches!(self, Self::Test | Self::TestUtility | Self::Benchmark)
    }
}
