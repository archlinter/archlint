use oxc_ast::ast::{ArrowFunctionExpression, Function};
use oxc_ast_visit::Visit;

pub struct ComplexityVisitor {
    pub complexity: usize,
    pub current_depth: usize,
    pub max_depth: usize,
}

impl Default for ComplexityVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplexityVisitor {
    pub fn new() -> Self {
        Self {
            complexity: 1,
            current_depth: 0,
            max_depth: 0,
        }
    }

    fn enter_nesting(&mut self) {
        self.current_depth += 1;
        if self.current_depth > self.max_depth {
            self.max_depth = self.current_depth;
        }
    }

    fn exit_nesting(&mut self) {
        self.current_depth -= 1;
    }
}

pub fn calculate_complexity(func: &Function<'_>) -> (usize, usize) {
    let mut visitor = ComplexityVisitor::new();
    if let Some(body) = &func.body {
        visitor.visit_function_body(body);
    }
    (visitor.complexity, visitor.max_depth)
}

pub fn calculate_arrow_complexity(func: &ArrowFunctionExpression<'_>) -> (usize, usize) {
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_function_body(&func.body);
    (visitor.complexity, visitor.max_depth)
}

impl<'a> Visit<'a> for ComplexityVisitor {
    fn visit_if_statement(&mut self, it: &oxc_ast::ast::IfStatement<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_if_statement(self, it);
        self.exit_nesting();
    }

    fn visit_while_statement(&mut self, it: &oxc_ast::ast::WhileStatement<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_while_statement(self, it);
        self.exit_nesting();
    }

    fn visit_do_while_statement(&mut self, it: &oxc_ast::ast::DoWhileStatement<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_do_while_statement(self, it);
        self.exit_nesting();
    }

    fn visit_for_statement(&mut self, it: &oxc_ast::ast::ForStatement<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_for_statement(self, it);
        self.exit_nesting();
    }

    fn visit_for_in_statement(&mut self, it: &oxc_ast::ast::ForInStatement<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_for_in_statement(self, it);
        self.exit_nesting();
    }

    fn visit_for_of_statement(&mut self, it: &oxc_ast::ast::ForOfStatement<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_for_of_statement(self, it);
        self.exit_nesting();
    }

    fn visit_switch_case(&mut self, it: &oxc_ast::ast::SwitchCase<'a>) {
        if it.test.is_some() {
            self.complexity += 1;
            self.enter_nesting();
            oxc_ast_visit::walk::walk_switch_case(self, it);
            self.exit_nesting();
        } else {
            oxc_ast_visit::walk::walk_switch_case(self, it);
        }
    }

    fn visit_catch_clause(&mut self, it: &oxc_ast::ast::CatchClause<'a>) {
        self.complexity += 1;
        self.enter_nesting();
        oxc_ast_visit::walk::walk_catch_clause(self, it);
        self.exit_nesting();
    }

    fn visit_conditional_expression(&mut self, it: &oxc_ast::ast::ConditionalExpression<'a>) {
        self.complexity += 1;
        oxc_ast_visit::walk::walk_conditional_expression(self, it);
    }

    fn visit_logical_expression(&mut self, it: &oxc_ast::ast::LogicalExpression<'a>) {
        self.complexity += 1;
        oxc_ast_visit::walk::walk_logical_expression(self, it);
    }

    fn visit_static_member_expression(&mut self, it: &oxc_ast::ast::StaticMemberExpression<'a>) {
        if it.optional {
            self.complexity += 1;
        }
        oxc_ast_visit::walk::walk_static_member_expression(self, it);
    }

    fn visit_computed_member_expression(
        &mut self,
        it: &oxc_ast::ast::ComputedMemberExpression<'a>,
    ) {
        if it.optional {
            self.complexity += 1;
        }
        oxc_ast_visit::walk::walk_computed_member_expression(self, it);
    }

    fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
        if it.optional {
            self.complexity += 1;
        }
        oxc_ast_visit::walk::walk_call_expression(self, it);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    fn parse_function(code: &str) -> (usize, usize) {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_typescript(true);
        let ret = Parser::new(&allocator, code, source_type).parse();

        let program = ret.program;
        for stmt in &program.body {
            if let oxc_ast::ast::Statement::FunctionDeclaration(func) = stmt {
                return calculate_complexity(func);
            }
            if let oxc_ast::ast::Statement::ExpressionStatement(expr_stmt) = stmt {
                if let oxc_ast::ast::Expression::ArrowFunctionExpression(arrow) =
                    &expr_stmt.expression
                {
                    return calculate_arrow_complexity(arrow);
                }
            }
        }
        (0, 0)
    }

    #[test]
    fn test_empty_function() {
        let (complexity, max_depth) = parse_function("function test() {}");
        assert_eq!(complexity, 1);
        assert_eq!(max_depth, 0);
    }

    #[test]
    fn test_nested_if_while() {
        let code = r#"
            function test(x) {
                if (x > 0) {
                    while (x < 10) {
                        x++;
                    }
                }
            }
        "#;
        let (complexity, max_depth) = parse_function(code);
        assert_eq!(complexity, 3); // 1 + if + while
        assert_eq!(max_depth, 2);
    }

    #[test]
    fn test_switch_case_complexity() {
        let code = r#"
            function test(x) {
                switch(x) {
                    case 1: return 1;
                    case 2: return 2;
                    default: return 0;
                }
            }
        "#;
        let (complexity, max_depth) = parse_function(code);
        assert_eq!(complexity, 3); // 1 + case 1 + case 2
        assert_eq!(max_depth, 1);
    }

    #[test]
    fn test_catch_clause() {
        let code = r#"
            function test() {
                try {
                    doSomething();
                } catch (e) {
                    handleError(e);
                }
            }
        "#;
        let (complexity, max_depth) = parse_function(code);
        assert_eq!(complexity, 2); // 1 + catch
        assert_eq!(max_depth, 1);
    }

    #[test]
    fn test_ternary_operator() {
        let code = r#"
            function test(x) {
                return x > 0 ? 1 : 0;
            }
        "#;
        let (complexity, _max_depth) = parse_function(code);
        assert_eq!(complexity, 2); // 1 + ternary
    }

    #[test]
    fn test_logical_operators() {
        let code = r#"
            function test(a, b) {
                if (a && b || a) {
                    return 1;
                }
                return 0;
            }
        "#;
        let (complexity, _max_depth) = parse_function(code);
        assert_eq!(complexity, 4); // 1 + if + && + ||
    }

    #[test]
    fn test_optional_chaining() {
        let code = r#"
            function test(obj) {
                return obj?.prop?.method()?.();
            }
        "#;
        let (complexity, _max_depth) = parse_function(code);
        assert_eq!(complexity, 4); // 1 + obj?.prop + .method() + .()
    }

    #[test]
    fn test_arrow_function_complexity() {
        let code = "(x) => { if (x) return 1; return 0; }";
        let (complexity, max_depth) = parse_function(code);
        assert_eq!(complexity, 2); // 1 + if
        assert_eq!(max_depth, 1);
    }

    #[test]
    fn test_do_while_complexity() {
        let code = r#"
            function test(x) {
                do {
                    x--;
                } while (x > 0);
            }
        "#;
        let (complexity, max_depth) = parse_function(code);
        assert_eq!(complexity, 2); // 1 + do-while
        assert_eq!(max_depth, 1);
    }

    #[test]
    fn test_for_variants_complexity() {
        let code_for = "function t() { for(let i=0; i<10; i++) {} }";
        let (c1, _) = parse_function(code_for);
        assert_eq!(c1, 2);

        let code_for_in = "function t(obj) { for(let k in obj) {} }";
        let (c2, _) = parse_function(code_for_in);
        assert_eq!(c2, 2);

        let code_for_of = "function t(arr) { for(let v of arr) {} }";
        let (c3, _) = parse_function(code_for_of);
        assert_eq!(c3, 2);
    }
}
