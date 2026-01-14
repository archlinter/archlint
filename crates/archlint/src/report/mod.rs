use serde::{Deserialize, Serialize};
use strum::Display;
pub mod json;
pub mod markdown;
pub mod mermaid;
pub mod sarif;

use crate::config::SeverityConfig;
use crate::detectors::{ArchSmell, CodeRange, LocationDetail, Severity, SmellType};
use crate::engine::context::FileMetrics;
use crate::explain::{ExplainEngine, Explanation};
use crate::framework::presets::FrameworkPreset;
use crate::graph::DependencyGraph;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::modifiers::UTF8_ROUND_CORNERS;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::presets::UTF8_FULL;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;
use crate::parser::{FileIgnoredLines, FileSymbols, FunctionComplexity};
use crate::Result;
#[cfg(feature = "cli")]
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
#[cfg(feature = "cli")]
use comfy_table::presets::UTF8_FULL;
#[cfg(feature = "cli")]
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
#[cfg(feature = "cli")]
use console::style;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Display)]
pub enum GradeLevel {
    #[default]
    Excellent,
    Good,
    Fair,
    Moderate,
    Poor,
    Critical,
}

/// Represents the overall architectural grade of a project.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArchitectureGrade {
    /// Numerical score (0.0 to 10.0).
    pub score: f32,
    /// Qualitative level (Excellent, Good, etc.).
    pub level: GradeLevel,
    /// Calculated density of smells (total weighted score / analyzed files).
    pub density: f32,
}

/// Formats a location as `path:line` or `path:line:col` for terminal clickability
pub(crate) fn format_location(path: &Path, line: usize, col: Option<usize>) -> String {
    let formatted_path = ExplainEngine::format_file_path(path);
    if line == 0 {
        return formatted_path;
    }
    match col {
        Some(c) => format!("{}:{}:{}", formatted_path, line, c),
        None => format!("{}:{}", formatted_path, line),
    }
}

/// Formats a LocationDetail as `path:line[:col] (message)`
pub(crate) fn format_location_detail(loc: &LocationDetail) -> String {
    format_location_parts(
        &loc.file,
        loc.line,
        loc.column,
        loc.range.as_ref(),
        &loc.description,
    )
}

/// Generic location formatter shared between report types
pub(crate) fn format_location_parts(
    file: &Path,
    line: usize,
    column: Option<usize>,
    range: Option<&CodeRange>,
    description: &str,
) -> String {
    let line_str = if let Some(range) = range {
        if range.start_line == range.end_line {
            range.start_line.to_string()
        } else {
            format!("{}-{}", range.start_line, range.end_line)
        }
    } else {
        line.to_string()
    };

    let formatted_path = ExplainEngine::format_file_path(file);
    let base = if line == 0 && range.is_none() {
        formatted_path
    } else {
        // Don't show column if it's a multi-line range
        let is_multi_line = range.is_some_and(|r| r.start_line != r.end_line);

        match column {
            Some(c) if !is_multi_line => format!("{}:{}:{}", formatted_path, line_str, c),
            _ => format!("{}:{}", formatted_path, line_str),
        }
    };

    if description.is_empty() {
        base
    } else {
        format!("{} ({})", base, description)
    }
}

