use napi_derive::napi;

// ============ Options ============

#[napi(object)]
#[derive(Default, Clone)]
pub struct JsScanOptions {
    /// Path to config file
    pub config_path: Option<String>,
    /// Only run these detectors (by ID)
    pub detectors: Option<Vec<String>>,
    /// Exclude these detectors (by ID)
    pub exclude_detectors: Option<Vec<String>>,
    /// Minimum severity to report
    pub min_severity: Option<String>,
    /// Minimum score to report
    pub min_score: Option<u32>,
    /// Enable caching (default: true)
    pub cache: Option<bool>,
    /// Enable git integration (default: true)
    pub git: Option<bool>,
    /// Git history analysis period (e.g. "90d", "1y", "all")
    pub git_history_period: Option<String>,
    /// Maximum file size in bytes to analyze
    pub max_file_size: Option<u32>,
}

// ============ Results ============

#[napi(object)]
pub struct JsScanResult {
    pub smells: Vec<JsSmellWithExplanation>,
    pub summary: JsSummary,
    pub files: Vec<JsFileInfo>,
    pub grade: JsArchitectureGrade,
    pub project_path: String,
}

#[napi(object)]
pub struct JsIncrementalResult {
    pub smells: Vec<JsSmellWithExplanation>,
    pub affected_files: Vec<String>,
    pub changed_count: u32,
    pub affected_count: u32,
    pub analysis_time_ms: u32,
}

#[napi(object)]
pub struct JsSmellWithExplanation {
    pub smell: JsSmell,
    pub explanation: JsExplanation,
}

#[napi(object)]
pub struct JsSmell {
    pub smell_type: String,
    pub severity: String,
    pub files: Vec<String>,
    pub locations: Vec<JsLocationDetail>,
    /// Additional metrics as JSON
    #[napi(ts_type = "Record<string, unknown>")]
    pub metrics: serde_json::Value,
    pub cluster: Option<JsCycleCluster>,
}

#[napi(object)]
pub struct JsLocationDetail {
    pub file: String,
    pub line: u32,
    pub column: Option<u32>,
    pub range: Option<JsCodeRange>,
    pub description: String,
}

#[napi(object)]
pub struct JsCodeRange {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[napi(object)]
pub struct JsExplanation {
    pub problem: String,
    pub reason: String,
    pub risks: Vec<String>,
    pub recommendations: Vec<String>,
}

// ============ Summary ============

#[napi(object)]
pub struct JsSummary {
    pub files_analyzed: u32,
    pub total_smells: u32,
    pub cyclic_dependencies: u32,
    pub cycle_clusters: u32,
    pub files_in_cycles: u32,
    pub god_modules: u32,
    pub dead_code: u32,
    pub dead_symbols: u32,
    pub high_complexity_functions: u32,
    pub unstable_interfaces: u32,
    pub feature_envy: u32,
    pub shotgun_surgery: u32,
    pub hub_dependencies: u32,
}

#[napi(object)]
pub struct JsArchitectureGrade {
    pub score: f64,
    pub level: String,
    pub density: f64,
}

// ============ File Info ============

#[napi(object)]
pub struct JsFileInfo {
    pub path: String,
    pub relative_path: String,
    pub imports: Vec<JsImportInfo>,
    pub exports: Vec<JsExportInfo>,
    pub metrics: JsFileMetrics,
}

#[napi(object)]
pub struct JsImportInfo {
    pub source: String,
    pub names: Vec<String>,
    pub line: u32,
    pub is_default: bool,
    pub is_namespace: bool,
}

#[napi(object)]
pub struct JsExportInfo {
    pub name: String,
    pub kind: String,
    pub is_default: bool,
    pub source: Option<String>,
}

#[napi(object)]
pub struct JsFileMetrics {
    pub lines: u32,
    pub complexity: Option<u32>,
    pub fan_in: u32,
    pub fan_out: u32,
}

// ============ Cycle Info ============

#[napi(object)]
pub struct JsCycleCluster {
    pub files: Vec<String>,
    pub hotspots: Vec<JsCycleHotspot>,
    pub critical_edges: Vec<JsCriticalEdge>,
}

#[napi(object)]
pub struct JsCycleHotspot {
    pub file: String,
    pub in_degree: u32,
    pub out_degree: u32,
}

#[napi(object)]
pub struct JsCriticalEdge {
    pub from: String,
    pub to: String,
    pub line: u32,
    pub range: Option<JsCodeRange>,
    pub impact: String,
}

// ============ Detector Info ============

#[napi(object)]
pub struct JsDetectorInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub default_enabled: bool,
    pub is_deep: bool,
}

