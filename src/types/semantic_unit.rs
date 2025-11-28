// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Kind of semantic unit in Rust source code
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SemanticUnitKind {
    /// Function or method definition
    Function,
    /// Struct definition
    Struct,
    /// Enum definition
    Enum,
    /// Trait definition
    Trait,
    /// Impl block
    Impl,
    /// Constant definition
    Const,
    /// Static variable definition
    Static,
    /// Type alias
    TypeAlias,
    /// Macro definition
    Macro,
    /// Module definition
    Module,
}

impl SemanticUnitKind {
    /// Returns string representation of the unit kind
    ///
    /// # Returns
    ///
    /// A static string slice representing the kind
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::SemanticUnitKind;
    ///
    /// let kind = SemanticUnitKind::Function;
    /// assert_eq!(kind.as_str(), "function");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Impl => "impl",
            Self::Const => "const",
            Self::Static => "static",
            Self::TypeAlias => "type_alias",
            Self::Macro => "macro",
            Self::Module => "module",
        }
    }
}

/// Visibility level of a semantic unit
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Visibility {
    /// Public visibility (`pub`)
    Public,
    /// Crate-level visibility (`pub(crate)`)
    Crate,
    /// Restricted visibility (`pub(in path)`)
    Restricted,
    /// Private visibility (default)
    Private,
}

impl Visibility {
    /// Returns string representation of visibility
    ///
    /// # Returns
    ///
    /// A static string slice representing the visibility
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::Visibility;
    ///
    /// let vis = Visibility::Public;
    /// assert_eq!(vis.as_str(), "public");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Crate => "crate",
            Self::Restricted => "restricted",
            Self::Private => "private",
        }
    }

    /// Checks if this visibility is public
    ///
    /// # Returns
    ///
    /// `true` if visibility is Public
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::Visibility;
    ///
    /// assert!(Visibility::Public.is_public());
    /// assert!(!Visibility::Private.is_public());
    /// ```
    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public)
    }
}

/// Line span in source file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineSpan {
    /// Starting line (1-indexed)
    pub start: usize,
    /// Ending line (1-indexed, inclusive)
    pub end: usize,
}

impl LineSpan {
    /// Creates a new line span
    ///
    /// # Arguments
    ///
    /// * `start` - Starting line number (1-indexed)
    /// * `end` - Ending line number (1-indexed, inclusive)
    ///
    /// # Returns
    ///
    /// A new LineSpan instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::LineSpan;
    ///
    /// let span = LineSpan::new(10, 20);
    /// assert_eq!(span.start, 10);
    /// assert_eq!(span.end, 20);
    /// ```
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Checks if a line number is contained within this span
    ///
    /// # Arguments
    ///
    /// * `line` - Line number to check (1-indexed)
    ///
    /// # Returns
    ///
    /// `true` if line is within span bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::LineSpan;
    ///
    /// let span = LineSpan::new(10, 20);
    /// assert!(span.contains(15));
    /// assert!(!span.contains(5));
    /// assert!(!span.contains(25));
    /// ```
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }

    /// Returns the number of lines in this span
    ///
    /// # Returns
    ///
    /// Number of lines (inclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::LineSpan;
    ///
    /// let span = LineSpan::new(10, 20);
    /// assert_eq!(span.len(), 11);
    /// ```
    pub fn len(&self) -> usize {
        if self.end >= self.start {
            self.end - self.start + 1
        } else {
            0
        }
    }

    /// Checks if span is empty
    ///
    /// # Returns
    ///
    /// `true` if span has zero length
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::LineSpan;
    ///
    /// let span = LineSpan::new(10, 20);
    /// assert!(!span.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A semantic unit extracted from Rust source code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticUnit {
    /// Kind of semantic unit
    pub kind: SemanticUnitKind,
    /// Name of the unit (function name, struct name, etc.)
    pub name: String,
    /// Parent impl block name for methods (e.g., "Foo" or "Display for Foo")
    pub impl_name: Option<String>,
    /// Visibility level
    pub visibility: Visibility,
    /// Line span in source file
    pub span: LineSpan,
    /// Attributes on the unit (e.g., "test", "cfg(test)")
    pub attributes: Vec<String>,
}

