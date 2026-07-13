// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use proc_macro2::{Span, TokenStream, TokenTree};
use syn::{
    Attribute, File, ImplItem, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMacro, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemType, TraitItem, Visibility as SynVisibility,
    spanned::Spanned, visit::Visit,
};

use crate::types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};

/// Checks whether a `cfg` predicate token stream enables the item for test
/// builds.
///
/// Matches a bare `test` ident at any nesting level except inside `not(...)`,
/// so `#[cfg(test)]` and `#[cfg(any(test, unix))]` match while
/// `#[cfg(not(test))]` and `#[cfg(feature = "latest")]` do not.
fn cfg_predicate_enables_test(tokens: TokenStream, inside_not: bool) -> bool {
    let mut last_ident: Option<String> = None;

    for tree in tokens {
        match tree {
            TokenTree::Ident(ident) => {
                let name = ident.to_string();
                if name == "test" && !inside_not {
                    return true;
                }
                last_ident = Some(name);
            }
            TokenTree::Group(group) => {
                let inner_not = inside_not || last_ident.as_deref() == Some("not");
                if cfg_predicate_enables_test(group.stream(), inner_not) {
                    return true;
                }
                last_ident = None;
            }
            _ => {}
        }
    }

    false
}

/// Collects `feature = "name"` values from a `cfg` predicate token stream,
/// skipping features negated via `not(...)`.
fn collect_cfg_features(tokens: TokenStream, inside_not: bool, out: &mut Vec<String>) {
    let mut last_ident: Option<String> = None;
    let mut after_feature_eq = false;

    for tree in tokens {
        match tree {
            TokenTree::Ident(ident) => {
                last_ident = Some(ident.to_string());
                after_feature_eq = false;
            }
            TokenTree::Punct(punct) => {
                after_feature_eq =
                    punct.as_char() == '=' && last_ident.as_deref() == Some("feature");
            }
            TokenTree::Literal(literal) => {
                if after_feature_eq && !inside_not {
                    let raw = literal.to_string();
                    out.push(raw.trim_matches('"').to_string());
                }
                after_feature_eq = false;
                last_ident = None;
            }
            TokenTree::Group(group) => {
                let inner_not = inside_not || last_ident.as_deref() == Some("not");
                collect_cfg_features(group.stream(), inner_not, out);
                last_ident = None;
                after_feature_eq = false;
            }
        }
    }
}

/// Visitor for extracting semantic units from Rust AST
pub struct SemanticUnitVisitor {
    units: Vec<SemanticUnit>,
    in_test_module: bool,
    current_impl_name: Option<String>,
    current_trait_visibility: Option<Visibility>,
}

