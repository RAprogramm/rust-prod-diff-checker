// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use proc_macro2::Span;
use syn::{
    Attribute, File, ImplItem, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMacro, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemType, TraitItem, Visibility as SynVisibility,
    spanned::Spanned, visit::Visit,
};

use crate::types::{LineSpan, SemanticUnit, SemanticUnitKind, Visibility};

/// Visitor for extracting semantic units from Rust AST
pub struct SemanticUnitVisitor {
    units: Vec<SemanticUnit>,
    in_test_module: bool,
    current_impl_name: Option<String>,
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
        attrs
            .iter()
            .filter_map(|attr| attr.path().get_ident().map(|ident| ident.to_string()))
            .collect()
    }

    fn has_test_attribute(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            let path = attr.path();
            if path.is_ident("test") || path.is_ident("bench") {
                return true;
            }
            if path.is_ident("cfg")
                && let Ok(meta) = attr.meta.require_list()
            {
                let tokens = meta.tokens.to_string();
                if tokens.contains("test") {
                    return true;
                }
            }
            false
        })
    }

    fn is_test_module(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            if attr.path().is_ident("cfg")
                && let Ok(meta) = attr.meta.require_list()
            {
                let tokens = meta.tokens.to_string();
                return tokens.contains("test");
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
        self.add_unit(
            SemanticUnitKind::Trait,
            node.ident.to_string(),
            self.convert_visibility(&node.vis),
            node.span(),
            &node.attrs,
        );
        syn::visit::visit_item_trait(self, node);
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
        match node {
            TraitItem::Fn(method) => {
                self.add_unit(
                    SemanticUnitKind::Function,
                    method.sig.ident.to_string(),
                    Visibility::Public,
                    method.span(),
                    &method.attrs,
                );
            }
            TraitItem::Const(c) => {
                self.add_unit(
                    SemanticUnitKind::Const,
                    c.ident.to_string(),
                    Visibility::Public,
                    c.span(),
                    &c.attrs,
                );
            }
            TraitItem::Type(t) => {
                self.add_unit(
                    SemanticUnitKind::TypeAlias,
                    t.ident.to_string(),
                    Visibility::Public,
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
}
