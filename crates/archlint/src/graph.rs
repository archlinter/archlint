use crate::detectors::CodeRange;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
}

/// Data associated with a dependency edge in the graph.
#[derive(Debug, Clone)]
pub struct EdgeData {
    /// Line number where the import occurs.
    pub import_line: usize,
    /// Exact code range of the import statement.
    pub import_range: Option<CodeRange>,
    /// List of symbols imported through this dependency.
    pub imported_symbols: Vec<String>,
}

impl EdgeData {
    /// Create new edge data with only the line number.
    pub fn new(import_line: usize) -> Self {
        Self {
            import_line,
            import_range: None,
            imported_symbols: Vec::new(),
        }
    }

    /// Create edge data with line number and range.
    pub fn with_range(import_line: usize, range: CodeRange) -> Self {
        Self {
            import_line,
            import_range: Some(range),
            imported_symbols: Vec::new(),
        }
    }

    /// Create edge data with line number and imported symbols.
    pub fn with_symbols(import_line: usize, imported_symbols: Vec<String>) -> Self {
        Self {
            import_line,
            import_range: None,
            imported_symbols,
        }
    }

    /// Create edge data with all available information.
    pub fn with_all(import_line: usize, range: CodeRange, imported_symbols: Vec<String>) -> Self {
        Self {
            import_line,
            import_range: Some(range),
            imported_symbols,
        }
    }
}

/// A directed graph representing dependencies between files.
#[derive(Clone)]
pub struct DependencyGraph {
    graph: DiGraph<FileNode, EdgeData>,
    path_to_node: HashMap<PathBuf, NodeIndex>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            path_to_node: HashMap::new(),
        }
    }

    /// Add a file to the graph and return its node index.
    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> NodeIndex {
        let path = path.as_ref().to_path_buf();

        if let Some(&node) = self.path_to_node.get(&path) {
            return node;
        }

        let node = self.graph.add_node(FileNode { path: path.clone() });
        self.path_to_node.insert(path, node);
        node
    }

    pub fn add_dependency(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        edge_data: EdgeData,
    ) -> EdgeIndex {
        if let Some(edge_idx) = self.graph.find_edge(from, to) {
            // Update existing edge
            if let Some(edge_weight) = self.graph.edge_weight_mut(edge_idx) {
                edge_weight
                    .imported_symbols
                    .extend(edge_data.imported_symbols);
            }
            edge_idx
        } else {
            self.graph.add_edge(from, to, edge_data)
        }
    }

    pub fn get_edge_data(&self, from: NodeIndex, to: NodeIndex) -> Option<&EdgeData> {
        self.graph
            .find_edge(from, to)
            .and_then(|edge_idx| self.graph.edge_weight(edge_idx))
    }

    pub fn get_node(&self, path: &Path) -> Option<NodeIndex> {
        self.path_to_node.get(path).copied()
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn fan_in(&self, node: NodeIndex) -> usize {
        self.graph
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .count()
    }

    pub fn fan_out(&self, node: NodeIndex) -> usize {
        self.graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .count()
    }

    pub fn get_file_path(&self, node: NodeIndex) -> Option<&PathBuf> {
        self.graph.node_weight(node).map(|n| &n.path)
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph.node_indices()
    }

    pub fn edges(&self) -> impl Iterator<Item = (NodeIndex, NodeIndex)> + '_ {
        self.graph
            .edge_indices()
            .map(|e| self.graph.edge_endpoints(e).unwrap())
    }

    pub fn graph(&self) -> &DiGraph<FileNode, EdgeData> {
        &self.graph
    }

    pub fn dependencies(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
    }

    pub fn remove_outgoing_edges(&mut self, node: NodeIndex) {
        let edge_indices: Vec<_> = self
            .graph
            .edges_directed(node, petgraph::Direction::Outgoing)
            .map(|e| e.id())
            .collect();

        for edge_idx in edge_indices {
            self.graph.remove_edge(edge_idx);
        }
    }

    pub fn remove_file(&mut self, path: &Path) {
        if let Some(node) = self.path_to_node.remove(path) {
            self.graph.remove_node(node);
            // After remove_node, all indices might change if not using StableGraph.
            // But path_to_node still has old indices.
            // We need to rebuild path_to_node.
            self.rebuild_path_index();
        }
    }

    fn rebuild_path_index(&mut self) {
        self.path_to_node.clear();
        for node in self.graph.node_indices() {
            if let Some(weight) = self.graph.node_weight(node) {
                self.path_to_node.insert(weight.path.clone(), node);
            }
        }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file() {
        let mut graph = DependencyGraph::new();
        let p1 = PathBuf::from("src/a.ts");
        let n1 = graph.add_file(&p1);
        let n2 = graph.add_file(&p1);

        assert_eq!(n1, n2);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.get_file_path(n1), Some(&p1));
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = DependencyGraph::new();
        let n1 = graph.add_file("src/a.ts");
        let n2 = graph.add_file("src/b.ts");

        let edge_data = EdgeData::with_symbols(10, vec!["foo".to_string()]);
        graph.add_dependency(n1, n2, edge_data);

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.fan_out(n1), 1);
        assert_eq!(graph.fan_in(n2), 1);

        let data = graph.get_edge_data(n1, n2).unwrap();
        assert_eq!(data.imported_symbols, vec!["foo"]);
    }

    #[test]
    fn test_duplicate_edge_merges_symbols() {
        let mut graph = DependencyGraph::new();
        let n1 = graph.add_file("src/a.ts");
        let n2 = graph.add_file("src/b.ts");

        graph.add_dependency(n1, n2, EdgeData::with_symbols(10, vec!["foo".to_string()]));
        graph.add_dependency(n1, n2, EdgeData::with_symbols(20, vec!["bar".to_string()]));

        assert_eq!(graph.edge_count(), 1);
        let data = graph.get_edge_data(n1, n2).unwrap();
        assert_eq!(data.imported_symbols, vec!["foo", "bar"]);
    }

    #[test]
    fn test_edge_data_variants() {
        let range = CodeRange {
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 10,
        };

        let d1 = EdgeData::new(1);
        assert_eq!(d1.import_line, 1);
        assert!(d1.import_range.is_none());

        let d2 = EdgeData::with_range(2, range);
        assert_eq!(d2.import_line, 2);
        assert!(d2.import_range.is_some());

        let d3 = EdgeData::with_all(3, range, vec!["x".into()]);
        assert_eq!(d3.imported_symbols.len(), 1);
    }

    #[test]
    fn test_iterators() {
        let mut graph = DependencyGraph::new();
        let n1 = graph.add_file("a.ts");
        let n2 = graph.add_file("b.ts");
        graph.add_dependency(n1, n2, EdgeData::new(1));

        assert_eq!(graph.nodes().count(), 2);
        assert_eq!(graph.edges().count(), 1);
        assert_eq!(graph.dependencies(n1).count(), 1);
    }
}
