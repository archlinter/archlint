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
    let visitor = parse_code("function test(a: string, b: number, c: any, d: { x: number }) {}");
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
    let visitor = parse_code("class A { constructor(private x: number) {} method() { this.x; } }");
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
