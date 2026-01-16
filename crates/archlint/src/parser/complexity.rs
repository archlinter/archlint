use oxc_ast::ast::{ArrowFunctionExpression, Function, LogicalOperator, Statement};
use oxc_ast_visit::Visit;

pub struct ComplexityVisitor {
    pub cyclomatic: usize,
    pub cognitive: usize,
    pub max_depth: usize,
    current_depth: usize,
    current_nesting: usize,
    current_logical_op: Option<LogicalOperator>,
}

impl Default for ComplexityVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplexityVisitor {
    pub fn new() -> Self {
        Self {
            cyclomatic: 1,
            cognitive: 0,
            max_depth: 0,
            current_depth: 0,
            current_nesting: 0,
            current_logical_op: None,
        }
    }

    fn enter_nesting<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.current_depth += 1;
        if self.current_depth > self.max_depth {
            self.max_depth = self.current_depth;
        }
        let old_nesting = self.current_nesting;
        self.current_nesting += 1;
        f(self);
        self.current_nesting = old_nesting;
        self.current_depth -= 1;
    }
}

pub fn calculate_complexity(func: &Function<'_>) -> (usize, usize, usize) {
    let mut visitor = ComplexityVisitor::new();
    if let Some(body) = &func.body {
        visitor.visit_function_body(body);
    }
    (visitor.cyclomatic, visitor.cognitive, visitor.max_depth)
}

pub fn calculate_arrow_complexity(func: &ArrowFunctionExpression<'_>) -> (usize, usize, usize) {
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_function_body(&func.body);
    (visitor.cyclomatic, visitor.cognitive, visitor.max_depth)
}

impl<'a> Visit<'a> for ComplexityVisitor {
    fn visit_function(
        &mut self,
        _it: &oxc_ast::ast::Function<'a>,
        _flags: oxc_syntax::scope::ScopeFlags,
    ) {
    }

    fn visit_arrow_function_expression(&mut self, _it: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
    }

    fn visit_class(&mut self, _it: &oxc_ast::ast::Class<'a>) {}

    fn visit_if_statement(&mut self, it: &oxc_ast::ast::IfStatement<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;

        // Ensure logical operators in the condition are counted
        self.visit_expression(&it.test);

        self.enter_nesting(|v| {
            v.visit_statement(&it.consequent);
        });

        if let Some(alternate) = &it.alternate {
            // Cyclomatic: doesn't count 'else' itself, only branching points.
            // Cognitive: +1 for 'else' or 'else if'.
            if let Statement::IfStatement(_) = alternate {
                // For 'else if', we don't increase nesting level for the alternate itself,
                // the nested visit_if_statement will handle its own structural increment.
                self.visit_statement(alternate);
            } else {
                self.cognitive += 1;
                self.enter_nesting(|v| {
                    v.visit_statement(alternate);
                });
            }
        }
    }

    fn visit_while_statement(&mut self, it: &oxc_ast::ast::WhileStatement<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_while_statement(v, it);
        });
    }

    fn visit_do_while_statement(&mut self, it: &oxc_ast::ast::DoWhileStatement<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_do_while_statement(v, it);
        });
    }

    fn visit_for_statement(&mut self, it: &oxc_ast::ast::ForStatement<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_for_statement(v, it);
        });
    }

    fn visit_for_in_statement(&mut self, it: &oxc_ast::ast::ForInStatement<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_for_in_statement(v, it);
        });
    }

    fn visit_for_of_statement(&mut self, it: &oxc_ast::ast::ForOfStatement<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_for_of_statement(v, it);
        });
    }

    fn visit_switch_statement(&mut self, it: &oxc_ast::ast::SwitchStatement<'a>) {
        // Cognitive: +1 for the whole switch
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_switch_statement(v, it);
        });
    }

    fn visit_switch_case(&mut self, it: &oxc_ast::ast::SwitchCase<'a>) {
        // Cyclomatic: each case counts
        self.cyclomatic += 1;
        oxc_ast_visit::walk::walk_switch_case(self, it);
    }

    fn visit_catch_clause(&mut self, it: &oxc_ast::ast::CatchClause<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_catch_clause(v, it);
        });
    }

    fn visit_conditional_expression(&mut self, it: &oxc_ast::ast::ConditionalExpression<'a>) {
        self.cyclomatic += 1;
        self.cognitive += 1 + self.current_nesting;
        self.enter_nesting(|v| {
            oxc_ast_visit::walk::walk_conditional_expression(v, it);
        });
    }

    fn visit_logical_expression(&mut self, it: &oxc_ast::ast::LogicalExpression<'a>) {
        self.cyclomatic += 1;

        // Cognitive: sequences of same operators count as 1.
        let is_new_sequence = self.current_logical_op != Some(it.operator);
        if is_new_sequence {
            self.cognitive += 1;
        }

        let old_op = self.current_logical_op;
        self.current_logical_op = Some(it.operator);
        oxc_ast_visit::walk::walk_logical_expression(self, it);
        self.current_logical_op = old_op;
    }

    fn visit_assignment_expression(&mut self, it: &oxc_ast::ast::AssignmentExpression<'a>) {
        use oxc_ast::ast::AssignmentOperator;
        if matches!(
            it.operator,
            AssignmentOperator::LogicalAnd
                | AssignmentOperator::LogicalOr
                | AssignmentOperator::LogicalNullish
        ) {
            self.cyclomatic += 1;
            self.cognitive += 1;
        }
        oxc_ast_visit::walk::walk_assignment_expression(self, it);
    }

    fn visit_break_statement(&mut self, it: &oxc_ast::ast::BreakStatement<'a>) {
        if it.label.is_some() {
            self.cognitive += 1;
        }
    }

    fn visit_continue_statement(&mut self, it: &oxc_ast::ast::ContinueStatement<'a>) {
        if it.label.is_some() {
            self.cognitive += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    fn parse_function(code: &str) -> (usize, usize, usize) {
        let allocator = Allocator::default();
        let source_type = SourceType::default().with_typescript(true);
        let ret = Parser::new(&allocator, code, source_type).parse();

        let program = ret.program;
        for stmt in &program.body {
            if let oxc_ast::ast::Statement::FunctionDeclaration(func) = stmt {
                return calculate_complexity(func);
            }
        }
        (0, 0, 0)
    }

    #[test]
    fn test_cyclomatic_basics() {
        let code = r#"
            function test(a, b) {
                if (a) { // +1
                    while (b) { // +1
                        console.log(1);
                    }
                }
            }
        "#;
        let (cc, cog, depth) = parse_function(code);
        assert_eq!(cc, 3);
        assert_eq!(cog, 3); // if (+1), while (+2)
        assert_eq!(depth, 2);
    }

    #[test]
    fn test_cognitive_nesting() {
        let code = r#"
            function test(a, b, c) {
                if (a) { // +1
                    if (b) { // +2
                        if (c) { // +3
                            return 1;
                        }
                    }
                }
            }
        "#;
        let (cc, cog, _) = parse_function(code);
        assert_eq!(cc, 4);
        assert_eq!(cog, 6); // 1 + 2 + 3
    }

    #[test]
    fn test_else_if_logic() {
        let code = r#"
            function test(a, b) {
                if (a) { // +1
                    return 1;
                } else if (b) { // +1 (cc), +1 (cog: structural only, no nesting)
                    return 2;
                } else { // +0 (cc), +1 (cog structural)
                    return 3;
                }
            }
        "#;
        let (cc, cog, _) = parse_function(code);
        assert_eq!(cc, 3);
        assert_eq!(cog, 3); // if (+1), else if (+1), else (+1)
    }
}
