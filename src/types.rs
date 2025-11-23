// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

pub mod change;
pub mod classification;
pub mod semantic_unit;

pub use change::{AnalysisResult, Change, Summary};
pub use classification::CodeType;
pub use semantic_unit::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};
