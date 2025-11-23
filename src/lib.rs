// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

pub mod analysis;
pub mod classifier;
pub mod config;
pub mod error;
pub mod git;
pub mod output;
pub mod types;

pub use config::Config;
pub use error::{
    ConfigError, ConfigValidationError, DiffParseError, FileReadError, IoError,
    LimitExceededError, OutputError, ParseError,
};
pub use masterror::AppError;
pub use types::{AnalysisResult, Change, CodeType, SemanticUnit, Summary};
