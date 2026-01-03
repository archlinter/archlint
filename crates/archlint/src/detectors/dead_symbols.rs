use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::{FileSymbols, SymbolKind};
use inventory;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

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
            category: DetectorCategory::Global,
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
        Self::detect_symbols(ctx.file_symbols.as_ref(), &ctx.script_entry_points)
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
        let all_project_usages = Self::collect_all_usages(file_symbols);
        let symbol_usages = Self::build_symbol_imports_map(file_symbols);

        let mut all_smells = Vec::new();
        all_smells.extend(Self::check_dead_local_symbols(
            file_symbols,
            &all_project_usages,
        ));
        all_smells.extend(Self::check_dead_exports(
            file_symbols,
            entry_points,
            &symbol_usages,
            &all_project_usages,
        ));

        all_smells
    }

    fn collect_all_usages(file_symbols: &HashMap<PathBuf, FileSymbols>) -> HashSet<String> {
        file_symbols
            .values()
            .flat_map(|symbols| &symbols.local_usages)
            .map(|usage| usage.to_string())
            .collect()
    }

    fn build_symbol_imports_map(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
    ) -> HashMap<(PathBuf, String), HashSet<PathBuf>> {
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

        symbol_usages
    }

    fn check_dead_local_symbols(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        all_project_usages: &HashSet<String>,
    ) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (file_path, symbols) in file_symbols {
            for local_def in &symbols.local_definitions {
                let is_exported = symbols
                    .exports
                    .iter()
                    .any(|e| e.name.as_str() == local_def.as_str());
                let is_used_anywhere = all_project_usages.contains(local_def.as_str());

                if !is_exported && !is_used_anywhere {
                    smells.push(ArchSmell::new_dead_symbol(
                        file_path.clone(),
                        local_def.to_string(),
                        "Local Variable/Function".to_string(),
                    ));
                }
            }
        }

        smells
    }

    fn check_dead_exports(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        entry_points: &HashSet<PathBuf>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        all_project_usages: &HashSet<String>,
    ) -> Vec<ArchSmell> {
        file_symbols
            .iter()
            .filter(|(file_path, _)| !entry_points.contains(*file_path))
            .flat_map(|(file_path, symbols)| {
                Self::check_file_exports(
                    file_path.as_path(),
                    symbols,
                    symbol_usages,
                    all_project_usages,
                )
            })
            .collect()
    }

    fn check_file_exports(
        file_path: &Path,
        symbols: &FileSymbols,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        all_project_usages: &HashSet<String>,
    ) -> Vec<ArchSmell> {
        symbols
            .exports
            .iter()
            .filter(|export| !export.is_reexport && export.name != "default" && export.name != "*")
            .filter_map(|export| {
                Self::check_export_usage(file_path, export, symbol_usages, all_project_usages)
            })
            .collect()
    }

    fn check_export_usage(
        file_path: &Path,
        export: &crate::parser::ExportedSymbol,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        all_project_usages: &HashSet<String>,
    ) -> Option<ArchSmell> {
        let usages = symbol_usages.get(&(file_path.to_path_buf(), export.name.to_string()));
        let is_imported = usages.is_some() && !usages.unwrap().is_empty();
        let is_used_by_name = all_project_usages.contains(export.name.as_str());

        if is_imported || is_used_by_name {
            return None;
        }

        let kind_str = Self::format_symbol_kind(&export.kind);
        let mut smell = ArchSmell::new_dead_symbol_with_line(
            file_path.to_path_buf(),
            export.name.to_string(),
            kind_str,
            export.line,
        );
        if let Some(loc) = smell.locations.first_mut() {
            *loc = loc.clone().with_range(export.range);
        }
        Some(smell)
    }

    fn format_symbol_kind(kind: &SymbolKind) -> String {
        match kind {
            SymbolKind::Function => "Function",
            SymbolKind::Class => "Class",
            SymbolKind::Variable => "Variable",
            SymbolKind::Type => "Type",
            SymbolKind::Interface => "Interface",
            _ => "Symbol",
        }
        .to_string()
    }
}
