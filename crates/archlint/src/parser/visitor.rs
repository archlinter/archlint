use crate::detectors::CodeRange;
use crate::parser::complexity::{calculate_arrow_complexity, calculate_complexity};
use crate::parser::line_index::LineIndex;
use crate::parser::types::{
    ClassSymbol, ExportedSymbol, FunctionComplexity, ImportedSymbol, MethodAccessibility,
    MethodSymbol, ParserConfig, SymbolKind, SymbolName, SymbolSet,
};
use compact_str::CompactString;
use oxc_ast::ast::{Argument, Expression, Function, TSType};
use oxc_ast::visit::Visit;
use oxc_span::GetSpan;
use oxc_syntax::scope::ScopeFlags;
use smallvec::SmallVec;

/// Static interned strings for common symbols to avoid repeated allocations
mod interned {
    use compact_str::CompactString;
    use std::sync::LazyLock;

    pub static STAR: LazyLock<CompactString> = LazyLock::new(|| CompactString::const_new("*"));
    pub static DEFAULT: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new("default"));
    pub static ANONYMOUS: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new("<anonymous>"));
    pub static CONSTRUCTOR: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new("constructor"));
    pub static CONSTRUCTOR_SUFFIX: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new(".constructor"));
}

pub struct UnifiedVisitor {
    pub exports: Vec<ExportedSymbol>,
    pub imports: Vec<ImportedSymbol>,
    pub classes: Vec<ClassSymbol>,
    pub local_definitions: Vec<SymbolName>,
    pub local_usages: SymbolSet,
    pub has_runtime_code: bool,
    pub functions: Vec<FunctionComplexity>,
    pub config: ParserConfig,
    pub current_name_override: Option<SymbolName>,
    pub current_span_override: Option<oxc_span::Span>,
    pub current_class: Option<SymbolName>,

    pub temp_fields: SymbolSet,
    pub temp_methods: SmallVec<[MethodSymbol; 8]>,
    pub current_method: Option<MethodSymbol>,
    pub current_top_level_export: Option<usize>,
    pub env_vars: SymbolSet,

    /// Pre-computed line index for O(log n) line/column lookup
    line_index: LineIndex,
}

