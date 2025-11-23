// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use masterror::{AppCode, AppErrorKind, Error};

/// Error for file read operations
#[derive(Debug, Error)]
#[error("failed to read file '{path}': {source}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal, message)]
pub struct FileReadError {
    pub path: String,
    pub source: std::io::Error,
}

impl FileReadError {
    /// Creates a new FileReadError from a path and source error
    pub fn new(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self {
            path: path.into().display().to_string(),
            source,
        }
    }
}

/// Error for Rust source code parsing
#[derive(Debug, Error)]
#[error("failed to parse '{path}': {message}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
pub struct ParseError {
    pub path: String,
    pub message: String,
}

impl ParseError {
    /// Creates a new ParseError from a path and message
    pub fn new(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            path: path.into().display().to_string(),
            message: message.into(),
        }
    }
}

/// Error for unified diff parsing
#[derive(Debug, Error)]
#[error("failed to parse diff: {message}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
pub struct DiffParseError {
    pub message: String,
}

/// Error for configuration file parsing
#[derive(Debug, Error)]
#[error("failed to parse config '{path}': {message}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
pub struct ConfigError {
    pub path: String,
    pub message: String,
}

impl ConfigError {
    /// Creates a new ConfigError from a path and message
    pub fn new(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            path: path.into().display().to_string(),
            message: message.into(),
        }
    }
}

/// Error for invalid configuration values
#[derive(Debug, Error)]
#[error("invalid config field '{field}': {message}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
pub struct ConfigValidationError {
    pub field: String,
    pub message: String,
}

/// Error for output formatting
#[derive(Debug, Error)]
#[error("output error for format '{format}': {message}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal, message)]
pub struct OutputError {
    pub format: String,
    pub message: String,
}

/// Error for analysis limit exceeded
#[derive(Debug, Error)]
#[error("limit exceeded for '{limit_type}': {actual} > {maximum}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
pub struct LimitExceededError {
    pub limit_type: String,
    pub actual: usize,
    pub maximum: usize,
}

/// Error for IO operations
#[derive(Debug, Error)]
#[error("io error: {0}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal, message)]
pub struct IoError(pub std::io::Error);
