use serde::{Deserialize, Serialize};
pub mod json;
pub mod markdown;
pub mod mermaid;

use crate::config::SeverityConfig;
use crate::detectors::{ArchSmell, LocationDetail};
use crate::explain::{ExplainEngine, Explanation};
use crate::graph::DependencyGraph;
use crate::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use console::style;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradeLevel {
    Excellent,
    Good,
    Fair,
    Moderate,
    Poor,
    Critical,
}

impl std::fmt::Display for GradeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GradeLevel::Excellent => write!(f, "Excellent"),
            GradeLevel::Good => write!(f, "Good"),
            GradeLevel::Fair => write!(f, "Fair"),
            GradeLevel::Moderate => write!(f, "Moderate"),
            GradeLevel::Poor => write!(f, "Poor"),
            GradeLevel::Critical => write!(f, "Critical"),
        }
    }
}

pub struct ArchitectureGrade {
    pub score: f32,
    pub level: GradeLevel,
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
    let base = format_location(&loc.file, loc.line, loc.column);
    if loc.description.is_empty() {
        base
    } else {
        format!("{} ({})", base, loc.description)
    }
}

pub struct AnalysisReport {
    pub files_analyzed: usize,
    pub cyclic_dependencies: usize,
    pub god_modules: usize,
    pub dead_code: usize,
    pub dead_symbols: usize,
    pub high_complexity_functions: usize,
    pub large_files: usize,
    pub unstable_interfaces: usize,
    pub feature_envy: usize,
    pub shotgun_surgery: usize,
    pub hub_dependencies: usize,
    pub smells: Vec<(ArchSmell, Explanation)>,
    pub graph: Option<DependencyGraph>,
}