impl<'a> UnifiedVisitor {
    pub fn new(source_text: &str, config: ParserConfig) -> Self {
        // Pre-allocate based on file size heuristics
        let estimated_imports = (source_text.len() / 500).max(8);
        let estimated_exports = (source_text.len() / 1000).max(4);
        let estimated_functions = (source_text.len() / 200).max(8);

        Self {
            exports: Vec::with_capacity(estimated_exports),
            imports: Vec::with_capacity(estimated_imports),
            classes: Vec::new(),
            local_definitions: Vec::new(),
            local_usages: SymbolSet::default(),
            has_runtime_code: false,
            functions: Vec::with_capacity(estimated_functions),
            config,
            current_name_override: None,
            current_span_override: None,
            current_class: None,
            temp_fields: SymbolSet::default(),
            temp_methods: SmallVec::new(),
            current_method: None,
            current_top_level_export: None,
            env_vars: SymbolSet::default(),
            line_index: LineIndex::new(source_text),
        }
    }

    #[inline]
    fn get_range(&self, span: oxc_span::Span) -> CodeRange {
        let (start_line, start_column) = self.line_index.line_col(span.start as usize);
        let (end_line, end_column) = self.line_index.line_col(span.end as usize);
        CodeRange {
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    #[inline]
    fn get_line_number(&self, span: oxc_span::Span) -> usize {
        self.line_index.line(span.start as usize)
    }

    #[inline]
    pub fn line_count(&self) -> usize {
        self.line_index.line_count()
    }

    fn count_primitive_params(&self, params: &oxc_ast::ast::FormalParameters<'_>) -> usize {
        params
            .items
            .iter()
            .filter(|param| {
                param
                    .pattern
                    .type_annotation
                    .as_ref()
                    .map(|a| Self::is_primitive_type(&a.type_annotation))
                    .unwrap_or(true)
            })
            .count()
    }

    #[inline]
    fn is_primitive_type(ts_type: &TSType<'_>) -> bool {
        matches!(
            ts_type,
            TSType::TSStringKeyword(_)
                | TSType::TSNumberKeyword(_)
                | TSType::TSBooleanKeyword(_)
                | TSType::TSBigIntKeyword(_)
                | TSType::TSNullKeyword(_)
                | TSType::TSUndefinedKeyword(_)
                | TSType::TSSymbolKeyword(_)
                | TSType::TSAnyKeyword(_)
        )
    }

    #[inline]
    fn is_env_object(expr: &Expression<'_>) -> bool {
        if let Expression::StaticMemberExpression(s) = expr {
            if s.property.name == "env" {
                return match &s.object {
                    Expression::Identifier(ident) => ident.name == "process",
                    Expression::MetaProperty(mp) => {
                        mp.meta.name == "import" && mp.property.name == "meta"
                    }
                    _ => false,
                };
            }
        }
        false
    }

    /// Convert oxc Atom to CompactString efficiently
    #[inline]
    fn atom_to_compact(atom: &oxc_span::Atom) -> CompactString {
        CompactString::new(atom.as_str())
    }

    /// Convert oxc ModuleExportName to CompactString
    #[inline]
    fn export_name_to_compact(name: oxc_span::Atom) -> CompactString {
        CompactString::new(name.as_str())
    }

    fn ts_type_name_to_string(it: &oxc_ast::ast::TSTypeName<'_>) -> String {
        match it {
            oxc_ast::ast::TSTypeName::IdentifierReference(id) => id.name.to_string(),
            oxc_ast::ast::TSTypeName::QualifiedName(qn) => {
                format!(
                    "{}.{}",
                    Self::ts_type_name_to_string(&qn.left),
                    qn.right.name
                )
            }
        }
    }

    fn handle_reexport_specifiers(
        &mut self,
        it: &oxc_ast::ast::ExportNamedDeclaration<'_>,
        source: &SymbolName,
    ) {
        for specifier in &it.specifiers {
            let range = self.get_range(specifier.span);
            self.imports.push(ImportedSymbol {
                name: Self::export_name_to_compact(specifier.local.name()),
                alias: Some(Self::export_name_to_compact(specifier.exported.name())),
                source: source.clone(),
                line: range.start_line,
                column: range.start_column,
                range,
                is_type_only: it.export_kind.is_type(),
                is_reexport: true,
            });
        }

        if it.specifiers.is_empty() && it.declaration.is_none() {
            let range = self.get_range(it.span);
            self.imports.push(ImportedSymbol {
                name: interned::STAR.clone(),
                alias: None,
                source: source.clone(),
                line: range.start_line,
                column: range.start_column,
                range,
                is_type_only: it.export_kind.is_type(),
                is_reexport: true,
            });
        }
    }

    fn handle_variable_export(&mut self, d: &oxc_ast::ast::VariableDeclaration<'_>) {
        self.has_runtime_code = true;
        let is_mutable = d.kind != oxc_ast::ast::VariableDeclarationKind::Const;

        for decl in &d.declarations {
            if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(id) = &decl.id.kind {
                let range = self.get_range(decl.span);
                let export_idx = self.exports.len();
                self.exports.push(ExportedSymbol {
                    name: Self::atom_to_compact(&id.name),
                    kind: SymbolKind::Variable,
                    is_reexport: false,
                    source: None,
                    line: range.start_line,
                    column: range.start_column,
                    range,
                    used_symbols: SymbolSet::default(),
                    is_mutable,
                });

                let old_top = self.current_top_level_export.take();
                self.current_top_level_export = Some(export_idx);
                self.visit_variable_declarator(decl);
                self.current_top_level_export = old_top;
            }
        }
    }

    fn handle_function_export(&mut self, d: &oxc_ast::ast::Function<'_>) {
        self.has_runtime_code = true;

        if let Some(id) = &d.id {
            let range = self.get_range(d.span);
            let export_idx = self.exports.len();
            self.exports.push(ExportedSymbol {
                name: Self::atom_to_compact(&id.name),
                kind: SymbolKind::Function,
                is_reexport: false,
                source: None,
                line: range.start_line,
                column: range.start_column,
                range,
                used_symbols: SymbolSet::default(),
                is_mutable: false,
            });

            let old_top = self.current_top_level_export.take();
            self.current_top_level_export = Some(export_idx);
            self.visit_function(d, ScopeFlags::Function);
            self.current_top_level_export = old_top;
        }
    }

    fn handle_class_export(&mut self, d: &oxc_ast::ast::Class<'_>) {
        self.has_runtime_code = true;

        if let Some(id) = &d.id {
            let range = self.get_range(d.span);
            let export_idx = self.exports.len();
            self.exports.push(ExportedSymbol {
                name: Self::atom_to_compact(&id.name),
                kind: SymbolKind::Class,
                is_reexport: false,
                source: None,
                line: range.start_line,
                column: range.start_column,
                range,
                used_symbols: SymbolSet::default(),
                is_mutable: false,
            });

            let old_top = self.current_top_level_export.take();
            self.current_top_level_export = Some(export_idx);
            self.visit_class(d);
            self.current_top_level_export = old_top;
        }
    }

    fn handle_type_alias_export(&mut self, d: &oxc_ast::ast::TSTypeAliasDeclaration<'_>) {
        let range = self.get_range(d.span);
        self.exports.push(ExportedSymbol {
            name: Self::atom_to_compact(&d.id.name),
            kind: SymbolKind::Type,
            is_reexport: false,
            source: None,
            line: range.start_line,
            column: range.start_column,
            range,
            used_symbols: SymbolSet::default(),
            is_mutable: false,
        });
        self.visit_ts_type(&d.type_annotation);
    }

    fn handle_interface_export(&mut self, d: &oxc_ast::ast::TSInterfaceDeclaration<'_>) {
        let range = self.get_range(d.span);
        self.exports.push(ExportedSymbol {
            name: Self::atom_to_compact(&d.id.name),
            kind: SymbolKind::Interface,
            is_reexport: false,
            source: None,
            line: range.start_line,
            column: range.start_column,
            range,
            used_symbols: SymbolSet::default(),
            is_mutable: false,
        });

        if let Some(extends) = &d.extends {
            for heritage in extends {
                self.visit_expression(&heritage.expression);
            }
        }
        self.visit_ts_interface_body(&d.body);
    }

    fn handle_enum_export(&mut self, d: &oxc_ast::ast::TSEnumDeclaration<'_>) {
        self.has_runtime_code = true;
        let range = self.get_range(d.span);
        self.exports.push(ExportedSymbol {
            name: Self::atom_to_compact(&d.id.name),
            kind: SymbolKind::Enum,
            is_reexport: false,
            source: None,
            line: range.start_line,
            column: range.start_column,
            range,
            used_symbols: SymbolSet::default(),
            is_mutable: false,
        });
    }

    fn handle_export_specifiers(
        &mut self,
        specifiers: &oxc_allocator::Vec<'_, oxc_ast::ast::ExportSpecifier<'_>>,
        source: Option<SymbolName>,
        span: oxc_span::Span,
    ) {
        for specifier in specifiers {
            let range = self.get_range(span);
            self.exports.push(ExportedSymbol {
                name: Self::export_name_to_compact(specifier.exported.name()),
                kind: SymbolKind::Unknown,
                is_reexport: source.is_some(),
                source: source.clone(),
                line: range.start_line,
                column: range.start_column,
                range,
                used_symbols: SymbolSet::default(),
                is_mutable: false,
            });
        }
    }

    fn handle_static_member(&mut self, s: &oxc_ast::ast::StaticMemberExpression<'_>) {
        let name = Self::atom_to_compact(&s.property.name);
        self.local_usages.insert(name.clone());

        if self.config.collect_used_symbols {
            if let Expression::ThisExpression(_) = &s.object {
                if let Some(method) = &mut self.current_method {
                    method.used_fields.insert(name.clone());
                    method.used_methods.insert(name.clone());
                }
            }
            if let Some(idx) = self.current_top_level_export {
                self.exports[idx].used_symbols.insert(name.clone());
            }
        }

        if self.config.collect_env_vars && Self::is_env_object(&s.object) {
            self.env_vars.insert(name);
        }
    }

    fn handle_computed_member(&mut self, c: &oxc_ast::ast::ComputedMemberExpression<'_>) {
        if let Expression::StringLiteral(s) = &c.expression {
            let name = Self::atom_to_compact(&s.value);
            self.local_usages.insert(name.clone());

            if self.config.collect_env_vars && Self::is_env_object(&c.object) {
                self.env_vars.insert(name);
            }
        }
    }

    fn enter_class_scope(
        &mut self,
        it: &oxc_ast::ast::Class<'a>,
    ) -> (
        SymbolName,
        SymbolSet,
        SmallVec<[MethodSymbol; 8]>,
        Option<SymbolName>,
    ) {
        let class_name = it
            .id
            .as_ref()
            .map(|id| Self::atom_to_compact(&id.name))
            .unwrap_or_else(|| interned::ANONYMOUS.clone());

        let old_class = self.current_class.replace(class_name.clone());
        let old_fields = std::mem::take(&mut self.temp_fields);
        let old_methods = std::mem::take(&mut self.temp_methods);

        (class_name, old_fields, old_methods, old_class)
    }

    fn collect_class_fields(&mut self, it: &oxc_ast::ast::Class<'a>) {
        for item in &it.body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(p) = item {
                if let Some(name) = p.key.name() {
                    self.temp_fields.insert(CompactString::new(&name));
                }
            }
        }
    }

    fn expression_to_string(expr: &oxc_ast::ast::Expression<'_>) -> Option<String> {
        match expr {
            Expression::Identifier(id) => Some(id.name.to_string()),
            Expression::StaticMemberExpression(s) => {
                if let Some(obj) = Self::expression_to_string(&s.object) {
                    Some(format!("{}.{}", obj, s.property.name))
                } else {
                    Some(s.property.name.to_string())
                }
            }
            _ => None,
        }
    }

    fn finalize_class_symbol(&mut self, it: &oxc_ast::ast::Class<'a>, class_name: SymbolName) {
        let super_class = it
            .super_class
            .as_ref()
            .and_then(|expr| Self::expression_to_string(expr).map(CompactString::new));

        let mut implements = Vec::new();
        if let Some(impls) = &it.implements {
            for imp in impls {
                let name = Self::ts_type_name_to_string(&imp.expression);
                implements.push(CompactString::new(name));
            }
        }

        let class_symbol = ClassSymbol {
            name: class_name,
            super_class,
            implements,
            fields: self.temp_fields.iter().cloned().collect(),
            methods: self.temp_methods.clone(),
            is_abstract: it.r#abstract,
        };
        self.classes.push(class_symbol);
    }

    fn exit_class_scope(
        &mut self,
        old_fields: SymbolSet,
        old_methods: SmallVec<[MethodSymbol; 8]>,
        old_class: Option<SymbolName>,
    ) {
        self.temp_fields = old_fields;
        self.temp_methods = old_methods;
        self.current_class = old_class;
    }
}

