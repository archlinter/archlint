use crate::parser::line_index::LineIndex;
use compact_str::CompactString;
use oxc_allocator::Allocator;
use oxc_ast::ast;
use oxc_ast::visit::Visit;
use oxc_parser::Parser;
use oxc_span::{GetSpan, SourceType, Span};
use oxc_syntax::scope::ScopeFlags;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    String,
    Number,
    Keyword,
    Operator,
    Punctuation,
    Other,
}

#[derive(Debug, Clone)]
pub struct NormalizedToken {
    pub kind: TokenKind,
    pub normalized: CompactString,
    pub span: Span,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    /// Stable order key to disambiguate same-span tokens
    pub seq: u32,
}

pub struct TokenCollector {
    pub tokens: Vec<NormalizedToken>,
    line_index: LineIndex,
    source: Arc<str>,
    seq: u32,
}

impl TokenCollector {
    pub fn new(source: Arc<str>) -> Self {
        Self {
            tokens: Vec::new(),
            line_index: LineIndex::new(&source),
            source,
            seq: 0,
        }
    }

    fn add_token(&mut self, kind: TokenKind, normalized: CompactString, span: Span) {
        let (line, column) = self.line_index.line_col(span.start as usize);
        let (end_line, end_column) = self.line_index.line_col(span.end as usize);
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(1);
        self.tokens.push(NormalizedToken {
            kind,
            normalized,
            span,
            line,
            column,
            end_line,
            end_column,
            seq,
        });
    }

    #[inline]
    fn add_marker(&mut self, marker: &'static str, span: Span) {
        self.add_token(TokenKind::Keyword, CompactString::from(marker), span);
    }

    #[inline]
    fn slice(&self, span: Span) -> &str {
        let start = span.start as usize;
        let end = span.end as usize;
        self.source.get(start..end).unwrap_or("")
    }
}

impl<'a> Visit<'a> for TokenCollector {
    fn visit_identifier_reference(&mut self, it: &ast::IdentifierReference<'a>) {
        self.add_token(
            TokenKind::Identifier,
            CompactString::from(self.slice(it.span)),
            it.span,
        );
    }

    fn visit_binding_identifier(&mut self, it: &ast::BindingIdentifier<'a>) {
        self.add_token(
            TokenKind::Identifier,
            CompactString::from(self.slice(it.span)),
            it.span,
        );
    }

    fn visit_identifier_name(&mut self, it: &ast::IdentifierName<'a>) {
        self.add_token(
            TokenKind::Identifier,
            CompactString::from(self.slice(it.span)),
            it.span,
        );
    }

    fn visit_ts_type_name(&mut self, it: &ast::TSTypeName<'a>) {
        self.add_token(
            TokenKind::Identifier,
            CompactString::from(self.slice(it.span())),
            it.span(),
        );
        // Do not call walk_ts_type_name to avoid redundant tokens for parts of qualified names
    }

    fn visit_string_literal(&mut self, it: &ast::StringLiteral<'a>) {
        self.add_token(
            TokenKind::String,
            CompactString::from(self.slice(it.span)),
            it.span,
        );
    }

    fn visit_numeric_literal(&mut self, it: &ast::NumericLiteral<'a>) {
        self.add_token(
            TokenKind::Number,
            CompactString::from(self.slice(it.span)),
            it.span,
        );
    }

    fn visit_call_expression(&mut self, it: &ast::CallExpression<'a>) {
        self.add_marker("$CALL", it.span);
        oxc_ast::visit::walk::walk_call_expression(self, it);
    }

    fn visit_new_expression(&mut self, it: &ast::NewExpression<'a>) {
        self.add_marker("$NEW", it.span);
        oxc_ast::visit::walk::walk_new_expression(self, it);
    }

    fn visit_await_expression(&mut self, it: &ast::AwaitExpression<'a>) {
        self.add_marker("$AWAIT", it.span);
        oxc_ast::visit::walk::walk_await_expression(self, it);
    }