// ============ Config ============

#[napi(object)]
pub struct JsConfig {
    pub ignore: Vec<String>,
    #[napi(ts_type = "Record<string, string>")]
    pub aliases: serde_json::Value,
    #[napi(ts_type = "Record<string, any>")]
    pub rules: serde_json::Value,
    #[napi(ts_type = "any[]")]
    pub overrides: serde_json::Value,
    #[napi(ts_type = "any")]
    pub scoring: serde_json::Value,
    pub entry_points: Vec<String>,
    pub enable_git: bool,
}

// ============ Conversions ============

impl From<JsScanOptions> for archlint::ScanOptions {
    fn from(opts: JsScanOptions) -> Self {
        archlint::ScanOptions {
            config_path: opts.config_path.map(Into::into),
            config: None,
            detectors: opts.detectors,
            exclude_detectors: opts.exclude_detectors.unwrap_or_default(),
            min_severity: opts.min_severity.and_then(|s| s.parse().ok()),
            min_score: opts.min_score,
            enable_cache: opts.cache.unwrap_or(true),
            enable_git: opts.git.unwrap_or(true),
            git_history_period: opts.git_history_period,
            max_file_size: opts.max_file_size.map(|s| s as u64),
        }
    }
}

impl From<archlint::ScanResult> for JsScanResult {
    fn from(res: archlint::ScanResult) -> Self {
        Self {
            smells: res.smells.into_iter().map(Into::into).collect(),
            summary: res.summary.into(),
            files: res.files.into_iter().map(Into::into).collect(),
            grade: res.grade.into(),
            project_path: res.project_path.to_string_lossy().to_string(),
        }
    }
}

impl From<archlint::IncrementalResult> for JsIncrementalResult {
    fn from(res: archlint::IncrementalResult) -> Self {
        Self {
            smells: res.smells.into_iter().map(Into::into).collect(),
            affected_files: res
                .affected_files
                .into_iter()
                .map(|p: std::path::PathBuf| p.to_string_lossy().to_string())
                .collect(),
            changed_count: res.changed_count as u32,
            affected_count: res.affected_count as u32,
            analysis_time_ms: res.analysis_time_ms as u32,
        }
    }
}

impl From<archlint::SmellWithExplanation> for JsSmellWithExplanation {
    fn from(s: archlint::SmellWithExplanation) -> Self {
        Self {
            smell: s.smell.into(),
            explanation: s.explanation.into(),
        }
    }
}

impl From<archlint::ArchSmell> for JsSmell {
    fn from(s: archlint::ArchSmell) -> Self {
        Self {
            smell_type: format!("{:?}", s.smell_type),
            severity: format!("{:?}", s.severity),
            files: s
                .files
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            locations: s.locations.into_iter().map(Into::into).collect(),
            metrics: serde_json::to_value(&s.metrics)
                .unwrap_or(serde_json::Value::Object(Default::default())),
            cluster: s.cluster.map(Into::into),
        }
    }
}

impl From<archlint::LocationDetail> for JsLocationDetail {
    fn from(l: archlint::LocationDetail) -> Self {
        Self {
            file: l.file.to_string_lossy().to_string(),
            line: l.line as u32,
            column: l.column.map(|c| c as u32),
            range: l.range.map(Into::into),
            description: l.description,
        }
    }
}

