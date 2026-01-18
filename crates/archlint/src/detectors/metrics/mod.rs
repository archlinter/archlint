use crate::detectors::ArchSmell;
use crate::engine::AnalysisContext;
use crate::parser::FunctionComplexity;
use crate::rule_resolver::ResolvedRuleConfig;
use std::path::Path;

pub mod cognitive_complexity;
pub mod cyclomatic_complexity;
pub mod deep_nesting;
pub mod large_file;
pub mod lcom;
pub mod long_params;

pub const fn init() {
    cognitive_complexity::init();
    cyclomatic_complexity::init();
    deep_nesting::init();
    large_file::init();
    lcom::init();
    long_params::init();
}

/// Common detection logic for both cyclomatic and cognitive complexity.
#[must_use]
pub fn detect_complexity_smells(
    ctx: &AnalysisContext,
    detector_id: &str,
    is_cognitive: bool,
) -> Vec<ArchSmell> {
    let mut smells = Vec::new();

    for (path, functions) in ctx.function_complexity.as_ref() {
        let Some(rule) = resolve_complexity_rule(ctx, detector_id, path) else {
            continue;
        };

        let threshold = get_complexity_threshold(&rule);

        for func in functions {
            if let Some(mut smell) = detect_function_smell(path, func, threshold, is_cognitive) {
                smell.severity = rule.severity;
                smells.push(smell);
            }
        }
    }

    smells
}

fn resolve_complexity_rule(
    ctx: &AnalysisContext,
    detector_id: &str,
    path: &Path,
) -> Option<ResolvedRuleConfig> {
    if let Some(rule) = ctx.get_rule_for_file(detector_id, path) {
        return Some(rule);
    }

    if detector_id == "cyclomatic_complexity" {
        return ctx.get_rule_for_file("complexity", path);
    }

    None
}

fn get_complexity_threshold(rule: &ResolvedRuleConfig) -> usize {
    rule.get_option("max_complexity")
        .or(rule.get_option("function_threshold"))
        .or(rule.get_option("threshold"))
        .unwrap_or(15)
}

fn detect_function_smell(
    path: &Path,
    func: &FunctionComplexity,
    threshold: usize,
    is_cognitive: bool,
) -> Option<ArchSmell> {
    let val = if is_cognitive {
        func.cognitive_complexity
    } else {
        func.cyclomatic_complexity
    };

    if val <= threshold {
        return None;
    }

    let smell = if is_cognitive {
        ArchSmell::new_high_cognitive_complexity(
            path.to_path_buf(),
            func.name.to_string(),
            func.line,
            val,
            threshold,
            Some(func.range),
        )
    } else {
        ArchSmell::new_high_cyclomatic_complexity(
            path.to_path_buf(),
            func.name.to_string(),
            func.line,
            val,
            threshold,
            Some(func.range),
        )
    };

    Some(smell)
}