impl SemanticUnitVisitor {
    /// Creates a new semantic unit visitor
    ///
    /// # Returns
    ///
    /// A new SemanticUnitVisitor instance
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::analysis::ast_visitor::SemanticUnitVisitor;
    ///
    /// let visitor = SemanticUnitVisitor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
            in_test_module: false,
            current_impl_name: None,
            current_trait_visibility: None,
        }
    }

    /// Extracts semantic units from a parsed AST
    ///
    /// # Arguments
    ///
    /// * `file` - Parsed syn File
    ///
    /// # Returns
    ///
    /// Vector of extracted semantic units
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_diff_analyzer::analysis::ast_visitor::SemanticUnitVisitor;
    ///
    /// let code = "fn main() {}";
    /// let file = syn::parse_file(code).unwrap();
    /// let units = SemanticUnitVisitor::extract(&file);
    /// assert_eq!(units.len(), 1);
    /// ```
    pub fn extract(file: &File) -> Vec<SemanticUnit> {
        let mut visitor = Self::new();
        visitor.visit_file(file);
        visitor.units
    }

    fn span_to_line_span(&self, span: Span) -> LineSpan {
        let start = span.start();
        let end = span.end();
        LineSpan::new(start.line, end.line)
    }

    fn convert_visibility(&self, vis: &SynVisibility) -> Visibility {
        match vis {
            SynVisibility::Public(_) => Visibility::Public,
            SynVisibility::Restricted(r) => {
                if r.path.is_ident("crate") {
                    Visibility::Crate
                } else {
                    Visibility::Restricted
                }
            }
            SynVisibility::Inherited => Visibility::Private,
        }
    }

    fn extract_attributes(&self, attrs: &[Attribute]) -> Vec<String> {
        let mut attributes = Vec::new();

        for attr in attrs {
            let path = attr.path();
            let name = path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            if !name.is_empty() {
                attributes.push(name);
            }

            if path.is_ident("cfg")
                && let Ok(meta) = attr.meta.require_list()
            {
                let mut features = Vec::new();
                collect_cfg_features(meta.tokens.clone(), false, &mut features);
                for feature in features {
                    attributes.push(format!("cfg_feature:{}", feature));
                }
            }
        }

        attributes
    }

    fn has_test_attribute(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            let path = attr.path();
            let last_is_test = path
                .segments
                .last()
                .map(|s| s.ident == "test" || s.ident == "bench")
                .unwrap_or(false);
            if last_is_test && !path.is_ident("cfg") {
                return true;
            }
            if path.is_ident("cfg")
                && let Ok(meta) = attr.meta.require_list()
            {
                return cfg_predicate_enables_test(meta.tokens.clone(), false);
            }
            false
        })
    }

    fn is_test_module(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            if attr.path().is_ident("cfg")
                && let Ok(meta) = attr.meta.require_list()
            {
                return cfg_predicate_enables_test(meta.tokens.clone(), false);
            }
            false
        })
    }

    fn add_unit(
        &mut self,
        kind: SemanticUnitKind,
        name: String,
        visibility: Visibility,
        span: Span,
        attrs: &[Attribute],
    ) {
        let mut attributes = self.extract_attributes(attrs);

        if self.in_test_module && !attributes.contains(&"cfg_test".to_string()) {
            attributes.push("cfg_test".to_string());
        }

        if self.has_test_attribute(attrs) && !attributes.contains(&"test".to_string()) {
            attributes.push("test".to_string());
        }

        let unit = match &self.current_impl_name {
            Some(impl_name) => SemanticUnit::with_impl(
                kind,
                name,
                impl_name.clone(),
                visibility,
                self.span_to_line_span(span),
                attributes,
            ),
            None => SemanticUnit::new(
                kind,
                name,
                visibility,
                self.span_to_line_span(span),
                attributes,
            ),
        };
        self.units.push(unit);
    }
}