/// A comprehensive report containing all analysis results.
pub struct AnalysisReport {
    /// Number of files analyzed.
    files_analyzed: usize,
    /// Count of cyclic dependencies found.
    cyclic_dependencies: usize,
    /// Count of god modules found.
    god_modules: usize,
    /// Count of dead code files.
    dead_code: usize,
    /// Count of unused exported symbols.
    dead_symbols: usize,
    /// Count of high complexity functions.
    high_complexity_functions: usize,
    /// Count of large files.
    large_files: usize,
    /// Count of unstable interfaces.
    unstable_interfaces: usize,
    /// Count of feature envy instances.
    feature_envy: usize,
    /// Count of shotgun surgery instances.
    shotgun_surgery: usize,
    /// Count of hub dependencies.
    hub_dependencies: usize,
    /// Count of code clone instances.
    code_clones: usize,
    /// List of detected smells with their human-readable explanations.
    pub smells: Vec<(ArchSmell, Explanation)>,
    /// The project's dependency graph.
    pub graph: Option<DependencyGraph>,
    /// Symbol information for each analyzed file.
    pub file_symbols: HashMap<PathBuf, FileSymbols>,
    /// Basic metrics for each file.
    pub file_metrics: HashMap<PathBuf, FileMetrics>,
    /// Complexity details for functions in each file.
    pub function_complexity: HashMap<PathBuf, Vec<FunctionComplexity>>,
    /// Line-level ignore rules.
    pub ignored_lines: FileIgnoredLines,
    /// Git history metrics (churn).
    pub churn_map: HashMap<PathBuf, usize>,
    /// Framework-specific presets applied.
    pub presets: Vec<FrameworkPreset>,
    /// Minimum severity filter applied to the report.
    pub min_severity: Option<Severity>,
    /// Minimum score filter applied to the report.
    pub min_score: Option<u32>,
    /// Config used during analysis
    pub config: crate::config::Config,
}

impl AnalysisReport {
    pub fn files_analyzed(&self) -> usize {
        self.files_analyzed
    }

    pub fn cyclic_dependencies(&self) -> usize {
        self.cyclic_dependencies
    }

    pub fn god_modules(&self) -> usize {
        self.god_modules
    }

    pub fn dead_code(&self) -> usize {
        self.dead_code
    }

    pub fn dead_symbols(&self) -> usize {
        self.dead_symbols
    }

    pub fn high_complexity_functions(&self) -> usize {
        self.high_complexity_functions
    }

    pub fn large_files(&self) -> usize {
        self.large_files
    }

    pub fn unstable_interfaces(&self) -> usize {
        self.unstable_interfaces
    }

    pub fn feature_envy(&self) -> usize {
        self.feature_envy
    }

    pub fn shotgun_surgery(&self) -> usize {
        self.shotgun_surgery
    }

    pub fn hub_dependencies(&self) -> usize {
        self.hub_dependencies
    }

    pub fn code_clones(&self) -> usize {
        self.code_clones
    }

    pub fn set_min_severity(&mut self, severity: Severity) {
        self.min_severity = Some(severity);
    }

    pub fn set_min_score(&mut self, score: u32) {
        self.min_score = Some(score);
    }

    pub fn set_files_analyzed(&mut self, count: usize) {
        self.files_analyzed = count;
    }

    /// Recompute per-smell counters from `self.smells`.
    /// Useful if callers mutate `smells` directly.
    pub fn recompute_counts(&mut self) {
        self.update_counts();
    }

    pub fn apply_severity_config(&mut self, config: &SeverityConfig) {
        // Filter by minimum severity
        let min_sev = self.min_severity.or(config.minimum);
        if let Some(min) = min_sev {
            // Note: effective_severity now just returns s.severity because we resolve severity at detection time
            self.smells.retain(|(s, _)| s.severity >= min);
        }

        // Filter by minimum score
        let min_score = self.min_score.or(config.minimum_score);
        if let Some(ms) = min_score {
            self.smells
                .retain(|(s, _)| config.weights.score(&s.severity) >= ms);
        }

        // Sort by score (descending)
        self.smells.sort_by(|(a, _), (b, _)| {
            config
                .weights
                .score(&b.severity)
                .cmp(&config.weights.score(&a.severity))
        });

        // Update counts
        self.update_counts();
    }

