use std::fmt;
use std::io;
use std::path::PathBuf;

/// Application error types for rust-diff-analyzer
#[derive(Debug)]
pub enum AppError {
    /// Failed to read file from filesystem
    FileRead { path: PathBuf, source: io::Error },

    /// Failed to parse Rust source code
    ParseError { path: PathBuf, message: String },

    /// Failed to parse unified diff format
    DiffParseError { message: String },

    /// Failed to parse configuration file
    ConfigError { path: PathBuf, message: String },

    /// Invalid configuration value
    ConfigValidation { field: String, message: String },

    /// Output formatting error
    OutputError { format: String, message: String },

    /// Analysis limit exceeded
    LimitExceeded {
        limit_type: String,
        actual: usize,
        maximum: usize,
    },

    /// IO operation failed
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileRead { path, source } => {
                write!(f, "failed to read file '{}': {}", path.display(), source)
            }
            Self::ParseError { path, message } => {
                write!(f, "failed to parse '{}': {}", path.display(), message)
            }
            Self::DiffParseError { message } => {
                write!(f, "failed to parse diff: {}", message)
            }
            Self::ConfigError { path, message } => {
                write!(f, "failed to parse config '{}': {}", path.display(), message)
            }
            Self::ConfigValidation { field, message } => {
                write!(f, "invalid config field '{}': {}", field, message)
            }
            Self::OutputError { format, message } => {
                write!(f, "output error for format '{}': {}", format, message)
            }
            Self::LimitExceeded {
                limit_type,
                actual,
                maximum,
            } => {
                write!(
                    f,
                    "limit exceeded for '{}': {} > {}",
                    limit_type, actual, maximum
                )
            }
            Self::Io(source) => write!(f, "io error: {}", source),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FileRead { source, .. } => Some(source),
            Self::Io(source) => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