impl AnalysisReport {
    pub fn new(smells: Vec<ArchSmell>, graph: Option<DependencyGraph>) -> Self {
        let cyclic_dependencies = smells
            .iter()
            .filter(|s| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::CyclicDependency
                        | crate::detectors::SmellType::CyclicDependencyCluster
                )
            })
            .count();

        let god_modules = smells
            .iter()
            .filter(|s| matches!(s.smell_type, crate::detectors::SmellType::GodModule))
            .count();

        let dead_code = smells
            .iter()
            .filter(|s| matches!(s.smell_type, crate::detectors::SmellType::DeadCode))
            .count();

        let dead_symbols = smells
            .iter()
            .filter(|s| matches!(s.smell_type, crate::detectors::SmellType::DeadSymbol { .. }))
            .count();

        let high_complexity_functions = smells
            .iter()
            .filter(|s| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::HighComplexity { .. }
                )
            })
            .count();

        let large_files = smells
            .iter()
            .filter(|s| matches!(s.smell_type, crate::detectors::SmellType::LargeFile))
            .count();

        let unstable_interfaces = smells
            .iter()
            .filter(|s| matches!(s.smell_type, crate::detectors::SmellType::UnstableInterface))
            .count();

        let feature_envy = smells
            .iter()
            .filter(|s| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::FeatureEnvy { .. }
                )
            })
            .count();

        let shotgun_surgery = smells
            .iter()
            .filter(|s| matches!(s.smell_type, crate::detectors::SmellType::ShotgunSurgery))
            .count();

        let hub_dependencies = smells
            .iter()
            .filter(|s| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::HubDependency { .. }
                )
            })
            .count();

        let smells_with_explanations = smells
            .into_iter()
            .map(|smell| {
                let explanation = ExplainEngine::explain(&smell);
                (smell, explanation)
            })
            .collect();

        Self {
            files_analyzed: 0,
            cyclic_dependencies,
            god_modules,
            dead_code,
            dead_symbols,
            high_complexity_functions,
            large_files,
            unstable_interfaces,
            feature_envy,
            shotgun_surgery,
            hub_dependencies,
            smells: smells_with_explanations,
            graph,
        }
    }

    pub fn set_files_analyzed(&mut self, count: usize) {
        self.files_analyzed = count;
    }

    pub fn apply_severity_config(&mut self, config: &SeverityConfig) {
        // Filter by minimum severity
        if let Some(min) = config.minimum.as_ref() {
            self.smells
                .retain(|(s, _)| s.effective_severity(config) >= *min);
        }

        // Filter by minimum score
        if let Some(min_score) = config.minimum_score {
            self.smells.retain(|(s, _)| s.score(config) >= min_score);
        }

        // Sort by score (descending)
        self.smells
            .sort_by(|(a, _), (b, _)| b.score(config).cmp(&a.score(config)));

        // Update counts
        self.cyclic_dependencies = self
            .smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::CyclicDependency
                        | crate::detectors::SmellType::CyclicDependencyCluster
                )
            })
            .count();
        self.god_modules = self
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::GodModule))
            .count();
        self.dead_code = self
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::DeadCode))
            .count();
        self.dead_symbols = self
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::DeadSymbol { .. }))
            .count();
        self.high_complexity_functions = self
            .smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::HighComplexity { .. }
                )
            })
            .count();
        self.large_files = self
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::LargeFile))
            .count();
        self.unstable_interfaces = self
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::UnstableInterface))
            .count();
        self.feature_envy = self
            .smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::FeatureEnvy { .. }
                )
            })
            .count();
        self.shotgun_surgery = self
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::ShotgunSurgery))
            .count();
        self.hub_dependencies = self
            .smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::HubDependency { .. }
                )
            })
            .count();
    }

    pub fn total_score(&self, config: &SeverityConfig) -> u32 {
        self.smells.iter().map(|(s, _)| s.score(config)).sum()
    }

    pub fn grade(&self, config: &crate::config::SeverityConfig) -> ArchitectureGrade {
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
        format: crate::cli::OutputFormat,
        no_diagram: bool,
        severity_config: &SeverityConfig,
        scan_root: Option<&Path>,
    ) -> Result<()> {
        match format {
            crate::cli::OutputFormat::Table => self.write_table(severity_config, scan_root),
            crate::cli::OutputFormat::Markdown => {
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
            crate::cli::OutputFormat::Json => {
                if let Some(path) = path {
                    self.write_json(path, severity_config)
                } else {
                    let output = json::generate_json(self, severity_config);
                    println!("{}", serde_json::to_string_pretty(&output)?);
                    Ok(())
                }
            }
        }
    }

    pub fn write_table(
        &self,
        _severity_config: &SeverityConfig,
        scan_root: Option<&Path>,
    ) -> Result<()> {
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

        let mut sorted_smells = self.smells.clone();
        sorted_smells.sort_by(|(a, _), (b, _)| b.severity.cmp(&a.severity));

        // Get canonical scan root for path comparison
        let canonical_scan_root = scan_root.and_then(|p| {
            let canonical = p.canonicalize().ok()?;
            // If scan_root is a file, use its parent directory to keep paths relative to it
            if canonical.is_file() {
                canonical.parent().map(|p| p.to_path_buf())
            } else {
                Some(canonical)
            }
        });

        for (smell, _explanation) in sorted_smells {
            let (severity_text, color) = match smell.severity {
                crate::detectors::Severity::Critical => ("🔴 CRITICAL", Color::Red),
                crate::detectors::Severity::High => ("🟠 HIGH", Color::Red),
                crate::detectors::Severity::Medium => ("🟡 MEDIUM", Color::Yellow),
                crate::detectors::Severity::Low => ("🔵 LOW", Color::Cyan),
            };

            let mut severity_cell = Cell::new(severity_text).fg(color);
            if smell.severity == crate::detectors::Severity::Critical {
                severity_cell = severity_cell.add_attribute(Attribute::Bold);
            }

            let smell_type_str = match &smell.smell_type {
                crate::detectors::SmellType::CyclicDependency => "Cyclic Dependency".to_string(),
                crate::detectors::SmellType::CyclicDependencyCluster => "Cycle Cluster".to_string(),
                crate::detectors::SmellType::GodModule => "God Module".to_string(),
                crate::detectors::SmellType::DeadCode => "Dead Code".to_string(),
                crate::detectors::SmellType::DeadSymbol { name, .. } => {
                    format!("Dead Symbol\n({})", name)
                }
                crate::detectors::SmellType::HighComplexity { name, .. } => {
                    format!("Complexity\n({})", name)
                }
                crate::detectors::SmellType::LargeFile => "Large File".to_string(),
                crate::detectors::SmellType::UnstableInterface => "Unstable Interface".to_string(),
                crate::detectors::SmellType::FeatureEnvy { .. } => "Feature Envy".to_string(),
                crate::detectors::SmellType::ShotgunSurgery => "Shotgun Surgery".to_string(),
                crate::detectors::SmellType::HubDependency { package } => {
                    format!("Hub Dependency\n({})", package)
                }
                crate::detectors::SmellType::OrphanType { name } => {
                    format!("Orphan Type\n({})", name)
                }
                crate::detectors::SmellType::TestLeakage { test_file } => {
                    format!("Test Leakage\n({})", test_file.display())
                }
                crate::detectors::SmellType::LayerViolation {
                    from_layer,
                    to_layer,
                } => {
                    format!("Layer Violation\n({} -> {})", from_layer, to_layer)
                }
                crate::detectors::SmellType::SdpViolation => "SDP Violation".to_string(),
                crate::detectors::SmellType::BarrelFileAbuse => "Barrel File Abuse".to_string(),
                crate::detectors::SmellType::VendorCoupling { package } => {
                    format!("Vendor Coupling\n({})", package)
                }
                crate::detectors::SmellType::SideEffectImport => "Side-Effect Import".to_string(),
                crate::detectors::SmellType::HubModule => "Hub Module".to_string(),
                crate::detectors::SmellType::LowCohesion { lcom } => {
                    format!("Low Cohesion\n(LCOM: {})", lcom)
                }
                crate::detectors::SmellType::ScatteredModule { components } => {
                    format!("Scattered Module\n({} components)", components)
                }
                crate::detectors::SmellType::HighCoupling { cbo } => {
                    format!("High Coupling\n(CBO: {})", cbo)
                }
                crate::detectors::SmellType::PackageCycle { packages } => {
                    format!("Package Cycle\n({} packages)", packages.len())
                }
                crate::detectors::SmellType::SharedMutableState { symbol } => {
                    format!("Shared Mutable State\n({})", symbol)
                }
                crate::detectors::SmellType::DeepNesting { depth } => {
                    format!("Deep Nesting\n(depth: {})", depth)
                }
                crate::detectors::SmellType::LongParameterList { count, function } => {
                    format!("Long Parameter List\n({}: {} params)", function, count)
                }
                crate::detectors::SmellType::PrimitiveObsession {
                    primitives,
                    function,
                } => {
                    format!(
                        "Primitive Obsession\n({}: {} primitives)",
                        function, primitives
                    )
                }
                crate::detectors::SmellType::CircularTypeDependency => {
                    "Circular Type Dependency".to_string()
                }
                crate::detectors::SmellType::AbstractnessViolation => {
                    "Abstractness Violation".to_string()
                }
                crate::detectors::SmellType::ScatteredConfiguration {
                    env_var,
                    files_count,
                } => {
                    format!(
                        "Scattered Configuration\n({}: {} files)",
                        env_var, files_count
                    )
                }
            };

            let locations_str = if !smell.locations.is_empty() {
                smell
                    .locations
                    .iter()
                    .map(|loc| {
                        let mut loc_clone = loc.clone();
                        // Try to make path relative to scan root
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
            } else {
                smell
                    .files
                    .iter()
                    .map(|f| {
                        let mut display_path = f.clone();
                        // Try to make path relative to scan root
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
            };

            let score = smell.score(_severity_config);

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

        Ok(())
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

    pub fn write_json<P: AsRef<Path>>(
        &self,
        path: P,
        config: &crate::config::SeverityConfig,
    ) -> Result<()> {
        json::write_report(self, path, config)
    }
}