    fn update_counts(&mut self) {
        self.cyclic_dependencies = 0;
        self.god_modules = 0;
        self.dead_code = 0;
        self.dead_symbols = 0;
        self.high_complexity_functions = 0;
        self.large_files = 0;
        self.unstable_interfaces = 0;
        self.feature_envy = 0;
        self.shotgun_surgery = 0;
        self.hub_dependencies = 0;
        self.code_clones = 0;

        for (smell, _) in &self.smells {
            match &smell.smell_type {
                SmellType::CyclicDependency | SmellType::CyclicDependencyCluster => {
                    self.cyclic_dependencies += 1;
                }
                SmellType::GodModule => self.god_modules += 1,
                SmellType::DeadCode => self.dead_code += 1,
                SmellType::DeadSymbol { .. } => self.dead_symbols += 1,
                SmellType::HighComplexity { .. } => self.high_complexity_functions += 1,
                SmellType::LargeFile => self.large_files += 1,
                SmellType::UnstableInterface => self.unstable_interfaces += 1,
                SmellType::FeatureEnvy { .. } => self.feature_envy += 1,
                SmellType::ShotgunSurgery => self.shotgun_surgery += 1,
                SmellType::HubDependency { .. } => self.hub_dependencies += 1,
                SmellType::CodeClone { .. } => self.code_clones += 1,
                // These types don't have dedicated summary counters yet
                SmellType::TestLeakage { .. }
                | SmellType::LayerViolation { .. }
                | SmellType::SdpViolation
                | SmellType::BarrelFileAbuse
                | SmellType::VendorCoupling { .. }
                | SmellType::SideEffectImport
                | SmellType::HubModule
                | SmellType::LowCohesion { .. }
                | SmellType::ScatteredModule { .. }
                | SmellType::HighCoupling { .. }
                | SmellType::PackageCycle { .. }
                | SmellType::DeepNesting { .. }
                | SmellType::LongParameterList { .. }
                | SmellType::PrimitiveObsession { .. }
                | SmellType::OrphanType { .. }
                | SmellType::CircularTypeDependency
                | SmellType::AbstractnessViolation
                | SmellType::ScatteredConfiguration { .. }
                | SmellType::SharedMutableState { symbol: _ }
                | SmellType::Unknown { .. } => {}
            }
        }
    }

    pub fn total_score(&self, config: &SeverityConfig) -> u32 {
        self.smells
            .iter()
            .map(|(s, _)| config.weights.score(&s.severity))
            .sum()
    }

    pub fn grade(&self, config: &SeverityConfig) -> ArchitectureGrade {
        let total_score = self.total_score(config) as f32;
        let files_analyzed = if self.files_analyzed == 0 {
            1
        } else {
            self.files_analyzed
        } as f32;
        let density = total_score / files_analyzed;

        let thresholds = &config.grade_thresholds;

        let (score, level) = if density <= thresholds.excellent {
            let s = 10.0 - (density / thresholds.excellent);
            (s.max(9.0), GradeLevel::Excellent)
        } else if density <= thresholds.good {
            let range = thresholds.good - thresholds.excellent;
            let offset = density - thresholds.excellent;
            let s = 9.0 - (offset / range);
            (s.max(8.0), GradeLevel::Good)
        } else if density <= thresholds.fair {
            let range = thresholds.fair - thresholds.good;
            let offset = density - thresholds.good;
            let s = 8.0 - (offset / range * 2.0);
            (s.max(6.0), GradeLevel::Fair)
        } else if density <= thresholds.moderate {
            let range = thresholds.moderate - thresholds.fair;
            let offset = density - thresholds.fair;
            let s = 6.0 - (offset / range * 2.0);
            (s.max(4.0), GradeLevel::Moderate)
        } else if density <= thresholds.poor {
            let range = thresholds.poor - thresholds.moderate;
            let offset = density - thresholds.moderate;
            let s = 4.0 - (offset / range * 2.0);
            (s.max(2.0), GradeLevel::Poor)
        } else {
            let range = thresholds.poor;
            let offset = density - thresholds.poor;
            let s = 2.0 - (offset / range * 2.0);
            (s.max(0.0), GradeLevel::Critical)
        };

        ArchitectureGrade {
            score,
            level,
            density,
        }
    }

