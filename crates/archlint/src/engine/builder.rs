use crate::config::Config;
use crate::engine::progress::{
    create_progress_bar, default_progress_chars, default_spinner_template,
};
use crate::graph::{DependencyGraph, EdgeData};
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;
use crate::parser::FileSymbols;
use crate::resolver::PathResolver;
use crate::Result;
#[cfg(feature = "cli")]
use console::style;
use log::info;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct EngineBuilder<'a> {
    project_root: &'a Path,
    config: &'a Config,
}

impl<'a> EngineBuilder<'a> {
    pub fn new(project_root: &'a Path, config: &'a Config) -> Self {
        Self {
            project_root,
            config,
        }
    }

    pub fn build_graph(
        &self,
        runtime_files: &HashSet<PathBuf>,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        use_progress: bool,
    ) -> Result<DependencyGraph> {
        info!(
            "{}  Building dependency graph...",
            style("🕸️").cyan().bold()
        );

        let mut graph = DependencyGraph::new();
        for file in runtime_files {
            graph.add_file(file);
        }

        let pb = use_progress.then(|| {
            create_progress_bar(
                runtime_files.len(),
                default_spinner_template(),
                default_progress_chars(),
            )
        });

        let resolver = PathResolver::new(self.project_root, self.config);
        let mut resolved_count = 0;

        for file in runtime_files {
            if let Some(ref pb) = pb {
                if let Some(name) = file.file_name() {
                    pb.set_message(name.to_string_lossy().to_string());
                }
            }

            resolved_count += self.process_file_dependencies(
                file,
                &resolver,
                &mut graph,
                runtime_files,
                file_symbols,
            )?;

            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        info!(
            "   {} Nodes: {}, Edges: {}, Resolved: {}",
            style("↳").dim(),
            style(graph.node_count()).yellow(),
            style(graph.edge_count()).yellow(),
            style(resolved_count).dim()
        );
        Ok(graph)
    }

    fn process_file_dependencies(
        &self,
        file: &Path,
        resolver: &PathResolver,
        graph: &mut DependencyGraph,
        runtime_files: &HashSet<PathBuf>,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
    ) -> Result<usize> {
        let from_node = graph.get_node(file).unwrap();
        let symbols = file_symbols.get(file).unwrap();
        let mut count = 0;

        for import in &symbols.imports {
            if let Some(resolved) = resolver.resolve(import.source.as_str(), file)? {
                if runtime_files.contains(&resolved) {
                    let to_node = graph.add_file(&resolved);
                    let edge_data = EdgeData::with_all(
                        import.line,
                        import.range,
                        vec![import.name.to_string()],
                    );
                    graph.add_dependency(from_node, to_node, edge_data);
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    pub fn resolve_symbols(
        &self,
        file_symbols: HashMap<PathBuf, FileSymbols>,
        use_progress: bool,
    ) -> HashMap<PathBuf, FileSymbols> {
        info!("{} Resolving symbols...", style("🔗").cyan().bold());
        let resolver = PathResolver::new(self.project_root, self.config);

        let pb = if use_progress {
            Some(create_progress_bar(
                file_symbols.len(),
                default_spinner_template(),
                default_progress_chars(),
            ))
        } else {
            None
        };

        let resolved_file_symbols: HashMap<_, _> = file_symbols
            .into_par_iter()
            .map(|(file, symbols)| {
                let mut resolved_symbols = symbols.clone();
                for import in &mut resolved_symbols.imports {
                    if let Some(resolved) = resolver
                        .resolve(import.source.as_str(), &file)
                        .ok()
                        .flatten()
                    {
                        import.source = resolved.to_string_lossy().to_string().into();
                    }
                }
                for export in &mut resolved_symbols.exports {
                    if let Some(ref source) = export.source {
                        if let Some(resolved) =
                            resolver.resolve(source.as_str(), &file).ok().flatten()
                        {
                            export.source = Some(resolved.to_string_lossy().to_string().into());
                        }
                    }
                }
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
                (file, resolved_symbols)
            })
            .collect();

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        resolved_file_symbols
    }
}