    fn visit_return_statement(&mut self, it: &ast::ReturnStatement<'a>) {
        self.add_marker("$RETURN", it.span);
        oxc_ast::visit::walk::walk_return_statement(self, it);
    }

    fn visit_if_statement(&mut self, it: &ast::IfStatement<'a>) {
        self.add_marker("$IF", it.span);
        oxc_ast::visit::walk::walk_if_statement(self, it);
    }

    fn visit_object_expression(&mut self, it: &ast::ObjectExpression<'a>) {
        self.add_marker("$OBJ", it.span);
        oxc_ast::visit::walk::walk_object_expression(self, it);
    }

    fn visit_assignment_expression(&mut self, it: &ast::AssignmentExpression<'a>) {
        self.add_marker("$ASSIGN", it.span);
        oxc_ast::visit::walk::walk_assignment_expression(self, it);
    }

    fn visit_binary_expression(&mut self, it: &ast::BinaryExpression<'a>) {
        let op_marker = match it.operator {
            ast::BinaryOperator::Addition => "$ADD",
            ast::BinaryOperator::Subtraction => "$SUB",
            ast::BinaryOperator::Multiplication => "$MUL",
            ast::BinaryOperator::Division => "$DIV",
            ast::BinaryOperator::Remainder => "$MOD",
            ast::BinaryOperator::Exponential => "$EXP",
            ast::BinaryOperator::BitwiseAnd => "$BITAND",
            ast::BinaryOperator::BitwiseOR => "$BITOR",
            ast::BinaryOperator::BitwiseXOR => "$BITXOR",
            ast::BinaryOperator::ShiftLeft => "$SHL",
            ast::BinaryOperator::ShiftRight => "$SHR",
            ast::BinaryOperator::ShiftRightZeroFill => "$SHR_U",
            ast::BinaryOperator::Equality => "$EQ",
            ast::BinaryOperator::Inequality => "$NE",
            ast::BinaryOperator::StrictEquality => "$SEQ",
            ast::BinaryOperator::StrictInequality => "$SNE",
            ast::BinaryOperator::LessThan => "$LT",
            ast::BinaryOperator::LessEqualThan => "$LE",
            ast::BinaryOperator::GreaterThan => "$GT",
            ast::BinaryOperator::GreaterEqualThan => "$GE",
            ast::BinaryOperator::In => "$IN",
            ast::BinaryOperator::Instanceof => "$INSTANCEOF",
        };
        self.add_marker(op_marker, it.span);
        oxc_ast::visit::walk::walk_binary_expression(self, it);
    }

    fn visit_member_expression(&mut self, it: &ast::MemberExpression<'a>) {
        // Member access is high-signal for clone detection
        match it {
            ast::MemberExpression::StaticMemberExpression(s) => {
                self.add_marker("$MEMBER", s.span);
            }
            ast::MemberExpression::ComputedMemberExpression(c) => {
                self.add_marker("$MEMBER", c.span);
            }
            ast::MemberExpression::PrivateFieldExpression(p) => {
                self.add_marker("$MEMBER", p.span);
            }
        }
        oxc_ast::visit::walk::walk_member_expression(self, it);
    }

    fn visit_this_expression(&mut self, it: &ast::ThisExpression) {
        self.add_marker("this", it.span);
    }

    fn visit_super(&mut self, it: &ast::Super) {
        self.add_marker("super", it.span);
    }

    fn visit_property_key(&mut self, it: &ast::PropertyKey<'a>) {
        self.add_marker("$KEY", it.span());
        oxc_ast::visit::walk::walk_property_key(self, it);
    }

    fn visit_spread_element(&mut self, it: &ast::SpreadElement<'a>) {
        self.add_marker("...", it.span);
        oxc_ast::visit::walk::walk_spread_element(self, it);
    }

