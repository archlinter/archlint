use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::parser::{FileSymbols, SymbolKind};
use crate::engine::AnalysisContext;
use crate::detectors::{Detector, ArchSmell, DetectorFactory, DetectorInfo};
use crate::config::Config;
use inventory;

pub fn init() {}

pub struct DeadSymbolsDetector;

pub struct DeadSymbolsDetectorFactory;

impl DetectorFactory for DeadSymbolsDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "dead_symbols",
            name: "Dead Symbols Detector",
            description: "Detects unused functions, classes, and variables within files",
            default_enabled: true,
            is_deep: true,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(DeadSymbolsDetector)
    }
}

inventory::submit! {
    &DeadSymbolsDetectorFactory as &dyn DetectorFactory
}

impl Detector for DeadSymbolsDetector {
    fn name(&self) -> &'static str {
        "DeadSymbols"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        Self::detect_symbols(&ctx.file_symbols, &ctx.script_entry_points)
    }
}

impl DeadSymbolsDetector {
    pub fn new(_entry_points: HashSet<PathBuf>) -> Self {
        Self
    }

    pub fn detect_symbols(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        entry_points: &HashSet<PathBuf>,
    ) -> Vec<ArchSmell> {
        let mut all_smells = Vec::new();

        let mut all_project_usages: HashSet<String> = HashSet::new();
        for symbols in file_symbols.values() {
            for usage in &symbols.local_usages {
                all_project_usages.insert(usage.to_string());
            }
        }

        let mut symbol_usages: HashMap<(PathBuf, String), HashSet<PathBuf>> = HashMap::new();

        for (importer_path, symbols) in file_symbols {
            for import in &symbols.imports {
                let source_path = PathBuf::from(import.source.as_str());
                symbol_usages
                    .entry((source_path, import.name.to_string()))
                    .or_default()
                    .insert(importer_path.clone());
            }
        }

        for (file_path, symbols) in file_symbols {
            for local_def in &symbols.local_definitions {
                let is_exported = symbols.exports.iter().any(|e| e.name.as_str() == local_def.as_str());
                let is_used_anywhere = all_project_usages.contains(local_def.as_str());

                if !is_exported && !is_used_anywhere {
                    all_smells.push(ArchSmell::new_dead_symbol(
                        file_path.clone(),
                        local_def.to_string(),
                        "Local Variable/Function".to_string(),
                    ));
                }
            }

            if entry_points.contains(file_path) {
                continue;
            }

            for export in &symbols.exports {
                if export.is_reexport {
                    continue;
                }

                if export.name == "default" || export.name == "*" {
                    continue;
                }

                let usages = symbol_usages.get(&(file_path.clone(), export.name.to_string()));
                let is_imported = usages.is_some() && !usages.unwrap().is_empty();
                let is_used_by_name = all_project_usages.contains(export.name.as_str());

                if !is_imported && !is_used_by_name {
                    let kind_str = match export.kind {
                        SymbolKind::Function => "Function",
                        SymbolKind::Class => "Class",
                        SymbolKind::Variable => "Variable",
                        SymbolKind::Type => "Type",
                        SymbolKind::Interface => "Interface",
                        _ => "Symbol",
                    };

                    let mut smell = ArchSmell::new_dead_symbol_with_line(
                        file_path.clone(),
                        export.name.to_string(),
                        kind_str.to_string(),
                        export.line,
                    );
                    if let Some(loc) = smell.locations.first_mut() {
                        *loc = loc.clone().with_range(export.range);
                    }
                    all_smells.push(smell);
                }
            }
        }

        all_smells
    }
}
