use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::{FileSymbols, MethodAccessibility, SymbolKind};
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
        Self::detect_symbols(ctx.file_symbols.as_ref(), &ctx.script_entry_points, ctx)
    }
}

impl DeadSymbolsDetector {
    pub fn new(_entry_points: HashSet<PathBuf>) -> Self {
        Self
    }

    pub fn detect_symbols(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        entry_points: &HashSet<PathBuf>,
        ctx: &AnalysisContext,
    ) -> Vec<ArchSmell> {
        let all_project_usages = Self::collect_all_usages(file_symbols);
        let symbol_usages = Self::build_symbol_imports_map(file_symbols);

        let mut all_smells = Vec::new();
        all_smells.extend(Self::check_dead_local_symbols(
            file_symbols,
            &all_project_usages,
        ));
        all_smells.extend(Self::check_dead_methods(
            file_symbols,
            &all_project_usages,
            &symbol_usages,
            ctx,
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

    fn is_symbol_imported(
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        file_path: &Path,
        symbol_name: &str,
    ) -> bool {
        // Check for named import
        if let Some(importers) =
            symbol_usages.get(&(file_path.to_path_buf(), symbol_name.to_string()))
        {
            if !importers.is_empty() {
                return true;
            }
        }

        // Check for namespace import (*)
        if let Some(importers) = symbol_usages.get(&(file_path.to_path_buf(), "*".to_string())) {
            if !importers.is_empty() {
                return true;
            }
        }

        false
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

    fn check_dead_methods(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        _all_project_usages: &HashSet<String>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        ctx: &AnalysisContext,
    ) -> Vec<ArchSmell> {
        let ignored_methods = Self::build_ignored_methods_set(ctx);
        let mut smells = Vec::new();

        for (file_path, symbols) in file_symbols {
            for class in &symbols.classes {
                smells.extend(Self::check_class_methods(
                    file_path,
                    class,
                    symbols,
                    file_symbols,
                    symbol_usages,
                    &ignored_methods,
                ));
            }
        }

        smells
    }

    fn build_ignored_methods_set(ctx: &AnalysisContext) -> HashSet<String> {
        let mut ignored_methods: HashSet<String> =
            ["constructor".to_string()].into_iter().collect();

        let presets = crate::framework::presets::get_presets(&ctx.detected_frameworks);
        for preset in presets {
            for method in preset.ignore_methods {
                ignored_methods.insert(method.to_string());
            }
        }

        for method in &ctx.config.thresholds.dead_symbols.ignore_methods {
            ignored_methods.insert(method.clone());
        }

        ignored_methods
    }

    fn check_class_methods(
        file_path: &PathBuf,
        class: &crate::parser::ClassSymbol,
        symbols: &FileSymbols,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        ignored_methods: &HashSet<String>,
    ) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for method in &class.methods {
            if Self::should_skip_method(method, ignored_methods) {
                continue;
            }

            if !Self::is_method_used(
                method,
                file_path,
                class,
                symbols,
                file_symbols,
                symbol_usages,
            ) {
                smells.push(Self::create_dead_method_smell(file_path, class, method));
            }
        }

        smells
    }

    fn should_skip_method(
        method: &crate::parser::MethodSymbol,
        ignored_methods: &HashSet<String>,
    ) -> bool {
        ignored_methods.contains(method.name.as_str())
            || method.has_decorators
            || method.is_accessor
    }

    fn is_method_used(
        method: &crate::parser::MethodSymbol,
        file_path: &PathBuf,
        class: &crate::parser::ClassSymbol,
        symbols: &FileSymbols,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
    ) -> bool {
        if symbols.local_usages.contains(method.name.as_str()) {
            return true;
        }

        if method.accessibility == Some(MethodAccessibility::Private) {
            return false;
        }

        Self::is_method_used_in_importers(method, file_path, class, file_symbols, symbol_usages)
    }

    fn is_method_used_in_importers(
        method: &crate::parser::MethodSymbol,
        file_path: &PathBuf,
        class: &crate::parser::ClassSymbol,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
    ) -> bool {
        let all_importers = Self::collect_class_importers(file_path, class, symbol_usages);

        for importer_path in all_importers {
            if let Some(importer_symbols) = file_symbols.get(&importer_path) {
                if importer_symbols.local_usages.contains(method.name.as_str()) {
                    return true;
                }
            }
        }

        false
    }

    fn collect_class_importers(
        file_path: &PathBuf,
        class: &crate::parser::ClassSymbol,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
    ) -> HashSet<PathBuf> {
        let mut all_importers = HashSet::new();

        if let Some(importers) = symbol_usages.get(&(file_path.clone(), class.name.to_string())) {
            all_importers.extend(importers.iter().cloned());
        }

        if let Some(importers) = symbol_usages.get(&(file_path.clone(), "*".to_string())) {
            all_importers.extend(importers.iter().cloned());
        }

        all_importers
    }

    fn create_dead_method_smell(
        file_path: &PathBuf,
        class: &crate::parser::ClassSymbol,
        method: &crate::parser::MethodSymbol,
    ) -> ArchSmell {
        let mut smell = ArchSmell::new_dead_symbol_with_line(
            file_path.clone(),
            format!("{}.{}", class.name, method.name),
            "Class Method".to_string(),
            method.line,
        );
        if let Some(loc) = smell.locations.first_mut() {
            *loc = loc.clone().with_range(method.range);
        }
        smell
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
        let is_imported = Self::is_symbol_imported(symbol_usages, file_path, export.name.as_str());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::AnalysisContext;
    use crate::parser::ImportParser;
    use std::sync::Arc;

    #[test]
    fn test_detect_unused_private_method() {
        let code = r#"
            class MyService {
                private usedMethod() {
                    return 1;
                }
                private unusedMethod() {
                    return 2;
                }
                public main() {
                    return this.usedMethod();
                }
            }
        "#;
        let path = PathBuf::from("service.ts");
        let parser = ImportParser::new().unwrap();
        let parsed = parser.parse_code(code, &path).unwrap();

        let mut file_symbols = HashMap::new();
        file_symbols.insert(path.clone(), parsed.symbols);

        let mut ctx = AnalysisContext::default_for_test();
        ctx.file_symbols = Arc::new(file_symbols);

        let detector = DeadSymbolsDetector;
        let smells = detector.detect(&ctx);

        let unused_smells: Vec<_> = smells
            .iter()
            .filter(|s| match &s.smell_type {
                crate::detectors::SmellType::DeadSymbol { name, .. } => {
                    name.contains("unusedMethod")
                }
                _ => false,
            })
            .collect();

        assert_eq!(unused_smells.len(), 1);
        if let crate::detectors::SmellType::DeadSymbol { name, .. } = &unused_smells[0].smell_type {
            assert!(name.contains("MyService.unusedMethod"));
        } else {
            panic!("Expected DeadSymbol smell type");
        }
    }

    #[test]
    fn test_detect_unused_public_method_with_name_collision() {
        let service_code = r#"
            export class MetricsService {
                public usedMethod() { return 1; }
                public unusedMethod() { return 2; }
            }
        "#;
        let other_service_code = r#"
            export class OtherService {
                public unusedMethod() { return 3; }
                public main() { return this.unusedMethod(); }
            }
        "#;
        let consumer_code = r#"
            import { MetricsService } from './metrics.service';
            class Consumer {
                constructor(private metrics: MetricsService) {}
                run() { this.metrics.usedMethod(); }
            }
        "#;

        let path1 = PathBuf::from("metrics.service.ts");
        let path2 = PathBuf::from("other.service.ts");
        let path3 = PathBuf::from("consumer.ts");

        let parser = ImportParser::new().unwrap();
        let parsed1 = parser.parse_code(service_code, &path1).unwrap();
        let parsed2 = parser.parse_code(other_service_code, &path2).unwrap();
        let parsed3 = parser.parse_code(consumer_code, &path3).unwrap();

        let mut file_symbols = HashMap::new();
        file_symbols.insert(path1.clone(), parsed1.symbols);
        file_symbols.insert(path2.clone(), parsed2.symbols);
        file_symbols.insert(path3.clone(), parsed3.symbols);

        let mut ctx = AnalysisContext::default_for_test();
        ctx.file_symbols = Arc::new(file_symbols);

        let detector = DeadSymbolsDetector;
        let smells = detector.detect(&ctx);

        // MetricsService.unusedMethod should be dead, even though other.service.ts uses "unusedMethod"
        let metrics_unused = smells.iter().find(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name == "MetricsService.unusedMethod"
            } else {
                false
            }
        });
        assert!(
            metrics_unused.is_some(),
            "MetricsService.unusedMethod should be reported as dead"
        );

        // OtherService.unusedMethod should NOT be dead because it's used locally
        let other_unused = smells.iter().find(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name == "OtherService.unusedMethod"
            } else {
                false
            }
        });
        assert!(
            other_unused.is_none(),
            "OtherService.unusedMethod should NOT be reported as dead"
        );
    }
}
