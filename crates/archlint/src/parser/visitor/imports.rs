use crate::parser::types::{ImportedSymbol, SymbolName};
use crate::parser::visitor::{interned, UnifiedVisitor};
use oxc_ast::ast::ImportDeclaration;
use oxc_span::GetSpan;

impl<'a> UnifiedVisitor {
    pub(crate) fn handle_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
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

    pub(crate) fn handle_reexport_specifiers(
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
}
