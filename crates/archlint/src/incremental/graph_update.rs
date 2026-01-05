use super::state::IncrementalState;
use crate::cache::hash::content_hash;
use crate::graph::EdgeData;
use crate::parser::{FileSymbols, ImportParser, ParsedFile, ParserConfig};
use crate::resolver::PathResolver;
use crate::Result;
use std::collections::HashMap;
use std::fs;
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
        self.update_files_with_overlays(
            changed,
            &HashMap::new(),
            parser,
            parser_config,
            resolver,
            false,
        )
    }

    /// Update files using overlay content (for unsaved IDE buffers)
    pub fn update_files_with_overlays(
        &mut self,
        changed: &[PathBuf],
        overlays: &HashMap<PathBuf, String>,
        parser: &ImportParser,
        parser_config: &ParserConfig,
        resolver: &PathResolver,
        skip_hash_update: bool,
    ) -> Result<()> {
        for file in changed {
            self.remove_outgoing_edges(file);

            let (content, hash) = Self::get_file_content_and_hash(file, overlays)?;
            let mut parsed = parser.parse_code_with_config(&content, file, parser_config)?;

            Self::resolve_symbols_paths(&mut parsed.symbols, file, resolver);

            self.update_file_data(file, &parsed, hash, skip_hash_update);
            self.update_graph_dependencies(file, &parsed.symbols);
        }
        Ok(())
    }

    fn get_file_content_and_hash(
        file: &PathBuf,
        overlays: &HashMap<PathBuf, String>,
    ) -> Result<(String, String)> {
        if let Some(overlay_content) = overlays.get(file) {
            let hash = content_hash(overlay_content);
            Ok((overlay_content.clone(), hash))
        } else {
            let content = fs::read_to_string(file)?;
            let hash = content_hash(&content);
            Ok((content, hash))
        }
    }

    fn resolve_symbols_paths(symbols: &mut FileSymbols, file: &PathBuf, resolver: &PathResolver) {
        for import in &mut symbols.imports {
            if let Some(resolved) = resolver
                .resolve(import.source.as_str(), file)
                .ok()
                .flatten()
            {
                import.source = resolved.to_string_lossy().to_string().into();
            }
        }

        for export in &mut symbols.exports {
            if let Some(ref source) = export.source {
                if let Some(resolved) = resolver.resolve(source.as_str(), file).ok().flatten() {
                    export.source = Some(resolved.to_string_lossy().to_string().into());
                }
            }
        }
    }

    fn update_file_data(
        &mut self,
        file: &PathBuf,
        parsed: &ParsedFile,
        hash: String,
        skip_hash_update: bool,
    ) {
        self.file_symbols_mut()
            .insert(file.clone(), parsed.symbols.clone());
        self.file_metrics_mut().insert(
            file.clone(),
            crate::engine::context::FileMetrics {
                lines: parsed.lines,
            },
        );
        self.function_complexity_mut()
            .insert(file.clone(), parsed.functions.clone());

        if !skip_hash_update {
            self.file_hashes.insert(file.clone(), hash);
        }
    }

    fn update_graph_dependencies(&mut self, file: &PathBuf, symbols: &FileSymbols) {
        let from_node = self.graph_mut().add_file(file);

        for import in &symbols.imports {
            let source = import.source.as_str();
            let resolved = PathBuf::from(source);

            if resolved.is_absolute() {
                let to_node = self.graph_mut().add_file(&resolved);
                let edge_data =
                    EdgeData::with_all(import.line, import.range, vec![import.name.to_string()]);
                self.graph_mut()
                    .add_dependency(from_node, to_node, edge_data);

                self.reverse_deps
                    .entry(resolved)
                    .or_default()
                    .insert(file.clone());
            }
        }
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