impl From<archlint::CodeRange> for JsCodeRange {
    fn from(r: archlint::CodeRange) -> Self {
        Self {
            start_line: r.start_line as u32,
            start_column: r.start_column as u32,
            end_line: r.end_line as u32,
            end_column: r.end_column as u32,
        }
    }
}

impl From<archlint::Explanation> for JsExplanation {
    fn from(e: archlint::Explanation) -> Self {
        Self {
            problem: e.problem,
            reason: e.reason,
            risks: e.risks,
            recommendations: e.recommendations,
        }
    }
}

impl From<archlint::Summary> for JsSummary {
    fn from(s: archlint::Summary) -> Self {
        Self {
            files_analyzed: s.files_analyzed as u32,
            total_smells: s.total_smells as u32,
            cyclic_dependencies: s.cyclic_dependencies as u32,
            cycle_clusters: s.cycle_clusters as u32,
            files_in_cycles: s.files_in_cycles as u32,
            god_modules: s.god_modules as u32,
            dead_code: s.dead_code as u32,
            dead_symbols: s.dead_symbols as u32,
            high_complexity_functions: s.high_complexity_functions as u32,
            unstable_interfaces: s.unstable_interfaces as u32,
            feature_envy: s.feature_envy as u32,
            shotgun_surgery: s.shotgun_surgery as u32,
            hub_dependencies: s.hub_dependencies as u32,
        }
    }
}

impl From<archlint::ArchitectureGrade> for JsArchitectureGrade {
    fn from(g: archlint::ArchitectureGrade) -> Self {
        Self {
            score: g.score as f64,
            level: format!("{:?}", g.level),
            density: g.density as f64,
        }
    }
}

impl From<archlint::FileInfo> for JsFileInfo {
    fn from(f: archlint::FileInfo) -> Self {
        Self {
            path: f.path.to_string_lossy().to_string(),
            relative_path: f.relative_path.to_string_lossy().to_string(),
            imports: f.imports.into_iter().map(Into::into).collect(),
            exports: f.exports.into_iter().map(Into::into).collect(),
            metrics: f.metrics.into(),
        }
    }
}

impl From<archlint::ImportInfo> for JsImportInfo {
    fn from(i: archlint::ImportInfo) -> Self {
        Self {
            source: i.source,
            names: i.names,
            line: i.line as u32,
            is_default: i.is_default,
            is_namespace: i.is_namespace,
        }
    }
}

impl From<archlint::ExportInfo> for JsExportInfo {
    fn from(e: archlint::ExportInfo) -> Self {
        Self {
            name: e.name,
            kind: format!("{:?}", e.kind).to_lowercase(),
            is_default: e.is_default,
            source: e.source,
        }
    }
}

impl From<archlint::FileMetrics> for JsFileMetrics {
    fn from(m: archlint::FileMetrics) -> Self {
        Self {
            lines: m.lines as u32,
            complexity: m.complexity.map(|c| c as u32),
            fan_in: m.fan_in as u32,
            fan_out: m.fan_out as u32,
        }
    }
}

impl From<archlint::CycleCluster> for JsCycleCluster {
    fn from(c: archlint::CycleCluster) -> Self {
        Self {
            files: c
                .files
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            hotspots: c
                .hotspots
                .into_iter()
                .map(|h| JsCycleHotspot {
                    file: h.file.to_string_lossy().to_string(),
                    in_degree: h.in_degree as u32,
                    out_degree: h.out_degree as u32,
                })
                .collect(),
            critical_edges: c
                .critical_edges
                .into_iter()
                .map(|e| JsCriticalEdge {
                    from: e.from.to_string_lossy().to_string(),
                    to: e.to.to_string_lossy().to_string(),
                    line: e.line as u32,
                    range: e.range.map(Into::into),
                    impact: e.impact,
                })
                .collect(),
        }
    }
}

impl From<archlint::DetectorInfo> for JsDetectorInfo {
    fn from(i: archlint::DetectorInfo) -> Self {
        Self {
            id: i.id.to_string(),
            name: i.name.to_string(),
            description: i.description.to_string(),
            default_enabled: i.default_enabled,
            is_deep: i.is_deep,
        }
    }
}