impl SemanticUnit {
    /// Creates a new semantic unit
    ///
    /// # Arguments
    ///
    /// * `kind` - Kind of semantic unit
    /// * `name` - Name of the unit
    /// * `visibility` - Visibility level
    /// * `span` - Line span in source
    /// * `attributes` - List of attributes
    ///
    /// # Returns
    ///
    /// A new SemanticUnit instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};
    ///
    /// let unit = SemanticUnit::new(
    ///     SemanticUnitKind::Function,
    ///     "parse_token".to_string(),
    ///     Visibility::Public,
    ///     LineSpan::new(10, 30),
    ///     vec!["inline".to_string()],
    /// );
    /// assert_eq!(unit.name, "parse_token");
    /// ```
    pub fn new(
        kind: SemanticUnitKind,
        name: String,
        visibility: Visibility,
        span: LineSpan,
        attributes: Vec<String>,
    ) -> Self {
        Self {
            kind,
            name,
            impl_name: None,
            visibility,
            span,
            attributes,
        }
    }

    /// Creates a new semantic unit with impl context
    ///
    /// # Arguments
    ///
    /// * `kind` - Kind of semantic unit
    /// * `name` - Name of the unit
    /// * `impl_name` - Parent impl block name
    /// * `visibility` - Visibility level
    /// * `span` - Line span in source
    /// * `attributes` - List of attributes
    ///
    /// # Returns
    ///
    /// A new SemanticUnit instance with impl context
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};
    ///
    /// let unit = SemanticUnit::with_impl(
    ///     SemanticUnitKind::Function,
    ///     "new".to_string(),
    ///     "Parser".to_string(),
    ///     Visibility::Public,
    ///     LineSpan::new(10, 30),
    ///     vec![],
    /// );
    /// assert_eq!(unit.qualified_name(), "Parser::new");
    /// ```
    pub fn with_impl(
        kind: SemanticUnitKind,
        name: String,
        impl_name: String,
        visibility: Visibility,
        span: LineSpan,
        attributes: Vec<String>,
    ) -> Self {
        Self {
            kind,
            name,
            impl_name: Some(impl_name),
            visibility,
            span,
            attributes,
        }
    }

    /// Returns qualified name including impl context if present
    ///
    /// # Returns
    ///
    /// Qualified name (e.g., "Foo::method" or just "function")
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};
    ///
    /// let unit = SemanticUnit::with_impl(
    ///     SemanticUnitKind::Function,
    ///     "new".to_string(),
    ///     "Parser".to_string(),
    ///     Visibility::Public,
    ///     LineSpan::new(10, 30),
    ///     vec![],
    /// );
    /// assert_eq!(unit.qualified_name(), "Parser::new");
    ///
    /// let unit2 = SemanticUnit::new(
    ///     SemanticUnitKind::Function,
    ///     "main".to_string(),
    ///     Visibility::Private,
    ///     LineSpan::new(1, 5),
    ///     vec![],
    /// );
    /// assert_eq!(unit2.qualified_name(), "main");
    /// ```
    pub fn qualified_name(&self) -> String {
        match &self.impl_name {
            Some(impl_name) => format!("{}::{}", impl_name, self.name),
            None => self.name.clone(),
        }
    }

    /// Checks if unit has a specific attribute
    ///
    /// # Arguments
    ///
    /// * `attr` - Attribute name to check
    ///
    /// # Returns
    ///
    /// `true` if unit has the attribute
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};
    ///
    /// let unit = SemanticUnit::new(
    ///     SemanticUnitKind::Function,
    ///     "test_parse".to_string(),
    ///     Visibility::Private,
    ///     LineSpan::new(10, 30),
    ///     vec!["test".to_string()],
    /// );
    /// assert!(unit.has_attribute("test"));
    /// assert!(!unit.has_attribute("bench"));
    /// ```
    pub fn has_attribute(&self, attr: &str) -> bool {
        self.attributes.iter().any(|a| a == attr)
    }
}
