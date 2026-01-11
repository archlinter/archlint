use crate::parser::types::ImportedSymbol;
use crate::parser::visitor::{interned, UnifiedVisitor};
use oxc_ast::ast::{Argument, Expression};

impl<'a> UnifiedVisitor {
    pub(crate) fn handle_identifier_reference(
        &mut self,
        it: &oxc_ast::ast::IdentifierReference<'a>,
    ) {
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

    pub(crate) fn handle_jsx_identifier(&mut self, it: &oxc_ast::ast::JSXIdentifier<'a>) {
        self.local_usages.insert(Self::atom_to_compact(&it.name));
    }

    pub(crate) fn handle_member_expression(&mut self, it: &oxc_ast::ast::MemberExpression<'a>) {
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

    pub(crate) fn handle_ts_type_name(&mut self, it: &oxc_ast::ast::TSTypeName<'a>) {
        if let oxc_ast::ast::TSTypeName::QualifiedName(qn) = it {
            self.local_usages
                .insert(Self::atom_to_compact(&qn.right.name));
        }
        oxc_ast::visit::walk::walk_ts_type_name(self, it);
    }

    pub(crate) fn handle_ts_type_reference(&mut self, it: &oxc_ast::ast::TSTypeReference<'a>) {
        oxc_ast::visit::walk::walk_ts_type_reference(self, it);
    }

    pub(crate) fn handle_ts_type_alias_declaration(
        &mut self,
        it: &oxc_ast::ast::TSTypeAliasDeclaration<'a>,
    ) {
        oxc_ast::visit::walk::walk_ts_type_alias_declaration(self, it);
    }

    pub(crate) fn handle_ts_interface_declaration(
        &mut self,
        it: &oxc_ast::ast::TSInterfaceDeclaration<'a>,
    ) {
        oxc_ast::visit::walk::walk_ts_interface_declaration(self, it);
    }

    pub(crate) fn handle_expression(&mut self, expr: &Expression<'a>) {
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

    pub(crate) fn handle_static_member(&mut self, s: &oxc_ast::ast::StaticMemberExpression<'_>) {
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

    pub(crate) fn handle_computed_member(
        &mut self,
        c: &oxc_ast::ast::ComputedMemberExpression<'_>,
    ) {
        if let Expression::StringLiteral(s) = &c.expression {
            let name = Self::atom_to_compact(&s.value);
            self.local_usages.insert(name.clone());

            if self.config.collect_env_vars && Self::is_env_object(&c.object) {
                self.env_vars.insert(name);
            }
        }
    }

    #[inline]
    pub(crate) fn is_env_object(expr: &Expression<'_>) -> bool {
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
}
