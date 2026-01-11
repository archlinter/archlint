pub mod engine;
pub mod tokenizer;
pub mod types;

use crate::detectors::{
    detector, ArchSmell, CodeRange, Detector, DetectorCategory, LocationDetail,
};
use crate::engine::AnalysisContext;
use rustc_hash::FxHashSet;

use self::engine::{build_window_map, detect_clusters, merge_overlapping_occurrences};
use self::tokenizer::tokenize_files;

/// Main detector for code clones (duplicated code blocks).
#[detector(
    id = "code_clone",
    name = "Code Clone Detector",
    description = "Detects duplicated code blocks across the project (Type-1 clones)",
    category = DetectorCategory::Global,
    is_deep = true
)]
pub struct CodeCloneDetector;

impl CodeCloneDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    /// Resolves detector configuration options from the analysis context.
    fn resolve_config(&self, ctx: &AnalysisContext) -> (usize, usize, usize) {
        let global_rule = ctx.resolve_rule("code_clone", None);
        let min_tokens: usize = global_rule.get_option("min_tokens").unwrap_or(50);
        let min_lines: usize = global_rule.get_option("min_lines").unwrap_or(6);
        let max_bucket_size: usize = global_rule.get_option("max_bucket_size").unwrap_or(1000);
        (min_tokens, min_lines, max_bucket_size)
    }

    /// Converts detected clusters into architectural smells (`ArchSmell`).
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
            for occ in &merged_occurrences {
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

    /// Formats the "Also found in" part of the smell description.
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

impl Detector for CodeCloneDetector {
    crate::impl_detector_report!(
        name: "Code Clone",
        explain: _smell => {
            crate::detectors::Explanation {
                problem: "Code Clone".into(),
                reason: "Identical or near-identical code blocks found in multiple locations. This violates the DRY (Don't Repeat Yourself) principle.".into(),
                risks: crate::strings![
                    "Increased maintenance effort",
                    "Bugs must be fixed in multiple places"
                ],
                recommendations: crate::strings![
                    "Extract the duplicated code into a shared function, class, or module"
                ]
            }
        },
        table: {
            title: "Code Clones",
            columns: ["Clone Info", "pts"],
            row: CodeClone { clone_hash, token_count } (smell, location, pts) => [
                format!("Clone `{}` ({} tokens)", &clone_hash[..8], token_count),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let (min_tokens, min_lines, max_bucket_size) = self.resolve_config(ctx);

        let file_tokens = tokenize_files(ctx, min_tokens);
        if file_tokens.is_empty() {
            return Vec::new();
        }

        let window_map = build_window_map(&file_tokens, min_tokens);
        let clusters = detect_clusters(
            &file_tokens,
            window_map,
            min_tokens,
            min_lines,
            max_bucket_size,
        );

        self.report_smells(ctx, clusters)
    }
}

pub fn init() {}