impl From<archlint::Config> for JsConfig {
    fn from(c: archlint::Config) -> Self {
        Self {
            ignore: c.ignore,
            aliases: serde_json::to_value(&c.aliases)
                .unwrap_or(serde_json::Value::Object(Default::default())),
            rules: serde_json::to_value(&c.rules)
                .unwrap_or(serde_json::Value::Object(Default::default())),
            overrides: serde_json::to_value(&c.overrides)
                .unwrap_or(serde_json::Value::Array(Default::default())),
            scoring: serde_json::to_value(&c.scoring)
                .unwrap_or(serde_json::Value::Object(Default::default())),
            entry_points: c.entry_points,
            enable_git: c.enable_git,
        }
    }
}

// ============ State Stats ============

#[napi(object)]
pub struct JsStateStats {
    pub files_count: u32,
    pub graph_nodes: u32,
    pub graph_edges: u32,
}

impl From<archlint::StateStats> for JsStateStats {
    fn from(s: archlint::StateStats) -> Self {
        Self {
            files_count: s.files_count as u32,
            graph_nodes: s.graph_nodes as u32,
            graph_edges: s.graph_edges as u32,
        }
    }
}

// ============ Diff ============

#[napi(object)]
pub struct JsDiffOptions {
    pub baseline_path: String,
    pub project_path: String,
    pub current_path: Option<String>,
}

#[napi(object)]
pub struct JsDiffResult {
    pub has_regressions: bool,
    pub regressions: Vec<JsRegression>,
    pub improvements: Vec<JsImprovement>,
    pub summary: JsDiffSummary,
    pub baseline_commit: Option<String>,
    pub current_commit: Option<String>,
}

#[napi(object)]
pub struct JsRegression {
    pub id: String,
    pub regression_type: JsRegressionType,
    pub smell: JsSnapshotSmell,
    pub message: String,
    pub explain: Option<JsExplainBlock>,
}

#[napi(object)]
pub struct JsRegressionType {
    pub r#type: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub metric: Option<String>,
    pub from_val: Option<f64>,
    pub to_val: Option<f64>,
    pub change_percent: Option<f64>,
}

#[napi(object)]
pub struct JsImprovement {
    pub id: String,
    pub improvement_type: JsImprovementType,
    pub message: String,
}

#[napi(object)]
pub struct JsImprovementType {
    pub r#type: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub metric: Option<String>,
    pub from_val: Option<f64>,
    pub to_val: Option<f64>,
}

#[napi(object)]
pub struct JsDiffSummary {
    pub new_smells: u32,
    pub fixed_smells: u32,
    pub worsened_smells: u32,
    pub improved_smells: u32,
    pub total_regressions: u32,
    pub total_improvements: u32,
}

#[napi(object)]
pub struct JsExplainBlock {
    pub why_bad: String,
    pub consequences: String,
    pub how_to_fix: String,
}

#[napi(object)]
pub struct JsSnapshotSmell {
    pub id: String,
    pub smell_type: String,
    pub severity: String,
    pub files: Vec<String>,
    #[napi(ts_type = "Record<string, unknown>")]
    pub metrics: serde_json::Value,
    #[napi(ts_type = "unknown")]
    pub details: Option<serde_json::Value>,
    pub locations: Vec<JsLocationDetail>,
}

impl From<archlint::diff::DiffResult> for JsDiffResult {
    fn from(res: archlint::diff::DiffResult) -> Self {
        Self {
            has_regressions: res.has_regressions,
            regressions: res.regressions.into_iter().map(Into::into).collect(),
            improvements: res.improvements.into_iter().map(Into::into).collect(),
            summary: res.summary.into(),
            baseline_commit: res.baseline_commit,
            current_commit: res.current_commit,
        }
    }
}