impl Default for SemanticUnitVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ast> Visit<'ast> for SemanticUnitVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.add_unit(
            SemanticUnitKind::Function,
            node.sig.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.add_unit(
            SemanticUnitKind::Struct,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        self.add_unit(
            SemanticUnitKind::Enum,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        let visibility = self.convert_visibility(&node.vis);
        self.add_unit(
            SemanticUnitKind::Trait,
            node.ident.to_string(),
            visibility.clone(),
            node.span(),
            &node.attrs,
        );

        let previous_visibility = self.current_trait_visibility.replace(visibility);
        syn::visit::visit_item_trait(self, node);
        self.current_trait_visibility = previous_visibility;
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let impl_name = if let Some((_, path, _)) = &node.trait_ {
            format!(
                "{} for {}",
                path.segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .unwrap_or_default(),
                type_to_string(&node.self_ty)
            )
        } else {
            type_to_string(&node.self_ty)
        };

        self.add_unit(
            SemanticUnitKind::Impl,
            impl_name.clone(),
            Visibility::Private,
            node.span(),
            &node.attrs,
        );

        let previous_impl_name = self.current_impl_name.take();
        self.current_impl_name = Some(impl_name);

        let was_in_test = self.in_test_module;
        if self.is_test_module(&node.attrs) {
            self.in_test_module = true;
        }

        for item in &node.items {
            match item {
                ImplItem::Fn(method) => {
                    self.add_unit(
                        SemanticUnitKind::Function,
                        method.sig.ident.to_string(),
                        self.convert_visibility(&method.vis),
                        method.span(),
                        &method.attrs,
                    );
                }
                ImplItem::Const(c) => {
                    self.add_unit(
                        SemanticUnitKind::Const,
                        c.ident.to_string(),
                        self.convert_visibility(&c.vis),
                        c.span(),
                        &c.attrs,
                    );
                }
                ImplItem::Type(t) => {
                    self.add_unit(
                        SemanticUnitKind::TypeAlias,
                        t.ident.to_string(),
                        self.convert_visibility(&t.vis),
                        t.span(),
                        &t.attrs,
                    );
                }
                _ => {}
            }
        }

        self.in_test_module = was_in_test;
        self.current_impl_name = previous_impl_name;
    }

    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        self.add_unit(
            SemanticUnitKind::Const,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
    }

    fn visit_item_static(&mut self, node: &'ast ItemStatic) {
        self.add_unit(
            SemanticUnitKind::Static,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        self.add_unit(
            SemanticUnitKind::TypeAlias,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
    }

    fn visit_item_macro(&mut self, node: &'ast ItemMacro) {
        if let Some(ident) = &node.ident {
            self.add_unit(
                SemanticUnitKind::Macro,
                ident.to_string(),
                Visibility::Private,
                node.span(),
                &node.attrs,
            );
        }
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        let is_test = self.is_test_module(&node.attrs) || node.ident == "tests";

        self.add_unit(
            SemanticUnitKind::Module,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );

        if let Some((_, items)) = &node.content {
            let was_in_test = self.in_test_module;
            self.in_test_module = is_test || was_in_test;

            for item in items {
                self.visit_item(item);
            }

            self.in_test_module = was_in_test;
        }
    }

    fn visit_trait_item(&mut self, node: &'ast TraitItem) {
        let visibility = self
            .current_trait_visibility
            .clone()
            .unwrap_or(Visibility::Public);

        match node {
            TraitItem::Fn(method) => {
                self.add_unit(
                    SemanticUnitKind::Function,
                    method.sig.ident.to_string(),
                    visibility,
                    method.span(),
                    &method.attrs,
                );
            }
            TraitItem::Const(c) => {
                self.add_unit(
                    SemanticUnitKind::Const,
                    c.ident.to_string(),
                    visibility,
                    c.span(),
                    &c.attrs,
                );
            }
            TraitItem::Type(t) => {
                self.add_unit(
                    SemanticUnitKind::TypeAlias,
                    t.ident.to_string(),
                    visibility,
                    t.span(),
                    &t.attrs,
                );
            }
            _ => {}
        }
        syn::visit::visit_trait_item(self, node);
    }
}

fn type_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        _ => "Unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function() {
        let code = "pub fn hello() {}";
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert_eq!(units.len(), 1);
        assert_eq!(units[0].name, "hello");
        assert!(matches!(units[0].kind, SemanticUnitKind::Function));
        assert!(matches!(units[0].visibility, Visibility::Public));
    }

    #[test]
    fn test_extract_struct() {
        let code = "struct Point { x: i32, y: i32 }";
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert_eq!(units.len(), 1);
        assert_eq!(units[0].name, "Point");
        assert!(matches!(units[0].kind, SemanticUnitKind::Struct));
    }

    #[test]
    fn test_extract_test_function() {
        let code = r#"
            #[test]
            fn test_something() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert_eq!(units.len(), 1);
        assert!(units[0].has_attribute("test"));
    }

    #[test]
    fn test_extract_impl_block() {
        let code = r#"
            struct Foo;
            impl Foo {
                pub fn new() -> Self { Foo }
            }
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert_eq!(units.len(), 3);
        assert!(
            units
                .iter()
                .any(|u| u.name == "Foo" && matches!(u.kind, SemanticUnitKind::Struct))
        );
        assert!(
            units
                .iter()
                .any(|u| u.name == "Foo" && matches!(u.kind, SemanticUnitKind::Impl))
        );
        assert!(
            units
                .iter()
                .any(|u| u.name == "new" && matches!(u.kind, SemanticUnitKind::Function))
        );
    }

    #[test]
    fn test_extract_test_module() {
        let code = r#"
            fn production() {}

            #[cfg(test)]
            mod tests {
                fn helper() {}

                #[test]
                fn test_it() {}
            }
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        let prod_fn = units
            .iter()
            .find(|u| u.name == "production")
            .expect("production not found");
        assert!(!prod_fn.has_attribute("cfg_test"));

        let helper_fn = units
            .iter()
            .find(|u| u.name == "helper")
            .expect("helper not found");
        assert!(helper_fn.has_attribute("cfg_test"));

        let test_fn = units
            .iter()
            .find(|u| u.name == "test_it")
            .expect("test_it not found");
        assert!(test_fn.has_attribute("test"));
        assert!(test_fn.has_attribute("cfg_test"));
    }

    #[test]
    fn test_cfg_not_test_is_production() {
        let code = r#"
            #[cfg(not(test))]
            pub fn prod_only() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert_eq!(units.len(), 1);
        assert!(!units[0].has_attribute("test"));
    }

    #[test]
    fn test_cfg_feature_latest_is_not_test() {
        let code = r#"
            #[cfg(feature = "latest")]
            pub fn latest_api() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert!(!units[0].has_attribute("test"));
        assert!(units[0].has_attribute("cfg_feature:latest"));
    }

    #[test]
    fn test_cfg_any_test_is_test() {
        let code = r#"
            #[cfg(any(test, feature = "slow"))]
            fn helper() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert!(units[0].has_attribute("test"));
    }

    #[test]
    fn test_multi_segment_test_attribute() {
        let code = r#"
            #[tokio::test]
            async fn async_test() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert!(units[0].has_attribute("test"));
        assert!(units[0].has_attribute("tokio::test"));
    }

    #[test]
    fn test_cfg_feature_marker_recorded() {
        let code = r#"
            #[cfg(feature = "mock")]
            pub fn mock_helper() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert!(units[0].has_attribute("cfg_feature:mock"));
    }

    #[test]
    fn test_cfg_not_feature_marker_skipped() {
        let code = r#"
            #[cfg(not(feature = "mock"))]
            pub fn real_impl() {}
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        assert!(!units[0].has_attribute("cfg_feature:mock"));
    }

    #[test]
    fn test_cfg_test_impl_marks_methods() {
        let code = r#"
            struct Foo;

            #[cfg(test)]
            impl Foo {
                fn helper() {}
            }
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        let helper = units
            .iter()
            .find(|u| u.name == "helper")
            .expect("helper not found");
        assert!(helper.has_attribute("cfg_test"));

        let foo = units
            .iter()
            .find(|u| matches!(u.kind, SemanticUnitKind::Struct))
            .expect("struct not found");
        assert!(!foo.has_attribute("cfg_test"));
    }

    #[test]
    fn test_private_trait_items_inherit_visibility() {
        let code = r#"
            trait Internal {
                fn hidden(&self);
            }

            pub trait External {
                fn visible(&self);
            }
        "#;
        let file = syn::parse_file(code).expect("parse failed");
        let units = SemanticUnitVisitor::extract(&file);

        let hidden = units
            .iter()
            .find(|u| u.name == "hidden")
            .expect("hidden not found");
        assert!(matches!(hidden.visibility, Visibility::Private));

        let visible = units
            .iter()
            .find(|u| u.name == "visible")
            .expect("visible not found");
        assert!(matches!(visible.visibility, Visibility::Public));
    }
}
