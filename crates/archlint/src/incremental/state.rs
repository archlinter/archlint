use crate::detectors::ArchSmell;
use crate::engine::context::FileMetrics;
use crate::framework::presets::FrameworkPreset;
use crate::framework::Framework;
use crate::graph::DependencyGraph;
use crate::parser::{FileIgnoredLines, FileSymbols, FunctionComplexity};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

pub struct IncrementalState {
    // Heavy structures wrapped in Arc
    pub graph: Arc<DependencyGraph>,
    pub file_symbols: Arc<HashMap<PathBuf, FileSymbols>>,
    pub file_metrics: Arc<HashMap<PathBuf, FileMetrics>>,
    pub function_complexity: Arc<HashMap<PathBuf, Vec<FunctionComplexity>>>,
    pub ignored_lines: Arc<FileIgnoredLines>,

    // Light structures remain owned
    pub file_hashes: HashMap<PathBuf, String>,
    pub churn_map: HashMap<PathBuf, usize>,

    // Reverse dependency index (file -> importers)
    pub reverse_deps: HashMap<PathBuf, HashSet<PathBuf>>,

    // Metadata
    pub project_root: PathBuf,
    pub config_hash: String,
    pub created_at: Instant,
    pub last_full_scan: Option<Instant>,

    // Framework and project info
    pub detected_frameworks: Vec<Framework>,
    pub presets: Vec<FrameworkPreset>,
    pub script_entry_points: HashSet<PathBuf>,
    pub dynamic_load_patterns: Vec<String>,

    /// Cache for file-local detector results: (detector_id, file_path) -> smells
    pub file_local_cache: HashMap<(String, PathBuf), Vec<ArchSmell>>,
}

impl IncrementalState {
    pub fn new(project_root: PathBuf, config_hash: String) -> Self {
        Self {
            graph: Arc::new(DependencyGraph::new()),
            file_symbols: Arc::new(HashMap::new()),
            file_metrics: Arc::new(HashMap::new()),
            function_complexity: Arc::new(HashMap::new()),
            ignored_lines: Arc::new(FileIgnoredLines::default()),
            file_hashes: HashMap::new(),
            churn_map: HashMap::new(),
            reverse_deps: HashMap::new(),
            project_root,
            config_hash,
            created_at: Instant::now(),
            last_full_scan: None,
            detected_frameworks: Vec::new(),
            presets: Vec::new(),
            script_entry_points: HashSet::new(),
            dynamic_load_patterns: Vec::new(),
            file_local_cache: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.graph = Arc::new(DependencyGraph::new());
        self.file_symbols = Arc::new(HashMap::new());
        self.file_metrics = Arc::new(HashMap::new());
        self.function_complexity = Arc::new(HashMap::new());
        self.ignored_lines = Arc::new(FileIgnoredLines::default());
        self.file_hashes.clear();
        self.churn_map.clear();
        self.reverse_deps.clear();
        self.last_full_scan = None;
        self.detected_frameworks.clear();
        self.presets.clear();
        self.script_entry_points.clear();
        self.dynamic_load_patterns.clear();
        self.file_local_cache.clear();
    }

    /// Get mutable access to graph via Arc::make_mut (copy-on-write)
    pub fn graph_mut(&mut self) -> &mut DependencyGraph {
        Arc::make_mut(&mut self.graph)
    }

    /// Get mutable access to file_symbols via Arc::make_mut (copy-on-write)
    pub fn file_symbols_mut(&mut self) -> &mut HashMap<PathBuf, FileSymbols> {
        Arc::make_mut(&mut self.file_symbols)
    }

    /// Get mutable access to file_metrics via Arc::make_mut (copy-on-write)
    pub fn file_metrics_mut(&mut self) -> &mut HashMap<PathBuf, FileMetrics> {
        Arc::make_mut(&mut self.file_metrics)
    }

    /// Get mutable access to function_complexity via Arc::make_mut (copy-on-write)
    pub fn function_complexity_mut(&mut self) -> &mut HashMap<PathBuf, Vec<FunctionComplexity>> {
        Arc::make_mut(&mut self.function_complexity)
    }

    /// Get mutable access to ignored_lines via Arc::make_mut (copy-on-write)
    pub fn ignored_lines_mut(&mut self) -> &mut FileIgnoredLines {
        Arc::make_mut(&mut self.ignored_lines)
    }
}