impl From<archlint::diff::Regression> for JsRegression {
    fn from(r: archlint::diff::Regression) -> Self {
        Self {
            id: r.id,
            regression_type: r.regression_type.into(),
            smell: r.smell.into(),
            message: r.message,
            explain: r.explain.map(Into::into),
        }
    }
}

impl From<archlint::diff::RegressionType> for JsRegressionType {
    fn from(rt: archlint::diff::RegressionType) -> Self {
        match rt {
            archlint::diff::RegressionType::NewSmell => Self {
                r#type: "NewSmell".into(),
                from: None,
                to: None,
                metric: None,
                from_val: None,
                to_val: None,
                change_percent: None,
            },
            archlint::diff::RegressionType::SeverityIncrease { from, to } => Self {
                r#type: "SeverityIncrease".into(),
                from: Some(from),
                to: Some(to),
                metric: None,
                from_val: None,
                to_val: None,
                change_percent: None,
            },
            archlint::diff::RegressionType::MetricWorsening {
                metric,
                from,
                to,
                change_percent,
            } => Self {
                r#type: "MetricWorsening".into(),
                from: None,
                to: None,
                metric: Some(metric),
                from_val: Some(from),
                to_val: Some(to),
                change_percent: Some(change_percent),
            },
        }
    }
}

impl From<archlint::diff::Improvement> for JsImprovement {
    fn from(i: archlint::diff::Improvement) -> Self {
        Self {
            id: i.id,
            improvement_type: i.improvement_type.into(),
            message: i.message,
        }
    }
}

impl From<archlint::diff::ImprovementType> for JsImprovementType {
    fn from(it: archlint::diff::ImprovementType) -> Self {
        match it {
            archlint::diff::ImprovementType::Fixed => Self {
                r#type: "Fixed".into(),
                from: None,
                to: None,
                metric: None,
                from_val: None,
                to_val: None,
            },
            archlint::diff::ImprovementType::SeverityDecrease { from, to } => Self {
                r#type: "SeverityDecrease".into(),
                from: Some(from),
                to: Some(to),
                metric: None,
                from_val: None,
                to_val: None,
            },
            archlint::diff::ImprovementType::MetricImprovement { metric, from, to } => Self {
                r#type: "MetricImprovement".into(),
                from: None,
                to: None,
                metric: Some(metric),
                from_val: Some(from),
                to_val: Some(to),
            },
        }
    }
}

impl From<archlint::diff::DiffSummary> for JsDiffSummary {
    fn from(s: archlint::diff::DiffSummary) -> Self {
        Self {
            new_smells: s.new_smells.try_into().unwrap_or(u32::MAX),
            fixed_smells: s.fixed_smells.try_into().unwrap_or(u32::MAX),
            worsened_smells: s.worsened_smells.try_into().unwrap_or(u32::MAX),
            improved_smells: s.improved_smells.try_into().unwrap_or(u32::MAX),
            total_regressions: s.total_regressions.try_into().unwrap_or(u32::MAX),
            total_improvements: s.total_improvements.try_into().unwrap_or(u32::MAX),
        }
    }
}

impl From<archlint::diff::ExplainBlock> for JsExplainBlock {
    fn from(e: archlint::diff::ExplainBlock) -> Self {
        Self {
            why_bad: e.why_bad,
            consequences: e.consequences,
            how_to_fix: e.how_to_fix,
        }
    }
}

impl From<archlint::snapshot::SnapshotSmell> for JsSnapshotSmell {
    fn from(s: archlint::snapshot::SnapshotSmell) -> Self {
        Self {
            id: s.id,
            smell_type: s.smell_type,
            severity: s.severity,
            files: s.files,
            metrics: serde_json::to_value(&s.metrics).unwrap_or(serde_json::Value::Null),
            details: s.details.and_then(|d| serde_json::to_value(&d).ok()),
            locations: s
                .locations
                .into_iter()
                .map(|l| JsLocationDetail {
                    file: l.file,
                    line: l.line as u32,
                    column: l.column.map(|c| c as u32),
                    range: None,
                    description: l.description.unwrap_or_default(),
                })
                .collect(),
        }
    }
}
