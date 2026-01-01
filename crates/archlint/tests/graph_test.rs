use archlint::graph::{DependencyGraph, EdgeData};
use std::path::PathBuf;

#[test]
fn test_add_file() {
    let mut graph = DependencyGraph::new();
    let path = PathBuf::from("/src/a.ts");
    let node = graph.add_file(&path);

    assert_eq!(graph.node_count(), 1);
    assert_eq!(graph.get_file_path(node).unwrap(), &path);
}

#[test]
fn test_add_duplicate_file() {
    let mut graph = DependencyGraph::new();
    let path = PathBuf::from("/src/a.ts");
    let node1 = graph.add_file(&path);
    let node2 = graph.add_file(&path);

    assert_eq!(graph.node_count(), 1);
    assert_eq!(node1, node2);
}

#[test]
fn test_add_dependency() {
    let mut graph = DependencyGraph::new();
    let a = graph.add_file(PathBuf::from("/a.ts"));
    let b = graph.add_file(PathBuf::from("/b.ts"));

    let edge_data = EdgeData::with_symbols(1, vec!["foo".to_string()]);
    graph.add_dependency(a, b, edge_data);

    assert_eq!(graph.edge_count(), 1);
    let data = graph.get_edge_data(a, b).unwrap();
    assert_eq!(data.import_line, 1);
    assert_eq!(data.imported_symbols, vec!["foo".to_string()]);
}

#[test]
fn test_fan_in_out() {
    let mut graph = DependencyGraph::new();
    let a = graph.add_file(PathBuf::from("/a.ts"));
    let b = graph.add_file(PathBuf::from("/b.ts"));
    let c = graph.add_file(PathBuf::from("/c.ts"));

    graph.add_dependency(a, b, EdgeData::new(1));
    graph.add_dependency(a, c, EdgeData::new(2));
    graph.add_dependency(b, c, EdgeData::new(3));

    assert_eq!(graph.fan_out(a), 2);
    assert_eq!(graph.fan_out(b), 1);
    assert_eq!(graph.fan_out(c), 0);

    assert_eq!(graph.fan_in(a), 0);
    assert_eq!(graph.fan_in(b), 1);
    assert_eq!(graph.fan_in(c), 2);
}

#[test]
fn test_get_node() {
    let mut graph = DependencyGraph::new();
    let path = PathBuf::from("/a.ts");
    let node = graph.add_file(&path);

    assert_eq!(graph.get_node(&path), Some(node));
    assert_eq!(graph.get_node(&PathBuf::from("/b.ts")), None);
}
