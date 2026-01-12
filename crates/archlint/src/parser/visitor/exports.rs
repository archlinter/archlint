use crate::parser::types::{ExportedSymbol, ImportedSymbol, SymbolKind, SymbolName, SymbolSet};
use crate::parser::visitor::{interned, UnifiedVisitor};
use oxc_ast::ast::{
    BindingPatternKind, Declaration, ExportAllDeclaration, ExportDefaultDeclaration,
    ExportNamedDeclaration, VariableDeclaration, VariableDeclarationKind,
};
use oxc_ast::visit::Visit;
use oxc_syntax::scope::ScopeFlags;

impl<'a> UnifiedVisitor {
    pub(crate) fn handle_export_named_declaration(&mut self, it: &ExportNamedDeclaration<'a>) {
        let source: Option<SymbolName> =
            it.source.as_ref().map(|s| Self::atom_to_compact(&s.value));

        if let Some(ref src) = source {
            self.handle_reexport_specifiers(it, src);
        }

        if let Some(decl) = &it.declaration {
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

        self.handle_export_specifiers(&it.specifiers, source);
    }

    pub(crate) fn handle_export_all_declaration(&mut self, it: &ExportAllDeclaration<'a>) {
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
            is_default: false,
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
            is_dynamic: false,
        });
        oxc_ast::visit::walk::walk_export_all_declaration(self, it);
    }

    pub(crate) fn handle_export_default_declaration(&mut self, it: &ExportDefaultDeclaration<'a>) {
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
            is_default: true,
        });
        self.has_runtime_code = true;
        oxc_ast::visit::walk::walk_export_default_declaration(self, it);
    }

    pub(crate) fn handle_variable_export(&mut self, d: &VariableDeclaration<'a>) {
        self.has_runtime_code = true;
        let is_mutable = d.kind != VariableDeclarationKind::Const;

        for decl in &d.declarations {
            if let BindingPatternKind::BindingIdentifier(id) = &decl.id.kind {
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
                    is_default: false,
                });

                let old_top = self.current_top_level_export.take();
                self.current_top_level_export = Some(export_idx);
                self.visit_variable_declarator(decl);
                self.current_top_level_export = old_top;
            }
        }
    }

    pub(crate) fn handle_function_export(&mut self, d: &oxc_ast::ast::Function<'a>) {
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
                is_default: false,
            });

            let old_top = self.current_top_level_export.take();
            self.current_top_level_export = Some(export_idx);
            self.visit_function(d, ScopeFlags::Function);
            self.current_top_level_export = old_top;
        }
    }

    pub(crate) fn handle_class_export(&mut self, d: &oxc_ast::ast::Class<'a>) {
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
                is_default: false,
            });

            let old_top = self.current_top_level_export.take();
            self.current_top_level_export = Some(export_idx);
            self.visit_class(d);
            self.current_top_level_export = old_top;
        }
    }

    pub(crate) fn handle_type_alias_export(
        &mut self,
        d: &oxc_ast::ast::TSTypeAliasDeclaration<'a>,
    ) {
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
            is_default: false,
        });
        self.visit_ts_type(&d.type_annotation);
    }

    pub(crate) fn handle_interface_export(&mut self, d: &oxc_ast::ast::TSInterfaceDeclaration<'a>) {
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
            is_default: false,
        });

        if let Some(extends) = &d.extends {
            for heritage in extends {
                self.visit_expression(&heritage.expression);
            }
        }
        self.visit_ts_interface_body(&d.body);
    }

    pub(crate) fn handle_enum_export(&mut self, d: &oxc_ast::ast::TSEnumDeclaration<'a>) {
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
            is_default: false,
        });
    }

    pub(crate) fn handle_export_specifiers(
        &mut self,
        specifiers: &oxc_allocator::Vec<'_, oxc_ast::ast::ExportSpecifier<'_>>,
        source: Option<SymbolName>,
    ) {
        for specifier in specifiers {
            let range = self.get_range(specifier.span);
            let name = Self::export_name_to_compact(specifier.exported.name());
            let is_default = name == *interned::DEFAULT;

            self.exports.push(ExportedSymbol {
                name,
                kind: SymbolKind::Unknown,
                is_reexport: source.is_some(),
                source: source.clone(),
                line: range.start_line,
                column: range.start_column,
                range,
                used_symbols: SymbolSet::default(),
                is_mutable: false,
                is_default,
            });
        }
    }
}