    pub fn write(
        &self,
        path: Option<&Path>,
        format: crate::args::OutputFormat,
        no_diagram: bool,
        severity_config: &SeverityConfig,
        scan_root: Option<&Path>,
    ) -> Result<()> {
        match format {
            crate::args::OutputFormat::Table => self.write_table(severity_config, scan_root),
            crate::args::OutputFormat::Markdown => {
                if let Some(path) = path {
                    self.write_markdown(path, self.graph.as_ref(), !no_diagram, severity_config)
                } else {
                    let output = markdown::generate_markdown(
                        self,
                        self.graph.as_ref(),
                        !no_diagram,
                        severity_config,
                    );
                    println!("{}", output);
                    Ok(())
                }
            }
            crate::args::OutputFormat::Json => {
                if let Some(path) = path {
                    self.write_json(path, severity_config)
                } else {
                    let output = json::generate_json(self, severity_config);
                    println!("{}", serde_json::to_string_pretty(&output)?);
                    Ok(())
                }
            }
            crate::args::OutputFormat::Sarif => {
                if let Some(path) = path {
                    sarif::write_report(self, path, severity_config, scan_root)
                } else {
                    let output = sarif::generate_sarif(self, severity_config, scan_root)?;
                    println!("{}", serde_json::to_string_pretty(&output)?);
                    Ok(())
                }
            }
        }
    }

    pub fn write_table(
        &self,
        severity_config: &SeverityConfig,
        scan_root: Option<&Path>,
    ) -> Result<()> {
        let mut table = Self::create_table_header();
        let mut sorted_smells = self.smells.clone();
        sorted_smells.sort_by(|(a, _), (b, _)| {
            // First, sort by severity (descending)
            match b.severity.cmp(&a.severity) {
                std::cmp::Ordering::Equal => {
                    // For equal severity, sort by smell_type, then files, then locations
                    match a.smell_type.cmp(&b.smell_type) {
                        std::cmp::Ordering::Equal => match a.files.cmp(&b.files) {
                            std::cmp::Ordering::Equal => a.locations.cmp(&b.locations),
                            ord => ord,
                        },
                        ord => ord,
                    }
                }
                ord => ord,
            }
        });

        let canonical_scan_root = Self::get_canonical_scan_root(scan_root);

        for (smell, _explanation) in sorted_smells {
            let severity_cell = Self::format_severity_cell(&smell.severity);
            let smell_type_str = Self::format_smell_type(&smell.smell_type);
            let locations_str = Self::format_file_paths(&smell, &canonical_scan_root);
            let score = severity_config.weights.score(&smell.severity);

            table.add_row(vec![
                severity_cell,
                Cell::new(smell_type_str),
                Cell::new(locations_str),
                Cell::new(score.to_string()),
            ]);
        }

        println!(
            "\n{}\n{}",
            style("Architectural Smells Report").bold().underlined(),
            table
        );

        if self.smells.is_empty() {
            println!(
                "{}",
                style("No smells found matching current filters.").dim()
            );
        }

        Ok(())
    }

