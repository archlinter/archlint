use crate::parser::complexity::{calculate_arrow_complexity, calculate_complexity};
use crate::parser::types::{
    ClassSymbol, FunctionComplexity, MethodAccessibility, MethodSymbol, SymbolName, SymbolSet,
};
use crate::parser::visitor::{interned, UnifiedVisitor};
use compact_str::CompactString;
use oxc_ast::ast::{Class, ClassElement, Expression, Function, MethodDefinitionKind, TSType};
use oxc_span::GetSpan;
use oxc_syntax::scope::ScopeFlags;
use smallvec::SmallVec;

impl<'a> UnifiedVisitor {
    pub(crate) fn handle_class(&mut self, it: &Class<'a>) {
        let (class_name, old_fields, old_methods, old_class) = self.enter_class_scope(it);

        if self.config.collect_classes {
            self.collect_class_fields(it);
        }

        oxc_ast_visit::walk::walk_class(self, it);

        if self.config.collect_classes {
            self.finalize_class_symbol(it, class_name);
        }

        self.exit_class_scope(old_fields, old_methods, old_class);
    }

    pub(crate) fn handle_variable_declarator(&mut self, it: &oxc_ast::ast::VariableDeclarator<'a>) {
        if let oxc_ast::ast::BindingPattern::BindingIdentifier(ref id) = &it.id {
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
        oxc_ast_visit::walk::walk_variable_declarator(self, it);
        self.current_name_override = None;
        self.current_span_override = None;
    }

    pub(crate) fn handle_method_definition(&mut self, it: &oxc_ast::ast::MethodDefinition<'a>) {
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
                MethodDefinitionKind::Get | MethodDefinitionKind::Set
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
        oxc_ast_visit::walk::walk_method_definition(self, it);
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

    pub(crate) fn handle_function(&mut self, it: &Function<'a>, flags: ScopeFlags) {
        let name = self.get_scoped_name(it.id.as_ref().map(|id| Self::atom_to_compact(&id.name)));

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

        self.collect_function_metrics(name, span, complexity, max_depth, &it.params);

        oxc_ast_visit::walk::walk_function(self, it, flags);
    }

    pub(crate) fn handle_arrow_function_expression(
        &mut self,
        it: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        let name = self.get_scoped_name(None);

        let (complexity, max_depth) = if self.config.collect_complexity {
            calculate_arrow_complexity(it)
        } else {
            (0, 0)
        };

        let span = self.current_span_override.take().unwrap_or(it.span);

        self.collect_function_metrics(name, span, complexity, max_depth, &it.params);

        oxc_ast_visit::walk::walk_arrow_function_expression(self, it);
    }

    fn get_scoped_name(&mut self, base_name: Option<CompactString>) -> CompactString {
        let mut name = base_name
            .or_else(|| self.current_name_override.take())
            .unwrap_or_else(|| interned::ANONYMOUS.clone());

        if let Some(class_name) = &self.current_class {
            name = CompactString::new(format!("{}.{}", class_name, name));
        }
        name
    }

    fn collect_function_metrics(
        &mut self,
        name: CompactString,
        span: oxc_span::Span,
        complexity: usize,
        max_depth: usize,
        params: &oxc_ast::ast::FormalParameters<'a>,
    ) {
        let line = self.get_line_number(span);
        let range = self.get_range(span);

        let param_count = params.items.len();
        let primitive_params = if self.config.collect_primitive_params {
            self.count_primitive_params(params)
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
    }

    fn count_primitive_params(&self, params: &oxc_ast::ast::FormalParameters<'_>) -> usize {
        params
            .items
            .iter()
            .filter(|param| {
                param
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

    fn enter_class_scope(
        &mut self,
        it: &Class<'a>,
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

    fn collect_class_fields(&mut self, it: &Class<'a>) {
        for item in &it.body.body {
            match item {
                ClassElement::PropertyDefinition(p) => {
                    if let Some(name) = p.key.name() {
                        self.temp_fields.insert(CompactString::new(&name));
                    }
                }
                ClassElement::MethodDefinition(m)
                    if m.kind == MethodDefinitionKind::Constructor =>
                {
                    for param in &m.value.params.items {
                        // In TypeScript, only parameters with access modifiers (public, private, protected)
                        // or readonly become class fields (parameter properties).
                        // However, for NestJS/Dependency Injection patterns, constructor parameters
                        // are typically fields even without explicit modifiers (decorators handle this).
                        // We collect all constructor parameters as fields to support both cases.
                        if let oxc_ast::ast::BindingPattern::BindingIdentifier(id) = &param.pattern
                        {
                            self.temp_fields.insert(Self::atom_to_compact(&id.name));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn expression_to_string(expr: &Expression<'_>) -> Option<String> {
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

    fn finalize_class_symbol(&mut self, it: &Class<'a>, class_name: SymbolName) {
        let super_class = it
            .super_class
            .as_ref()
            .and_then(|expr| Self::expression_to_string(expr).map(CompactString::new));

        let mut implements = Vec::new();
        for imp in &it.implements {
            let name = Self::ts_type_name_to_string(&imp.expression);
            implements.push(CompactString::new(name));
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
