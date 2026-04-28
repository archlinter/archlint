use archlint::config::{Config, RuleConfig, RuleFullConfig};
use archlint::engine::{context::FileMetrics, AnalysisContext};
use archlint::graph::DependencyGraph;
use archlint::package_json::PackageJsonParser;
use archlint::parser::{FileIgnoredLines, ImportParser};
use archlint::resolver::PathResolver;
use archlint::scanner::FileScanner;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join(name)
}

#[allow(dead_code)]
pub fn analyze_fixture(name: &str) -> AnalysisContext {
    analyze_fixture_with_config(name, Config::default())
}

pub fn analyze_fixture_with_config(name: &str, config: Config) -> AnalysisContext {
    let root = fixture_path(name);

    let parser = ImportParser::new().unwrap();
    let resolver = PathResolver::new(&root, &config);
    let scanner = FileScanner::new(
        &root,
        &root,
        vec![
            "ts".to_string(),
            "tsx".to_string(),
            "js".to_string(),
            "jsx".to_string(),
        ],
    );

    let files = scanner.scan().unwrap();
    let mut graph = DependencyGraph::new();
    let mut file_symbols = HashMap::new();
    let mut function_complexity = HashMap::new();
    let mut file_metrics = HashMap::new();
    let mut ignored_lines = FileIgnoredLines::default();

    for file in &files {
        graph.add_file(file);
    }

    for file in &files {
        let Ok(parsed) = parser.parse_file(file) else {
            continue;
        };
        let symbols = resolve_imports(parsed.symbols.clone(), file, &resolver);

        file_symbols.insert(file.clone(), symbols.clone());
        function_complexity.insert(file.clone(), parsed.functions.clone());
        file_metrics.insert(
            file.clone(),
            FileMetrics {
                lines: parsed.lines,
            },
        );
        if !parsed.ignored_lines.is_empty() {
            ignored_lines.insert(file.clone(), parsed.ignored_lines.clone());
        }

        add_graph_edges(&symbols, file, &mut graph);
    }

    let package_config =
        PackageJsonParser::parse(&root).unwrap_or(archlint::package_json::PackageConfig {
            entry_points: HashSet::new(),
            dynamic_load_patterns: Vec::new(),
            package_json_dirs: HashSet::new(),
        });

    AnalysisContext {
        project_path: root,
        graph: Arc::new(graph),
        file_symbols: Arc::new(file_symbols),
        function_complexity: Arc::new(function_complexity),
        file_metrics: Arc::new(file_metrics),
        ignored_lines: Arc::new(ignored_lines),
        churn_map: HashMap::new(),
        config,
        script_entry_points: package_config.entry_points,
        dynamic_load_patterns: package_config.dynamic_load_patterns,
        detected_frameworks: Vec::new(),
        presets: Vec::new(),
        package_json_dirs: package_config.package_json_dirs,
    }
}

fn resolve_imports(
    mut symbols: archlint::parser::FileSymbols,
    file: &std::path::Path,
    resolver: &PathResolver,
) -> archlint::parser::FileSymbols {
    for import in &mut symbols.imports {
        if let Ok(Some(resolved)) = resolver.resolve(&import.source, file) {
            import.source = resolved.to_string_lossy().to_string().into();
        }
    }
    symbols
}

fn add_graph_edges(
    symbols: &archlint::parser::FileSymbols,
    file: &std::path::Path,
    graph: &mut DependencyGraph,
) {
    let from_node = graph.get_node(file).unwrap();
    for import in &symbols.imports {
        let source_path = PathBuf::from(import.source.as_str());
        if source_path.is_absolute() {
            let to_node = graph.add_file(&source_path);
            graph.add_dependency(
                from_node,
                to_node,
                archlint::graph::EdgeData::with_symbols(import.line, vec![import.name.to_string()]),
            );
        }
    }
}

#[allow(dead_code)]
pub fn create_config_with_rule(rule_name: &str, options_yaml: Option<&str>) -> Config {
    let mut config = Config::default();
    let options = options_yaml.map_or(
        serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
        |s| serde_yaml::from_str(s).expect("Failed to parse options YAML"),
    );

    config.rules.insert(
        rule_name.to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            severity: None,
            exclude: Vec::new(),
            options,
        }),
    );
    config
}

#[allow(dead_code)]
pub fn analyze_fixture_with_rule(
    fixture: &str,
    rule: &str,
    options: Option<&str>,
) -> AnalysisContext {
    let config = create_config_with_rule(rule, options);
    analyze_fixture_with_config(fixture, config)
}

/// Helper to create `dead_code` detector with exclude patterns
#[allow(dead_code)]
pub fn create_dead_code_config(exclude: Vec<String>) -> Config {
    let mut rules = HashMap::new();
    rules.insert(
        "dead_code".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            exclude,
            ..Default::default()
        }),
    );

    Config {
        rules,
        entry_points: vec!["main.ts".to_string()],
        ..Default::default()
    }
}