    fn create_table_header() -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Severity").add_attribute(Attribute::Bold),
                Cell::new("Smell").add_attribute(Attribute::Bold),
                Cell::new("File").add_attribute(Attribute::Bold),
                Cell::new("Score").add_attribute(Attribute::Bold),
            ]);
        table
    }

    fn get_canonical_scan_root(scan_root: Option<&Path>) -> Option<PathBuf> {
        scan_root.and_then(|p| {
            let canonical = p.canonicalize().ok()?;
            if canonical.is_file() {
                canonical.parent().map(|p| p.to_path_buf())
            } else {
                Some(canonical)
            }
        })
    }

    fn format_severity_cell(severity: &Severity) -> Cell {
        let (severity_text, color) = match severity {
            Severity::Critical => ("🔴 CRITICAL", Color::Red),
            Severity::High => ("🟠 HIGH", Color::Red),
            Severity::Medium => ("🟡 MEDIUM", Color::Yellow),
            Severity::Low => ("🔵 LOW", Color::Cyan),
        };

        let mut cell = Cell::new(severity_text).fg(color);
        if *severity == Severity::Critical {
            cell = cell.add_attribute(Attribute::Bold);
        }
        cell
    }

    fn format_smell_type(smell_type: &SmellType) -> String {
        match smell_type {
            SmellType::CyclicDependency => "Cyclic Dependency".to_string(),
            SmellType::CyclicDependencyCluster => "Cycle Cluster".to_string(),
            SmellType::GodModule => "God Module".to_string(),
            SmellType::DeadCode => "Dead Code".to_string(),
            SmellType::DeadSymbol { name, .. } => {
                format!("Dead Symbol\n({})", name)
            }
            SmellType::HighComplexity {
                name, complexity, ..
            } => {
                format!("Complexity\n({}: {})", name, complexity)
            }
            SmellType::LargeFile => "Large File".to_string(),
            SmellType::UnstableInterface => "Unstable Interface".to_string(),
            SmellType::FeatureEnvy { .. } => "Feature Envy".to_string(),
            SmellType::ShotgunSurgery => "Shotgun Surgery".to_string(),
            SmellType::HubDependency { package } => {
                format!("Hub Dependency\n({})", package)
            }
            SmellType::OrphanType { name } => {
                format!("Orphan Type\n({})", name)
            }
            SmellType::TestLeakage { test_file } => {
                format!("Test Leakage\n({})", test_file.display())
            }
            SmellType::LayerViolation {
                from_layer,
                to_layer,
            } => {
                format!("Layer Violation\n({} -> {})", from_layer, to_layer)
            }
            SmellType::SdpViolation => "SDP Violation".to_string(),
            SmellType::BarrelFileAbuse => "Barrel File Abuse".to_string(),
            SmellType::VendorCoupling { package } => {
                format!("Vendor Coupling\n({})", package)
            }
            SmellType::SideEffectImport => "Side-Effect Import".to_string(),
            SmellType::HubModule => "Hub Module".to_string(),
            SmellType::LowCohesion { lcom, .. } => {
                format!("Low Cohesion\n(LCOM: {})", lcom)
            }
            SmellType::ScatteredModule { components } => {
                format!("Scattered Module\n({} components)", components)
            }
            SmellType::HighCoupling { cbo } => {
                format!("High Coupling\n(CBO: {})", cbo)
            }
            SmellType::PackageCycle { packages } => {
                format!("Package Cycle\n({} packages)", packages.len())
            }
            SmellType::SharedMutableState { symbol } => {
                format!("Shared Mutable State\n({})", symbol)
            }
            SmellType::DeepNesting {
                name,
                depth,
                line: _,
            } => {
                format!("Deep Nesting\n({}: depth {})", name, depth)
            }
            SmellType::LongParameterList { count, name } => {
                format!("Long Parameter List\n({}: {} params)", name, count)
            }
            SmellType::PrimitiveObsession { primitives, name } => {
                format!("Primitive Obsession\n({}: {} primitives)", name, primitives)
            }
            SmellType::CircularTypeDependency => "Circular Type Dependency".to_string(),
            SmellType::AbstractnessViolation => "Abstractness Violation".to_string(),
            SmellType::ScatteredConfiguration {
                env_var,
                files_count,
            } => {
                format!(
                    "Scattered Configuration\n({}: {} files)",
                    env_var, files_count
                )
            }
            SmellType::CodeClone { .. } => "Code Clone".to_string(),
            SmellType::Unknown { raw_type } => format!("Unknown Smell ({})", raw_type),
        }
    }

    fn format_file_paths(
        smell: &crate::detectors::ArchSmell,
        canonical_scan_root: &Option<PathBuf>,
    ) -> String {
        if !smell.locations.is_empty() {
            Self::format_location_paths(&smell.locations, canonical_scan_root)
        } else {
            Self::format_simple_file_paths(&smell.files, canonical_scan_root)
        }
    }

    fn format_location_paths(
        locations: &[crate::detectors::LocationDetail],
        canonical_scan_root: &Option<PathBuf>,
    ) -> String {
        locations
            .iter()
            .map(|loc| {
                let mut loc_clone = loc.clone();
                if let Some(ref root) = canonical_scan_root {
                    if let Ok(rel) = loc.file.strip_prefix(root) {
                        if rel.as_os_str().is_empty() {
                            loc_clone.file = loc
                                .file
                                .file_name()
                                .map(PathBuf::from)
                                .unwrap_or_else(|| loc.file.clone());
                        } else {
                            loc_clone.file = rel.to_path_buf();
                        }
                    }
                }
                format_location_detail(&loc_clone)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_simple_file_paths(
        files: &[PathBuf],
        canonical_scan_root: &Option<PathBuf>,
    ) -> String {
        files
            .iter()
            .map(|f| {
                let mut display_path = f.clone();
                if let Some(ref root) = canonical_scan_root {
                    if let Ok(rel) = f.strip_prefix(root) {
                        if rel.as_os_str().is_empty() {
                            display_path = f
                                .file_name()
                                .map(PathBuf::from)
                                .unwrap_or_else(|| f.clone());
                        } else {
                            display_path = rel.to_path_buf();
                        }
                    }
                }
                ExplainEngine::format_file_path(&display_path)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn write_markdown<P: AsRef<Path>>(
        &self,
        path: P,
        graph: Option<&DependencyGraph>,
        include_diagram: bool,
        severity_config: &SeverityConfig,
    ) -> Result<()> {
        markdown::write_report(self, path, graph, include_diagram, severity_config)
    }

    pub fn write_json<P: AsRef<Path>>(&self, path: P, config: &SeverityConfig) -> Result<()> {
        json::write_report(self, path, config)
    }
}

/// Builder for AnalysisReport to improve ergonomics
#[derive(Default)]
pub struct AnalysisReportBuilder {
    smells: Vec<ArchSmell>,
    graph: Option<DependencyGraph>,
    file_symbols: HashMap<PathBuf, FileSymbols>,
    file_metrics: HashMap<PathBuf, FileMetrics>,
    function_complexity: HashMap<PathBuf, Vec<FunctionComplexity>>,
    ignored_lines: FileIgnoredLines,
    churn_map: HashMap<PathBuf, usize>,
    presets: Vec<FrameworkPreset>,
    config: Option<crate::config::Config>,
    files_analyzed: usize,
}

impl AnalysisReportBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_smells(mut self, smells: Vec<ArchSmell>) -> Self {
        self.smells = smells;
        self
    }

    pub fn with_graph(mut self, graph: Option<DependencyGraph>) -> Self {
        self.graph = graph;
        self
    }

    pub fn with_symbols(mut self, symbols: HashMap<PathBuf, FileSymbols>) -> Self {
        self.file_symbols = symbols;
        self
    }

    pub fn with_metrics(mut self, metrics: HashMap<PathBuf, FileMetrics>) -> Self {
        self.file_metrics = metrics;
        self
    }

    pub fn with_complexity(
        mut self,
        complexity: HashMap<PathBuf, Vec<FunctionComplexity>>,
    ) -> Self {
        self.function_complexity = complexity;
        self
    }

    pub fn with_ignored_lines(mut self, ignored_lines: FileIgnoredLines) -> Self {
        self.ignored_lines = ignored_lines;
        self
    }

    pub fn with_churn(mut self, churn_map: HashMap<PathBuf, usize>) -> Self {
        self.churn_map = churn_map;
        self
    }

    pub fn with_presets(mut self, presets: Vec<FrameworkPreset>) -> Self {
        self.presets = presets;
        self
    }

    pub fn with_config(mut self, config: crate::config::Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_files_analyzed(mut self, count: usize) -> Self {
        self.files_analyzed = count;
        self
    }

    pub fn build(self) -> AnalysisReport {
        let config = self.config.unwrap_or_default();
        let smells_with_explanations = self
            .smells
            .into_iter()
            .map(|smell| {
                let explanation = ExplainEngine::explain(&smell, &config);
                (smell, explanation)
            })
            .collect();

        let mut report = AnalysisReport {
            files_analyzed: self.files_analyzed,
            cyclic_dependencies: 0,
            god_modules: 0,
            dead_code: 0,
            dead_symbols: 0,
            high_complexity_functions: 0,
            large_files: 0,
            unstable_interfaces: 0,
            feature_envy: 0,
            shotgun_surgery: 0,
            hub_dependencies: 0,
            code_clones: 0,
            smells: smells_with_explanations,
            graph: self.graph,
            file_symbols: self.file_symbols,
            file_metrics: self.file_metrics,
            function_complexity: self.function_complexity,
            ignored_lines: self.ignored_lines,
            churn_map: self.churn_map,
            presets: self.presets,
            min_severity: None,
            min_score: None,
            config,
        };

        report.update_counts();
        report
    }
}