impl<'a> Visit<'a> for UnifiedVisitor {
    fn visit_import_declaration(&mut self, it: &oxc_ast::ast::ImportDeclaration<'a>) {
        let source: SymbolName = Self::atom_to_compact(&it.source.value);
        let is_type_only_decl = it.import_kind.is_type();

        if let Some(specifiers) = &it.specifiers {
            if specifiers.is_empty() {
                let range = self.get_range(it.span);
                self.imports.push(ImportedSymbol {
                    name: interned::STAR.clone(),
                    alias: None,
                    source,
                    line: range.start_line,
                    column: range.start_column,
                    range,
                    is_type_only: is_type_only_decl,
                    is_reexport: false,
                });
            } else {
                for specifier in specifiers {
                    let range = self.get_range(specifier.span());
                    match specifier {
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(s) => {
                            self.imports.push(ImportedSymbol {
                                name: Self::export_name_to_compact(s.imported.name()),
                                alias: Some(Self::atom_to_compact(&s.local.name)),
                                source: source.clone(),
                                line: range.start_line,
                                column: range.start_column,
                                range,
                                is_type_only: is_type_only_decl || s.import_kind.is_type(),
                                is_reexport: false,
                            });
                        }
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                            self.imports.push(ImportedSymbol {
                                name: interned::DEFAULT.clone(),
                                alias: Some(Self::atom_to_compact(&s.local.name)),
                                source: source.clone(),
                                line: range.start_line,
                                column: range.start_column,
                                range,
                                is_type_only: is_type_only_decl,
                                is_reexport: false,
                            });
                        }
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                            self.imports.push(ImportedSymbol {
                                name: interned::STAR.clone(),
                                alias: Some(Self::atom_to_compact(&s.local.name)),
                                source: source.clone(),
                                line: range.start_line,
                                column: range.start_column,
                                range,
                                is_type_only: is_type_only_decl,
                                is_reexport: false,
                            });
                        }
                    }
                }
            }
        } else {
            let range = self.get_range(it.span);
            self.imports.push(ImportedSymbol {
                name: interned::STAR.clone(),
                alias: None,
                source,
                line: range.start_line,
                column: range.start_column,
                range,
                is_type_only: is_type_only_decl,
                is_reexport: false,
            });
        }
        oxc_ast::visit::walk::walk_import_declaration(self, it);
    }

    fn visit_export_named_declaration(&mut self, it: &oxc_ast::ast::ExportNamedDeclaration<'a>) {
        let source: Option<SymbolName> =
            it.source.as_ref().map(|s| Self::atom_to_compact(&s.value));

        if let Some(ref src) = source {
            self.handle_reexport_specifiers(it, src);
        }

        if let Some(decl) = &it.declaration {
            use oxc_ast::ast::Declaration;
            match decl {
                Declaration::VariableDeclaration(d) => self.handle_variable_export(d),
                Declaration::FunctionDeclaration(d) => self.handle_function_export(d),
                Declaration::ClassDeclaration(d) => self.handle_class_export(d),
                Declaration::TSTypeAliasDeclaration(d) => self.handle_type_alias_export(d),
                Declaration::TSInterfaceDeclaration(d) => self.handle_interface_export(d),
                Declaration::TSEnumDeclaration(d) => self.handle_enum_export(d),
                _ => {}
            }
        }

        self.handle_export_specifiers(&it.specifiers, source, it.span);
    }

    fn visit_export_all_declaration(&mut self, it: &oxc_ast::ast::ExportAllDeclaration<'a>) {
        let source = Self::atom_to_compact(&it.source.value);
        let range = self.get_range(it.span);
        let name = it
            .exported
            .as_ref()
            .map(|id| Self::export_name_to_compact(id.name()))
            .unwrap_or_else(|| interned::STAR.clone());

        self.exports.push(ExportedSymbol {
            name: name.clone(),
            kind: SymbolKind::Unknown,
            is_reexport: true,
            source: Some(source.clone()),
            line: range.start_line,
            column: range.start_column,
            range,
            used_symbols: SymbolSet::default(),
            is_mutable: false,
        });
        self.imports.push(ImportedSymbol {
            name,
            alias: None,
            source,
            line: range.start_line,
            column: range.start_column,
            range,
            is_type_only: it.export_kind.is_type(),
            is_reexport: true,
        });
        oxc_ast::visit::walk::walk_export_all_declaration(self, it);
    }

    fn visit_export_default_declaration(
        &mut self,
        it: &oxc_ast::ast::ExportDefaultDeclaration<'a>,
    ) {
        let range = self.get_range(it.span);
        self.exports.push(ExportedSymbol {
            name: interned::DEFAULT.clone(),
            kind: SymbolKind::Unknown,
            is_reexport: false,
            source: None,
            line: range.start_line,
            column: range.start_column,
            range,
            used_symbols: SymbolSet::default(),
            is_mutable: false,
        });
        self.has_runtime_code = true;
        oxc_ast::visit::walk::walk_export_default_declaration(self, it);
    }

    fn visit_identifier_reference(&mut self, it: &oxc_ast::ast::IdentifierReference<'a>) {
        let name = Self::atom_to_compact(&it.name);
        self.local_usages.insert(name.clone());
        if self.config.collect_used_symbols {
            if let Some(method) = &mut self.current_method {
                method.used_fields.insert(name.clone());
                method.used_methods.insert(name.clone());
            }
            if let Some(idx) = self.current_top_level_export {
                self.exports[idx].used_symbols.insert(name);
            }
        }
    }

    fn visit_jsx_identifier(&mut self, it: &oxc_ast::ast::JSXIdentifier<'a>) {
        self.local_usages.insert(Self::atom_to_compact(&it.name));
    }

    fn visit_member_expression(&mut self, it: &oxc_ast::ast::MemberExpression<'a>) {
        match it {
            oxc_ast::ast::MemberExpression::StaticMemberExpression(s) => {
                self.handle_static_member(s)
            }
            oxc_ast::ast::MemberExpression::ComputedMemberExpression(c) => {
                self.handle_computed_member(c)
            }
            _ => {}
        }
        oxc_ast::visit::walk::walk_member_expression(self, it);
    }

    fn visit_ts_type_name(&mut self, it: &oxc_ast::ast::TSTypeName<'a>) {
        match it {
            oxc_ast::ast::TSTypeName::IdentifierReference(ident) => {
                self.local_usages.insert(Self::atom_to_compact(&ident.name));
            }
            oxc_ast::ast::TSTypeName::QualifiedName(qn) => {
                self.visit_ts_type_name(&qn.left);
                self.local_usages
                    .insert(Self::atom_to_compact(&qn.right.name));
            }
        }
        oxc_ast::visit::walk::walk_ts_type_name(self, it);
    }

    fn visit_ts_type_reference(&mut self, it: &oxc_ast::ast::TSTypeReference<'a>) {
        self.visit_ts_type_name(&it.type_name);
        oxc_ast::visit::walk::walk_ts_type_reference(self, it);
    }

    fn visit_ts_type_alias_declaration(&mut self, it: &oxc_ast::ast::TSTypeAliasDeclaration<'a>) {
        self.visit_ts_type(&it.type_annotation);
        oxc_ast::visit::walk::walk_ts_type_alias_declaration(self, it);
    }

    fn visit_ts_interface_declaration(&mut self, it: &oxc_ast::ast::TSInterfaceDeclaration<'a>) {
        if let Some(extends) = &it.extends {
            for heritage in extends {
                self.visit_expression(&heritage.expression);
            }
        }
        self.visit_ts_interface_body(&it.body);
        oxc_ast::visit::walk::walk_ts_interface_declaration(self, it);
    }

    fn visit_expression(&mut self, expr: &Expression<'a>) {
        match expr {
            Expression::ImportExpression(import) => {
                if let Expression::StringLiteral(source) = &import.source {
                    let range = self.get_range(import.span);
                    self.imports.push(ImportedSymbol {
                        name: interned::STAR.clone(),
                        alias: None,
                        source: Self::atom_to_compact(&source.value),
                        line: range.start_line,
                        column: range.start_column,
                        range,
                        is_type_only: false,
                        is_reexport: false,
                    });
                }
            }
            Expression::CallExpression(call) => {
                self.has_runtime_code = true;
                if let Expression::Identifier(ident) = &call.callee {
                    if ident.name == "require" && call.arguments.len() == 1 {
                        if let Argument::StringLiteral(source) = &call.arguments[0] {
                            let range = self.get_range(call.span);
                            self.imports.push(ImportedSymbol {
                                name: interned::STAR.clone(),
                                alias: None,
                                source: Self::atom_to_compact(&source.value),
                                line: range.start_line,
                                column: range.start_column,
                                range,
                                is_type_only: false,
                                is_reexport: false,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        oxc_ast::visit::walk::walk_expression(self, expr);
    }

    fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
        let (class_name, old_fields, old_methods, old_class) = self.enter_class_scope(it);

        if self.config.collect_classes {
            self.collect_class_fields(it);
        }

        oxc_ast::visit::walk::walk_class(self, it);

        if self.config.collect_classes {
            self.finalize_class_symbol(it, class_name);
        }

        self.exit_class_scope(old_fields, old_methods, old_class);
    }

    fn visit_variable_declarator(&mut self, it: &oxc_ast::ast::VariableDeclarator<'a>) {
        if let oxc_ast::ast::BindingPatternKind::BindingIdentifier(ref id) = it.id.kind {
            if let Some(ref init) = it.init {
                if matches!(
                    init,
                    Expression::ArrowFunctionExpression(_)
                        | Expression::FunctionExpression(_)
                        | Expression::ClassExpression(_)
                ) {
                    self.current_name_override = Some(Self::atom_to_compact(&id.name));
                    self.current_span_override = Some(id.span);
                }
            }
        }
        oxc_ast::visit::walk::walk_variable_declarator(self, it);
        self.current_name_override = None;
        self.current_span_override = None;
    }

    fn visit_method_definition(&mut self, it: &oxc_ast::ast::MethodDefinition<'a>) {
        let name = it
            .key
            .name()
            .map(|c| CompactString::new(&c))
            .unwrap_or_else(|| interned::ANONYMOUS.clone());

        let old_method = self.current_method.take();
        if self.config.collect_classes {
            let span = it.key.span();
            let (line_num, col_num) = self.line_index.line_col(span.start as usize);
            let range = self.get_range(span);
            let has_decorators = !it.decorators.is_empty();
            let is_accessor = matches!(
                it.kind,
                oxc_ast::ast::MethodDefinitionKind::Get | oxc_ast::ast::MethodDefinitionKind::Set
            );
            let accessibility = it.accessibility.map(|a| match a {
                oxc_ast::ast::TSAccessibility::Public => MethodAccessibility::Public,
                oxc_ast::ast::TSAccessibility::Protected => MethodAccessibility::Protected,
                oxc_ast::ast::TSAccessibility::Private => MethodAccessibility::Private,
            });
            self.current_method = Some(MethodSymbol::new(
                name.clone(),
                line_num,
                col_num,
                range,
                has_decorators,
                is_accessor,
                accessibility,
                it.r#type.is_abstract(),
            ));
        }

        self.current_name_override = Some(name);
        self.current_span_override = Some(it.key.span());
        oxc_ast::visit::walk::walk_method_definition(self, it);
        self.current_name_override = None;
        self.current_span_override = None;

        if self.config.collect_classes {
            if let Some(mut method) = self.current_method.take() {
                method.used_fields.retain(|f| self.temp_fields.contains(f));
                self.temp_methods.push(method);
            }
        }

        self.current_method = old_method;
    }

    fn visit_function(&mut self, it: &Function<'a>, flags: ScopeFlags) {
        let mut name = it
            .id
            .as_ref()
            .map(|id| Self::atom_to_compact(&id.name))
            .or_else(|| self.current_name_override.take())
            .unwrap_or_else(|| interned::ANONYMOUS.clone());

        if let Some(class_name) = &self.current_class {
            name = CompactString::new(format!("{}.{}", class_name, name));
        }

        let (complexity, max_depth) = if self.config.collect_complexity {
            calculate_complexity(it)
        } else {
            (0, 0)
        };

        let span = it
            .id
            .as_ref()
            .map(|id| id.span)
            .or(self.current_span_override.take())
            .unwrap_or(it.span);
        let line = self.get_line_number(span);
        let range = self.get_range(span);

        let param_count = it.params.items.len();
        let primitive_params = if self.config.collect_primitive_params {
            self.count_primitive_params(&it.params)
        } else {
            0
        };
        let is_constructor =
            name == *interned::CONSTRUCTOR || name.ends_with(interned::CONSTRUCTOR_SUFFIX.as_str());

        self.functions.push(FunctionComplexity {
            name,
            line,
            range,
            complexity,
            max_depth,
            param_count,
            primitive_params,
            is_constructor,
        });

        oxc_ast::visit::walk::walk_function(self, it, flags);
    }

    fn visit_arrow_function_expression(&mut self, it: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        let mut name = self
            .current_name_override
            .take()
            .unwrap_or_else(|| interned::ANONYMOUS.clone());

        if let Some(class_name) = &self.current_class {
            name = CompactString::new(format!("{}.{}", class_name, name));
        }

        let (complexity, max_depth) = if self.config.collect_complexity {
            calculate_arrow_complexity(it)
        } else {
            (0, 0)
        };

        let span = self.current_span_override.take().unwrap_or(it.span);
        let line = self.get_line_number(span);
        let range = self.get_range(span);

        let param_count = it.params.items.len();
        let primitive_params = if self.config.collect_primitive_params {
            self.count_primitive_params(&it.params)
        } else {
            0
        };
        let is_constructor =
            name == *interned::CONSTRUCTOR || name.ends_with(interned::CONSTRUCTOR_SUFFIX.as_str());

        self.functions.push(FunctionComplexity {
            name,
            line,
            range,
            complexity,
            max_depth,
            param_count,
            primitive_params,
            is_constructor,
        });

        oxc_ast::visit::walk::walk_arrow_function_expression(self, it);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::types::ParserConfig;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    fn parse_code(code: &str) -> UnifiedVisitor {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_typescript(true);
        let ret = Parser::new(&allocator, code, source_type).parse();
        let mut visitor = UnifiedVisitor::new(code, ParserConfig::all());
        visitor.visit_program(&ret.program);
        visitor
    }

    #[test]
    fn test_is_primitive_type() {
        let code = "function test(a: string, b: number, c: boolean, d: bigint, e: any, f: undefined, g: symbol) {}";
        let visitor = parse_code(code);
        assert_eq!(visitor.functions[0].primitive_params, 7);
    }

    #[test]
    fn test_is_env_object() {
        let visitor = parse_code("process.env.DB_URL; import.meta.env.API_KEY;");
        assert!(visitor.env_vars.contains("DB_URL"));
        assert!(visitor.env_vars.contains("API_KEY"));
    }

    #[test]
    fn test_count_primitive_params() {
        let visitor =
            parse_code("function test(a: string, b: number, c: any, d: { x: number }) {}");
        assert_eq!(visitor.functions.len(), 1);
        assert_eq!(visitor.functions[0].primitive_params, 3);
    }

    #[test]
    fn test_empty_import_specifiers() {
        let visitor = parse_code("import './side-effect';");
        assert_eq!(visitor.imports.len(), 1);
        assert_eq!(visitor.imports[0].name, "*");
        assert_eq!(visitor.imports[0].source, "./side-effect");
    }

    #[test]
    fn test_reexport_star() {
        let visitor = parse_code("export * from './foo';");
        assert_eq!(visitor.imports.len(), 1);
        assert!(visitor.imports[0].is_reexport);
        assert_eq!(visitor.imports[0].name, "*");
    }

    #[test]
    fn test_export_variable_mutable() {
        let visitor = parse_code("export const a = 1; export let b = 2;");
        let a = visitor.exports.iter().find(|e| e.name == "a").unwrap();
        let b = visitor.exports.iter().find(|e| e.name == "b").unwrap();
        assert!(!a.is_mutable);
        assert!(b.is_mutable);
    }

    #[test]
    fn test_class_with_constructor() {
        let visitor =
            parse_code("class A { constructor(private x: number) {} method() { this.x; } }");
        assert_eq!(visitor.classes.len(), 1);
        assert_eq!(visitor.classes[0].methods.len(), 2);
        assert!(visitor.classes[0]
            .methods
            .iter()
            .any(|m| m.name == "constructor"));
    }

    #[test]
    fn test_interface_extends() {
        let visitor = parse_code("interface A {} interface B extends A {}");
        assert!(visitor.local_usages.contains("A"));
    }

    #[test]
    fn test_type_alias_union() {
        let visitor = parse_code("type T = string | number | MyType;");
        assert!(visitor.local_usages.contains("MyType"));
    }
}
