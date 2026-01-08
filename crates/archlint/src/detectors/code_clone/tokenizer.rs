use crate::engine::AnalysisContext;
use crate::parser::tokenizer::{tokenize_and_normalize, CloneTokenizationMode, NormalizedToken};
use oxc_span::SourceType;
use rustc_hash::FxHashMap;
use std::fs;
use std::path::PathBuf;

pub fn tokenize_files(
    ctx: &AnalysisContext,
    min_tokens: usize,
    mode: CloneTokenizationMode,
) -> FxHashMap<PathBuf, Vec<NormalizedToken>> {
    let mut file_tokens = FxHashMap::default();

    for path in ctx.file_metrics.keys() {
        let rule = ctx.resolve_rule("code_clone", Some(path));
        if !rule.enabled || ctx.is_excluded(path, &rule.exclude) {
            continue;
        }

        if let Ok(source) = fs::read_to_string(path) {
            let source_type = SourceType::from_path(path).unwrap_or_default();
            let tokens = tokenize_and_normalize(&source, source_type, mode);
            if tokens.len() >= min_tokens {
                file_tokens.insert(path.clone(), tokens);
            }
        }
    }

    file_tokens
}
