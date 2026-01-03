use super::state::IncrementalState;
use crate::cache::hash::file_content_hash;
use crate::graph::EdgeData;
use crate::parser::{ImportParser, ParserConfig};
use crate::resolver::PathResolver;
use crate::Result;
use std::path::{Path, PathBuf};

impl IncrementalState {
    /// Update state for changed files
    pub fn update_files(
        &mut self,
        changed: &[PathBuf],
        parser: &ImportParser,
        parser_config: &ParserConfig,
        resolver: &PathResolver,
    ) -> Result<()> {
        for file in changed {
            // 1. Remove old outgoing edges
            self.remove_outgoing_edges(file);

            // 2. Parse file
            let hash = file_content_hash(file)?;
            let parsed = parser.parse_file_with_config(file, parser_config)?;

            // 3. Update symbols, metrics, and hash
            self.file_symbols_mut()
                .insert(file.clone(), parsed.symbols.clone());
            self.file_metrics_mut().insert(
                file.clone(),
                crate::engine::context::FileMetrics {
                    lines: parsed.lines,
                },
            );
            self.function_complexity_mut()
                .insert(file.clone(), parsed.functions);
            self.file_hashes.insert(file.clone(), hash);

            // 4. Update graph and reverse deps
            let from_node = self.graph_mut().add_file(file);
            for import in &parsed.symbols.imports {
                if let Some(resolved) = resolver.resolve(import.source.as_str(), file)? {
                    // Only add to graph if it's a file we know about (runtime code)
                    // Note: In initial scan, we filter by has_runtime_code.
                    // For incremental, we should probably check if it was already in graph or if it has runtime code now.
                    let to_node = self.graph_mut().add_file(&resolved);
                    let edge_data = EdgeData::with_all(
                        import.line,
                        import.range,
                        vec![import.name.to_string()],
                    );
                    self.graph_mut()
                        .add_dependency(from_node, to_node, edge_data);

                    // Update reverse deps
                    self.reverse_deps
                        .entry(resolved)
                        .or_default()
                        .insert(file.clone());
                }
            }
        }
        Ok(())
    }

    pub fn remove_outgoing_edges(&mut self, file: &Path) {
        if let Some(node) = self.graph.get_node(file) {
            let dependencies: Vec<_> = self.graph.dependencies(node).collect();
            for target_node in dependencies {
                if let Some(target_path) = self.graph.get_file_path(target_node) {
                    if let Some(importers) = self.reverse_deps.get_mut(target_path) {
                        importers.remove(file);
                    }
                }
            }
            self.graph_mut().remove_outgoing_edges(node);
        }
    }

    pub fn remove_file(&mut self, path: &Path) {
        self.remove_outgoing_edges(path);
        self.file_symbols_mut().remove(path);
        self.file_metrics_mut().remove(path);
        self.function_complexity_mut().remove(path);
        self.file_hashes.remove(path);
        self.graph_mut().remove_file(path);
        self.reverse_deps.remove(path);
    }
}
