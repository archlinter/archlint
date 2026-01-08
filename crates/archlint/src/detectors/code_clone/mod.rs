pub mod engine;
pub mod tokenizer;
pub mod types;

use crate::config::Config;
use crate::detectors::{
    ArchSmell, CodeRange, Detector, DetectorCategory, DetectorFactory, DetectorInfo, LocationDetail,
};
use crate::engine::AnalysisContext;
use crate::parser::tokenizer::CloneTokenizationMode;
use rustc_hash::FxHashSet;

use self::engine::{build_window_map, detect_clusters, merge_overlapping_occurrences};
use self::tokenizer::tokenize_files;

pub struct CodeCloneDetector;

pub struct CodeCloneDetectorFactory;

impl DetectorFactory for CodeCloneDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "code_clone",
            name: "Code Clone Detector",
            description:
                "Detects duplicated code blocks across the project (Type-1 and Type-2 clones)",
            default_enabled: true,
            is_deep: true,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(CodeCloneDetector)
    }
}

inventory::submit! {
    &CodeCloneDetectorFactory as &dyn DetectorFactory
}

impl Detector for CodeCloneDetector {
    fn name(&self) -> &'static str {
        "Code Clone"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let (min_tokens, min_lines, mode) = self.resolve_config(ctx);

        let file_tokens = tokenize_files(ctx, min_tokens, mode);
        if file_tokens.is_empty() {
            return Vec::new();
        }

        let window_map = build_window_map(&file_tokens, min_tokens);
        let clusters = detect_clusters(&file_tokens, window_map, min_tokens, min_lines);

        self.report_smells(ctx, clusters)
    }
}

impl CodeCloneDetector {
    fn resolve_config(&self, ctx: &AnalysisContext) -> (usize, usize, CloneTokenizationMode) {
        let global_rule = ctx.resolve_rule("code_clone", None);
        let min_tokens: usize = global_rule.get_option("min_tokens").unwrap_or(50);
        let min_lines: usize = global_rule.get_option("min_lines").unwrap_or(6);
        let mode = match global_rule
            .get_option::<String>("mode")
            .unwrap_or_else(|| "exact".to_string())
            .to_lowercase()
            .as_str()
        {
            "exact" | "type1" => CloneTokenizationMode::Exact,
            _ => CloneTokenizationMode::Type2,
        };
        (min_tokens, min_lines, mode)
    }

    fn report_smells(
        &self,
        ctx: &AnalysisContext,
        clusters: Vec<types::Cluster>,
    ) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for cluster in clusters.into_iter().filter(|c| c.occurrences.len() >= 2) {
            let hash_str = cluster
                .hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();

            let merged_occurrences = merge_overlapping_occurrences(cluster.occurrences);

            if merged_occurrences.len() < 2 {
                continue;
            }

            let mut location_details = Vec::new();
            if let Some(occ) = merged_occurrences.first() {
                let other_refs = self.format_other_refs(ctx, &merged_occurrences, occ);

                let description = format!(
                    "Duplicated code ({} tokens, lines {}-{}). Also found in: {}",
                    cluster.token_count, occ.start_line, occ.end_line, other_refs
                );

                let range = CodeRange {
                    start_line: occ.start_line,
                    start_column: occ.start_column,
                    end_line: occ.end_line,
                    end_column: occ.end_column,
                };

                location_details.push(
                    LocationDetail::new(occ.file.clone(), occ.start_line, description)
                        .with_range(range),
                );
            }

            smells.push(ArchSmell::new_code_clone(
                location_details,
                cluster.token_count,
                hash_str,
            ));
        }

        smells
    }

    fn format_other_refs(
        &self,
        ctx: &AnalysisContext,
        merged_occurrences: &[types::Occurrence],
        primary_occ: &types::Occurrence,
    ) -> String {
        let mut seen = FxHashSet::<String>::default();
        let mut other_refs = merged_occurrences
            .iter()
            .filter(|o| !(o.file == primary_occ.file && o.token_start == primary_occ.token_start))
            .filter_map(|o| {
                let rel = o
                    .file
                    .strip_prefix(&ctx.project_path)
                    .unwrap_or(&o.file)
                    .to_string_lossy();
                let key = if o.start_line == o.end_line {
                    format!("{}:{}", rel, o.start_line)
                } else {
                    format!("{}:{}-{}", rel, o.start_line, o.end_line)
                };
                if seen.insert(key.clone()) {
                    Some(key)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        other_refs.sort();
        other_refs.join(", ")
    }
}

pub fn init() {}
