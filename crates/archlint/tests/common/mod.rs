use archlint::config::{Config, RuleConfig, RuleFullConfig};
use archlint::engine::{context::FileMetrics, AnalysisContext};
use archlint::graph::DependencyGraph;
use archlint::package_json::PackageJsonParser;
use archlint::parser::ImportParser;
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

    for file in &files {
        graph.add_file(file);
    }

    for file in &files {
        if let Ok(parsed) = parser.parse_file(file) {
            let from_node = graph.get_node(file).unwrap();

            file_symbols.insert(file.clone(), parsed.symbols.clone());
            function_complexity.insert(file.clone(), parsed.functions.clone());
            file_metrics.insert(
                file.clone(),
                FileMetrics {
                    lines: parsed.lines,
                },
            );

            for import in parsed.symbols.imports {
                if let Ok(Some(resolved)) = resolver.resolve(&import.source, file) {
                    let to_node = graph.add_file(&resolved);
                    graph.add_dependency(
                        from_node,
                        to_node,
                        archlint::graph::EdgeData::new(import.line),
                    );
                }
            }
        }
    }

    let package_config =
        PackageJsonParser::parse(&root).unwrap_or(archlint::package_json::PackageConfig {
            entry_points: HashSet::new(),
            dynamic_load_patterns: Vec::new(),
        });
    let script_entry_points = package_config.entry_points;
    let dynamic_load_patterns = package_config.dynamic_load_patterns;

    AnalysisContext {
        project_path: root,
        graph: Arc::new(graph),
        file_symbols: Arc::new(file_symbols),
        function_complexity: Arc::new(function_complexity),
        file_metrics: Arc::new(file_metrics),
        churn_map: HashMap::new(),
        config,
        script_entry_points,
        dynamic_load_patterns,
        detected_frameworks: Vec::new(),
        file_types: HashMap::new(),
    }
}

#[allow(dead_code)]
pub fn create_config_with_rule(rule_name: &str, options_yaml: Option<&str>) -> Config {
    let mut config = Config::default();
    let options = options_yaml
        .map(|s| serde_yaml::from_str(s).expect("Failed to parse options YAML"))
        .unwrap_or(serde_yaml::Value::Mapping(serde_yaml::Mapping::new()));

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
