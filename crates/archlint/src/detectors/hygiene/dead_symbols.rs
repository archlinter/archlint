use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;
use crate::parser::{FileSymbols, MethodAccessibility, SymbolKind};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    id = "dead_symbols",
    name = "Dead Symbols Detector",
    description = "Detects unused functions, classes, and variables within files",
    category = DetectorCategory::Global,
    is_deep = true
)]
pub struct DeadSymbolsDetector;

#[derive(Default)]
struct InheritanceContext {
    parents: HashMap<(PathBuf, String), (PathBuf, String)>,
    children: HashMap<(PathBuf, String), Vec<(PathBuf, String)>>,
    reexports: HashMap<PathBuf, HashSet<PathBuf>>,
}

impl DeadSymbolsDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    pub fn detect_symbols(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        entry_points: &HashSet<PathBuf>,
        ctx: &AnalysisContext,
    ) -> Vec<ArchSmell> {
        let all_project_usages = Self::collect_all_usages(file_symbols);
        let symbol_usages = Self::build_symbol_imports_map(file_symbols);
        let inheritance_ctx = Self::build_inheritance_context(file_symbols);

        let mut all_smells = Vec::new();
        all_smells.extend(Self::check_dead_local_symbols(
            file_symbols,
            &all_project_usages,
        ));
        all_smells.extend(Self::check_dead_methods(
            file_symbols,
            &symbol_usages,
            &inheritance_ctx,
            entry_points,
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

    fn build_inheritance_context(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
    ) -> InheritanceContext {
        let mut ctx = InheritanceContext::default();
        Self::collect_reexports(file_symbols, &mut ctx);
        Self::collect_inheritance(file_symbols, &mut ctx);
        ctx
    }

    fn collect_reexports(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        ctx: &mut InheritanceContext,
    ) {
        for (path, symbols) in file_symbols {
            for export in &symbols.exports {
                if export.is_reexport && export.name == "*" {
                    if let Some(ref source) = export.source {
                        ctx.reexports
                            .entry(PathBuf::from(source.as_str()))
                            .or_default()
                            .insert(path.clone());
                    }
                }
            }
        }
    }

    fn collect_inheritance(
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        ctx: &mut InheritanceContext,
    ) {
        for (path, symbols) in file_symbols {
            for class in &symbols.classes {
                if let Some(ref super_name) = class.super_class {
                    if let Some((parent_path, parent_name)) =
                        Self::resolve_parent_class(symbols, super_name)
                    {
                        let child_id = (path.clone(), class.name.to_string());
                        let parent_id = (parent_path, parent_name);

                        ctx.parents.insert(child_id.clone(), parent_id.clone());
                        ctx.children.entry(parent_id).or_default().push(child_id);
                    }
                }
            }
        }
    }

    fn resolve_parent_class(symbols: &FileSymbols, super_name: &str) -> Option<(PathBuf, String)> {
        if let Some(import) = symbols
            .imports
            .iter()
            .find(|i| i.alias.as_ref().map_or(i.name.as_str(), |a| a.as_str()) == super_name)
        {
            return Some((
                PathBuf::from(import.source.as_str()),
                import.name.to_string(),
            ));
        }

        if let Some(dot_pos) = super_name.find('.') {
            let ns = &super_name[..dot_pos];
            let name = &super_name[dot_pos + 1..];

            if let Some(import) = symbols
                .imports
                .iter()
                .find(|i| i.alias.as_ref().map_or(i.name.as_str(), |a| a.as_str()) == ns)
            {
                if import.name == "*" {
                    return Some((PathBuf::from(import.source.as_str()), name.to_string()));
                }
            }
        }

        None
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
        if let Some(importers) =
            symbol_usages.get(&(file_path.to_path_buf(), symbol_name.to_string()))
        {
            if !importers.is_empty() {
                return true;
            }
        }

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
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        inheritance_ctx: &InheritanceContext,
        entry_points: &HashSet<PathBuf>,
        ctx: &AnalysisContext,
    ) -> Vec<ArchSmell> {
        let ignored_methods = Self::build_ignored_methods_set(ctx);
        let contract_methods = Self::build_contract_methods_map(ctx);
        let mut smells = Vec::new();

        for (file_path, symbols) in file_symbols {
            for class in &symbols.classes {
                smells.extend(Self::check_class_methods(
                    file_path,
                    class,
                    symbols,
                    file_symbols,
                    symbol_usages,
                    inheritance_ctx,
                    entry_points,
                    &ignored_methods,
                    &contract_methods,
                ));
            }
        }

        smells
    }

    fn build_contract_methods_map(ctx: &AnalysisContext) -> HashMap<String, Vec<String>> {
        let rule = ctx.resolve_rule("dead_symbols", None);
        let options = rule.get_option::<HashMap<String, Vec<String>>>("contract_methods");
        options.unwrap_or_default()
    }

    fn build_ignored_methods_set(ctx: &AnalysisContext) -> HashSet<String> {
        let mut ignored_methods: HashSet<String> =
            ["constructor".to_string()].into_iter().collect();

        let rule = ctx.resolve_rule("dead_symbols", None);
        if let Some(methods) = rule.get_option::<Vec<String>>("ignore_methods") {
            for method in methods {
                ignored_methods.insert(method);
            }
        }

        ignored_methods
    }

    #[allow(clippy::too_many_arguments)]
    fn check_class_methods(
        file_path: &Path,
        class: &crate::parser::ClassSymbol,
        symbols: &FileSymbols,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        inheritance_ctx: &InheritanceContext,
        entry_points: &HashSet<PathBuf>,
        ignored_methods: &HashSet<String>,
        contract_methods: &HashMap<String, Vec<String>>,
    ) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for method in &class.methods {
            if Self::should_skip_method(method, class, ignored_methods, contract_methods) {
                continue;
            }

            if !Self::is_method_used(
                method,
                file_path,
                class,
                symbols,
                file_symbols,
                symbol_usages,
                inheritance_ctx,
                entry_points,
            ) {
                smells.push(Self::create_dead_method_smell(file_path, class, method));
            }
        }

        smells
    }

    fn should_skip_method(
        method: &crate::parser::MethodSymbol,
        class: &crate::parser::ClassSymbol,
        ignored_methods: &HashSet<String>,
        contract_methods: &HashMap<String, Vec<String>>,
    ) -> bool {
        if ignored_methods.contains(method.name.as_str())
            || method.has_decorators
            || method.is_accessor
        {
            return true;
        }

        for interface in &class.implements {
            let interface_name = if let Some(dot_pos) = interface.rfind('.') {
                &interface[dot_pos + 1..]
            } else {
                interface.as_str()
            };

            if let Some(methods) = contract_methods.get(interface_name) {
                if methods.iter().any(|m| m == method.name.as_str()) {
                    return true;
                }
            }
        }

        false
    }

    #[allow(clippy::too_many_arguments)]
    fn is_method_used(
        method: &crate::parser::MethodSymbol,
        file_path: &Path,
        class: &crate::parser::ClassSymbol,
        symbols: &FileSymbols,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        inheritance_ctx: &InheritanceContext,
        entry_points: &HashSet<PathBuf>,
    ) -> bool {
        if symbols.local_usages.contains(method.name.as_str()) {
            return true;
        }

        // If class itself is defined in entry point, all non-private methods are considered used
        if method.accessibility != Some(MethodAccessibility::Private)
            && entry_points.contains(file_path)
        {
            return true;
        }

        let mut current_class_id = (file_path.to_path_buf(), class.name.to_string());
        let mut visited_parents = HashSet::new();
        visited_parents.insert(current_class_id.clone());

        while let Some(parent_id) = inheritance_ctx.parents.get(&current_class_id) {
            if !visited_parents.insert(parent_id.clone()) {
                break;
            }
            if let Some(parent_symbols) = file_symbols.get(&parent_id.0) {
                if parent_symbols.local_usages.contains(method.name.as_str()) {
                    return true;
                }
            }
            current_class_id = parent_id.clone();
        }

        if method.accessibility != Some(MethodAccessibility::Private)
            && Self::is_method_used_in_importers(
                method,
                file_path,
                class,
                file_symbols,
                symbol_usages,
                inheritance_ctx,
                entry_points,
            )
        {
            return true;
        }

        false
    }

    fn is_method_used_in_importers(
        method: &crate::parser::MethodSymbol,
        file_path: &Path,
        class: &crate::parser::ClassSymbol,
        file_symbols: &HashMap<PathBuf, FileSymbols>,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        inheritance_ctx: &InheritanceContext,
        entry_points: &HashSet<PathBuf>,
    ) -> bool {
        let all_importers =
            Self::collect_class_importers(file_path, class, symbol_usages, inheritance_ctx);

        for importer_path in all_importers {
            // If importer is an entry point, we consider non-private methods as used (part of public API)
            if entry_points.contains(&importer_path) {
                return true;
            }

            if let Some(importer_symbols) = file_symbols.get(&importer_path) {
                if importer_symbols.local_usages.contains(method.name.as_str()) {
                    return true;
                }
            }
        }

        false
    }

    fn collect_all_reexporters_static(
        file_path: &Path,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
        visited: &mut HashSet<PathBuf>,
    ) {
        if !visited.insert(file_path.to_path_buf()) {
            return;
        }
        if let Some(reexporters) = reexport_map.get(file_path) {
            for reexporter in reexporters {
                Self::collect_all_reexporters_static(reexporter, reexport_map, visited);
            }
        }
    }

    fn collect_all_descendants_static(
        class_id: &(PathBuf, String),
        children_map: &HashMap<(PathBuf, String), Vec<(PathBuf, String)>>,
        visited: &mut HashSet<(PathBuf, String)>,
    ) {
        if !visited.insert(class_id.clone()) {
            return;
        }
        if let Some(children) = children_map.get(class_id) {
            for child in children {
                Self::collect_all_descendants_static(child, children_map, visited);
            }
        }
    }

    fn collect_class_importers(
        file_path: &Path,
        class: &crate::parser::ClassSymbol,
        symbol_usages: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        inheritance_ctx: &InheritanceContext,
    ) -> HashSet<PathBuf> {
        let mut all_importers = HashSet::new();

        let mut all_classes = HashSet::new();
        let class_id = (file_path.to_path_buf(), class.name.to_string());
        Self::collect_all_descendants_static(
            &class_id,
            &inheritance_ctx.children,
            &mut all_classes,
        );

        for (c_path, c_name) in all_classes {
            let mut all_source_files = HashSet::new();
            Self::collect_all_reexporters_static(
                &c_path,
                &inheritance_ctx.reexports,
                &mut all_source_files,
            );

            for source_path in all_source_files {
                if let Some(importers) = symbol_usages.get(&(source_path.clone(), c_name.clone())) {
                    all_importers.extend(importers.iter().cloned());
                }
                if let Some(importers) = symbol_usages.get(&(source_path, "*".to_string())) {
                    all_importers.extend(importers.iter().cloned());
                }
            }
        }

        all_importers
    }

    fn create_dead_method_smell(
        file_path: &Path,
        class: &crate::parser::ClassSymbol,
        method: &crate::parser::MethodSymbol,
    ) -> ArchSmell {
        let mut smell = ArchSmell::new_dead_symbol_with_line(
            file_path.to_path_buf(),
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

impl Detector for DeadSymbolsDetector {
    crate::impl_detector_report!(
        name: "DeadSymbols",
        explain: smell => {
            let kind = if let crate::detectors::SmellType::DeadSymbol { kind, .. } = &smell.smell_type {
                kind.as_str()
            } else {
                "symbol"
            };
            crate::detectors::Explanation {
                problem: format!("Unused {} detected", kind),
                reason: "The symbol is defined but not imported by any other file or used locally.".into(),
                risks: crate::strings![
                    "Increases cognitive load when reading the file",
                    "Dead code can hide bugs and complicate refactoring",
                    "May lead to confusion about the intended API of the module"
                ],
                recommendations: crate::strings![
                    "Remove the unused symbol if it is truly no longer needed",
                    "Check if it should be an internal helper or if it was meant to be exported and used"
                ]
            }
        },
        table: {
            title: "Dead Symbols",
            columns: ["Location", "Symbol", "Kind", "pts"],
            row: DeadSymbol { name, kind } (smell, location, pts) => [location, name, kind, pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        if ctx.get_rule("dead_symbols").is_none() {
            return Vec::new();
        }

        let smells = Self::detect_symbols(ctx.file_symbols.as_ref(), &ctx.script_entry_points, ctx);

        smells
            .into_iter()
            .filter_map(|mut smell| {
                if let Some(path) = smell.files.first() {
                    let file_rule = match ctx.get_rule_for_file("dead_symbols", path) {
                        Some(r) => r,
                        None => return None,
                    };
                    smell.severity = file_rule.severity;
                }
                Some(smell)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::AnalysisContext;
    use crate::parser::ImportParser;
    use crate::parser::{
        ClassSymbol, ExportedSymbol, FileSymbols, ImportedSymbol, MethodAccessibility,
        MethodSymbol, SymbolKind,
    };
    use crate::CodeRange;
    use compact_str::CompactString;
    use rustc_hash::FxHashSet;
    use smallvec::smallvec;
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;
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

    #[test]
    fn test_namespace_inheritance() {
        let mut file_symbols = HashMap::new();

        let base_path = PathBuf::from("base.ts");
        let base_symbols = FileSymbols {
            exports: vec![ExportedSymbol {
                name: CompactString::new("Base"),
                kind: SymbolKind::Class,
                is_reexport: false,
                source: None,
                line: 1,
                column: 0,
                range: CodeRange::default(),
                used_symbols: FxHashSet::default(),
                is_mutable: false,
                is_default: false,
            }],
            classes: vec![ClassSymbol {
                name: CompactString::new("Base"),
                super_class: None,
                implements: vec![],
                fields: smallvec![],
                methods: smallvec![MethodSymbol::new(
                    CompactString::new("method"),
                    2,
                    4,
                    CodeRange::default(),
                    false,
                    false,
                    Some(MethodAccessibility::Public),
                    false,
                )],
                is_abstract: false,
            }],
            imports: vec![],
            local_definitions: vec![],
            local_usages: FxHashSet::default(),
            has_runtime_code: true,
            env_vars: FxHashSet::default(),
        };
        file_symbols.insert(base_path.clone(), base_symbols);

        let child_path = PathBuf::from("child.ts");
        let child_symbols = FileSymbols {
            exports: vec![ExportedSymbol {
                name: CompactString::new("Child"),
                kind: SymbolKind::Class,
                is_reexport: false,
                source: None,
                line: 2,
                column: 0,
                range: CodeRange::default(),
                used_symbols: FxHashSet::default(),
                is_mutable: false,
                is_default: false,
            }],
            classes: vec![ClassSymbol {
                name: CompactString::new("Child"),
                super_class: Some(CompactString::new("NS.Base")),
                implements: vec![],
                fields: smallvec![],
                methods: smallvec![],
                is_abstract: false,
            }],
            imports: vec![ImportedSymbol {
                name: CompactString::new("*"),
                alias: Some(CompactString::new("NS")),
                source: CompactString::new("base.ts"),
                line: 1,
                column: 0,
                range: CodeRange::default(),
                is_type_only: false,
                is_reexport: false,
                is_dynamic: false,
            }],
            local_definitions: vec![],
            local_usages: FxHashSet::default(),
            has_runtime_code: true,
            env_vars: FxHashSet::default(),
        };
        file_symbols.insert(child_path.clone(), child_symbols);

        let main_path = PathBuf::from("main.ts");
        let mut local_usages = FxHashSet::default();
        local_usages.insert(CompactString::new("Child"));
        local_usages.insert(CompactString::new("method"));
        let main_symbols = FileSymbols {
            exports: vec![],
            classes: vec![],
            imports: vec![ImportedSymbol {
                name: CompactString::new("Child"),
                alias: None,
                source: CompactString::new("child.ts"),
                line: 1,
                column: 0,
                range: CodeRange::default(),
                is_type_only: false,
                is_reexport: false,
                is_dynamic: false,
            }],
            local_definitions: vec![],
            local_usages,
            has_runtime_code: true,
            env_vars: FxHashSet::default(),
        };
        file_symbols.insert(main_path.clone(), main_symbols);

        let mut ctx = AnalysisContext::default_for_test();
        ctx.file_symbols = Arc::new(file_symbols.clone());
        ctx.script_entry_points = HashSet::from_iter(vec![main_path.clone()]);

        let _detector = DeadSymbolsDetector;
        let smells =
            DeadSymbolsDetector::detect_symbols(&file_symbols, &ctx.script_entry_points, &ctx);

        let dead_method = smells.iter().find(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name.contains("method")
            } else {
                false
            }
        });
        assert!(dead_method.is_none(), "Method should be alive");
    }

    #[test]
    fn test_polymorphism_and_barrels() {
        let base_code = r#"
            export abstract class Base {
                protected abstract usedInBase(): void;
                public run() { this.usedInBase(); }
            }
        "#;
        let child_code = r#"
            import { Base } from './base';
            export class Child extends Base {
                protected usedInBase(): void { console.log("used"); }
                public unusedMethod(): void { console.log("unused"); }
            }
        "#;
        let index_code = r#"
            export * from './child';
        "#;
        let consumer_code = r#"
            import { Child } from './index';
            const c = new Child();
            c.run();
        "#;

        let path_base = PathBuf::from("base.ts");
        let path_child = PathBuf::from("child.ts");
        let path_index = PathBuf::from("index.ts");
        let path_consumer = PathBuf::from("consumer.ts");

        let parser = ImportParser::new().unwrap();
        let parsed_base = parser.parse_code(base_code, &path_base).unwrap();
        let parsed_child = parser.parse_code(child_code, &path_child).unwrap();
        let parsed_index = parser.parse_code(index_code, &path_index).unwrap();
        let parsed_consumer = parser.parse_code(consumer_code, &path_consumer).unwrap();

        let mut file_symbols = HashMap::new();
        file_symbols.insert(path_base.clone(), parsed_base.symbols);
        file_symbols.insert(path_child.clone(), parsed_child.symbols);
        file_symbols.insert(path_index.clone(), parsed_index.symbols);
        file_symbols.insert(path_consumer.clone(), parsed_consumer.symbols);

        let mut ctx = AnalysisContext::default_for_test();
        let mut resolved_symbols = file_symbols.clone();
        resolved_symbols.get_mut(&path_child).unwrap().imports[0].source = "base.ts".into();
        resolved_symbols.get_mut(&path_index).unwrap().exports[0].source = Some("child.ts".into());
        resolved_symbols.get_mut(&path_consumer).unwrap().imports[0].source = "index.ts".into();

        ctx.file_symbols = Arc::new(resolved_symbols);

        let detector = DeadSymbolsDetector;
        let smells = detector.detect(&ctx);

        let base_run_dead = smells.iter().any(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name == "Base.run"
            } else {
                false
            }
        });
        assert!(!base_run_dead, "Base.run should be alive");

        let child_used_dead = smells.iter().any(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name == "Child.usedInBase"
            } else {
                false
            }
        });
        assert!(!child_used_dead, "Child.usedInBase should be alive");

        let child_unused_dead = smells.iter().any(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name == "Child.unusedMethod"
            } else {
                false
            }
        });
        assert!(child_unused_dead, "Child.unusedMethod should be dead");
    }

    #[test]
    fn test_contract_methods_skip() {
        let code = r#"
            export class ThrottlerConfig implements ThrottlerOptionsFactory {
                createThrottlerOptions() { return {}; }
                unusedMethod() { return 1; }
            }
        "#;
        let path = PathBuf::from("config.ts");
        let parser = ImportParser::new().unwrap();
        let parsed = parser.parse_code(code, &path).unwrap();

        let mut file_symbols = HashMap::new();
        file_symbols.insert(path.clone(), parsed.symbols);

        let mut ctx = AnalysisContext::default_for_test();
        ctx.file_symbols = Arc::new(file_symbols);

        // Setup config with contract_methods
        let mut options = serde_yaml::Mapping::new();
        let mut contract_methods = serde_yaml::Mapping::new();
        contract_methods.insert(
            serde_yaml::Value::String("ThrottlerOptionsFactory".to_string()),
            serde_yaml::Value::Sequence(vec![serde_yaml::Value::String(
                "createThrottlerOptions".to_string(),
            )]),
        );
        options.insert(
            serde_yaml::Value::String("contract_methods".to_string()),
            serde_yaml::Value::Mapping(contract_methods),
        );

        ctx.config.rules.insert(
            "dead_symbols".to_string(),
            crate::config::RuleConfig::Full(crate::config::RuleFullConfig {
                enabled: Some(true),
                severity: None,
                exclude: vec![],
                options: serde_yaml::Value::Mapping(options),
            }),
        );

        let detector = DeadSymbolsDetector;
        let smells = detector.detect(&ctx);

        let contract_smell = smells.iter().find(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name.contains("createThrottlerOptions")
            } else {
                false
            }
        });
        assert!(contract_smell.is_none(), "Contract method should be alive");

        let unused_smell = smells.iter().find(|s| {
            if let crate::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type {
                name.contains("unusedMethod")
            } else {
                false
            }
        });
        assert!(
            unused_smell.is_some(),
            "Normal unused method should be dead"
        );
    }

    #[test]
    fn test_config_merging_contract_methods() {
        use crate::config::{Config, RuleConfig, RuleFullConfig};
        use serde_yaml::{Mapping, Value};

        let mut user_config = Config::default();
        let mut user_options = Mapping::new();
        let mut user_contracts = Mapping::new();
        user_contracts.insert(
            Value::String("ThrottlerOptionsFactory".to_string()),
            Value::Sequence(vec![Value::String("createThrottlerOptions".to_string())]),
        );
        user_options.insert(
            Value::String("contract_methods".to_string()),
            Value::Mapping(user_contracts),
        );
        user_config.rules.insert(
            "dead_symbols".to_string(),
            RuleConfig::Full(RuleFullConfig {
                options: Value::Mapping(user_options),
                ..Default::default()
            }),
        );

        let mut preset_options = Mapping::new();
        let mut preset_contracts = Mapping::new();
        preset_contracts.insert(
            Value::String("OnModuleInit".to_string()),
            Value::Sequence(vec![Value::String("onModuleInit".to_string())]),
        );
        preset_options.insert(
            Value::String("contract_methods".to_string()),
            Value::Mapping(preset_contracts),
        );
        let preset_rules = vec![(
            "dead_symbols".to_string(),
            RuleConfig::Full(RuleFullConfig {
                options: Value::Mapping(preset_options),
                ..Default::default()
            }),
        )]
        .into_iter()
        .collect();

        let preset = crate::framework::presets::FrameworkPreset {
            name: "nestjs".to_string(),
            rules: preset_rules,
            entry_points: vec![],
            overrides: vec![],
        };

        user_config.merge_preset(&preset);

        // Check if merged correctly
        let rule = user_config.rules.get("dead_symbols").unwrap();
        if let RuleConfig::Full(full) = rule {
            let contracts: HashMap<String, Vec<String>> = serde_yaml::from_value(
                full.options
                    .as_mapping()
                    .unwrap()
                    .get(Value::String("contract_methods".to_string()))
                    .unwrap()
                    .clone(),
            )
            .unwrap();

            assert!(contracts.contains_key("ThrottlerOptionsFactory"));
            assert!(contracts.contains_key("OnModuleInit"));
            assert_eq!(
                contracts.get("ThrottlerOptionsFactory").unwrap(),
                &vec!["createThrottlerOptions".to_string()]
            );
        } else {
            panic!("Expected Full config");
        }
    }
}