    fn visit_array_expression(&mut self, it: &ast::ArrayExpression<'a>) {
        self.add_marker("$ARRAY", it.span);
        oxc_ast::visit::walk::walk_array_expression(self, it);
    }

    fn visit_variable_declaration(&mut self, it: &ast::VariableDeclaration<'a>) {
        self.add_marker("$VAR", it.span);
        oxc_ast::visit::walk::walk_variable_declaration(self, it);
    }

    fn visit_method_definition(&mut self, it: &ast::MethodDefinition<'a>) {
        self.add_marker("$METHOD", it.span);
        // Skip the key (method name) to detect duplicate bodies regardless of name
        // Only visit the function value (body) for clone detection
        self.visit_function(&it.value, ScopeFlags::empty());
    }

    fn visit_function(&mut self, it: &ast::Function<'a>, flags: ScopeFlags) {
        self.add_marker("$FUNC", it.span);
        oxc_ast::visit::walk::walk_function(self, it, flags);
    }

    fn visit_arrow_function_expression(&mut self, it: &ast::ArrowFunctionExpression<'a>) {
        self.add_marker("$ARROW", it.span);
        oxc_ast::visit::walk::walk_arrow_function_expression(self, it);
    }

    fn visit_while_statement(&mut self, it: &ast::WhileStatement<'a>) {
        self.add_marker("$WHILE", it.span);
        oxc_ast::visit::walk::walk_while_statement(self, it);
    }

    fn visit_for_statement(&mut self, it: &ast::ForStatement<'a>) {
        self.add_marker("$FOR", it.span);
        oxc_ast::visit::walk::walk_for_statement(self, it);
    }

    fn visit_for_in_statement(&mut self, it: &ast::ForInStatement<'a>) {
        self.add_marker("$FOR", it.span);
        oxc_ast::visit::walk::walk_for_in_statement(self, it);
    }

    fn visit_for_of_statement(&mut self, it: &ast::ForOfStatement<'a>) {
        self.add_marker("$FOR", it.span);
        oxc_ast::visit::walk::walk_for_of_statement(self, it);
    }

    fn visit_try_statement(&mut self, it: &ast::TryStatement<'a>) {
        self.add_marker("$TRY", it.span);
        oxc_ast::visit::walk::walk_try_statement(self, it);
    }

    fn visit_catch_clause(&mut self, it: &ast::CatchClause<'a>) {
        self.add_marker("$CATCH", it.span);
        oxc_ast::visit::walk::walk_catch_clause(self, it);
    }

    fn visit_switch_statement(&mut self, it: &ast::SwitchStatement<'a>) {
        self.add_marker("$SWITCH", it.span);
        oxc_ast::visit::walk::walk_switch_statement(self, it);
    }

    fn visit_import_declaration(&mut self, _it: &ast::ImportDeclaration<'a>) {
        // Skip import statements to avoid false positives from identical import lists
    }

    fn visit_export_named_declaration(&mut self, it: &ast::ExportNamedDeclaration<'a>) {
        if let Some(decl) = &it.declaration {
            self.visit_declaration(decl);
        }
    }

    fn visit_export_all_declaration(&mut self, _it: &ast::ExportAllDeclaration<'a>) {
        // Skip "export * from '...'"
    }

    fn visit_export_default_declaration(&mut self, it: &ast::ExportDefaultDeclaration<'a>) {
        self.visit_export_default_declaration_kind(&it.declaration);
    }
}

pub fn tokenize_and_normalize(source: Arc<str>, source_type: SourceType) -> Vec<NormalizedToken> {
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, &source, source_type);
    let ret = parser.parse();

    if !ret.errors.is_empty() {
        return Vec::new();
    }

    let mut collector = TokenCollector::new(source.clone());
    collector.visit_program(&ret.program);

    // Sort tokens by span start to ensure they are in source order
    collector.tokens.sort_by_key(|t| (t.span.start, t.seq));

    collector.tokens
}
