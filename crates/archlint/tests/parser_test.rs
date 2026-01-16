use archlint::parser::ImportParser;
use compact_str::CompactString;

#[test]
fn test_local_usages_in_interface_property() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
export interface A { name: string; }
export interface B { ref: A; }
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    println!(
        "Exports: {:?}",
        result
            .symbols
            .exports
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>()
    );
    println!("Local usages: {:?}", result.symbols.local_usages);

    assert!(
        result.symbols.local_usages.contains("A"),
        "Type A used in property should be in local_usages"
    );
}

#[test]
fn test_local_usages_in_extends() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
export interface A { name: string; }
export interface B extends A { extra: number; }
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    println!(
        "Exports: {:?}",
        result
            .symbols
            .exports
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>()
    );
    println!("Local usages: {:?}", result.symbols.local_usages);

    assert!(
        result.symbols.local_usages.contains("A"),
        "Type A used in extends should be in local_usages"
    );
}

#[test]
fn test_local_usages_in_type_union() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
export interface A { name: string; }
export interface B { value: number; }
export type C = A | B;
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    println!(
        "Exports: {:?}",
        result
            .symbols
            .exports
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>()
    );
    println!("Local usages: {:?}", result.symbols.local_usages);

    assert!(
        result.symbols.local_usages.contains("A"),
        "Type A in union should be in local_usages"
    );
    assert!(
        result.symbols.local_usages.contains("B"),
        "Type B in union should be in local_usages"
    );
}

#[test]
fn test_parse_simple_import() {
    let parser = ImportParser::new().unwrap();
    let code = r#"import { foo } from './bar';"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    assert_eq!(result.symbols.imports.len(), 1);
    assert_eq!(result.symbols.imports[0].name, "foo");
    assert_eq!(result.symbols.imports[0].source, "./bar");
}

#[test]
fn test_parse_default_import() {
    let parser = ImportParser::new().unwrap();
    let code = r#"import foo from './bar';"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    assert_eq!(result.symbols.imports.len(), 1);
    assert_eq!(result.symbols.imports[0].name, "default");
    assert_eq!(
        result.symbols.imports[0].alias,
        Some(CompactString::from("foo"))
    );
    assert_eq!(result.symbols.imports[0].source, "./bar");
}

#[test]
fn test_parse_namespace_import() {
    let parser = ImportParser::new().unwrap();
    let code = r#"import * as foo from './bar';"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    assert_eq!(result.symbols.imports.len(), 1);
    assert_eq!(result.symbols.imports[0].name, "*");
    assert_eq!(
        result.symbols.imports[0].alias,
        Some(CompactString::from("foo"))
    );
    assert_eq!(result.symbols.imports[0].source, "./bar");
}

#[test]
fn test_parse_reexport() {
    let parser = ImportParser::new().unwrap();
    let code = r#"export { foo } from './bar';"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    assert_eq!(result.symbols.exports.len(), 1);
    assert_eq!(result.symbols.exports[0].name, "foo");
    assert_eq!(
        result.symbols.exports[0].source,
        Some(CompactString::from("./bar"))
    );
    assert!(result.symbols.exports[0].is_reexport);
}

#[test]
fn test_complexity_calculation() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
        function complex(x: number) {
            if (x > 0) {
                for (let i = 0; i < x; i++) {
                    if (i % 2 === 0) {
                        console.log(i);
                    }
                }
            }
        }
    "#;
    let result = parser.parse_code(code, "test.ts").unwrap();
    assert_eq!(result.functions.len(), 1);
    assert_eq!(result.functions[0].cyclomatic_complexity, 4); // 1 + if + for + if
}

#[test]
fn test_metrics_types_file() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
export interface MetricsDefaultMetricsConfig {
  enabled: boolean;
}

export interface MetricsPushConfig {
  enabled: boolean;
  url?: string;
}

export interface MetricsPullConfig {
  enabled: boolean;
  port: number;
}

export interface MetricsConfig {
  defaultMetrics: MetricsDefaultMetricsConfig;
  mode: 'push' | 'pull';
  push: MetricsPushConfig;
  pull: MetricsPullConfig;
}
"#;
    let result = parser.parse_code(code, "types.ts").unwrap();

    println!(
        "Exports: {:?}",
        result
            .symbols
            .exports
            .iter()
            .map(|e| e.name.as_str())
            .collect::<Vec<_>>()
    );
    println!("Local usages: {:?}", result.symbols.local_usages);

    assert!(result.symbols.local_usages.contains("MetricsDefaultMetricsConfig"),
            "MetricsDefaultMetricsConfig used in MetricsConfig.defaultMetrics should be in local_usages");
    assert!(
        result.symbols.local_usages.contains("MetricsPullConfig"),
        "MetricsPullConfig used in MetricsConfig.pull should be in local_usages"
    );
}
