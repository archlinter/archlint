use crate::args::ScanArgs;
use crate::config::{Config, RuleConfig, RuleSeverity};
use crate::detectors::{self, registry::DetectorRegistry};
use crate::engine::progress::{
    create_progress_bar, default_progress_chars, detector_progress_template,
};
use crate::engine::AnalysisContext;
use crate::framework::presets::FrameworkPreset;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;
use crate::Result;
#[cfg(feature = "cli")]
use console::style;
use log::info;
use rayon::prelude::*;
use std::collections::HashSet;

pub struct DetectorRunner<'a> {
    args: &'a ScanArgs,
}

impl<'a> DetectorRunner<'a> {
    #[must_use]
    pub const fn new(args: &'a ScanArgs) -> Self {
        Self { args }
    }

    pub fn run_detectors(
        &self,
        ctx: &AnalysisContext,
        use_progress: bool,
        presets: &[FrameworkPreset],
    ) -> Result<Vec<detectors::ArchSmell>> {
        let registry = DetectorRegistry::new();
        let (enabled_detectors, _) =
            registry.get_enabled_full(&ctx.config, presets, self.args.all_detectors);

        let final_detectors = self.filter_detectors(enabled_detectors, |(id, _)| id);

        let needs_deep = final_detectors
            .iter()
            .any(|(id, _)| registry.get_info(id).is_some_and(|info| info.is_deep));

        info!(
            "{} Detecting architectural smells...{}",
            style("🧪").green().bold(),
            if needs_deep {
                style(" (deep analysis enabled)").dim().to_string()
            } else {
                String::new()
            }
        );

        let pb = if use_progress {
            Some(create_progress_bar(
                final_detectors.len(),
                detector_progress_template(),
                default_progress_chars(),
            ))
        } else {
            None
        };

        let results: Vec<_> = final_detectors
            .into_par_iter()
            .map(|(_, detector)| {
                let smells = detector.detect(ctx);
                (detector.name().to_string(), smells)
            })
            .collect();

        let mut all_smells = Vec::new();
        for (name, smells) in results {
            let status = format!(
                "   {} {:<27} found: {}",
                style("↳").dim(),
                name,
                if smells.is_empty() {
                    style("0".to_string()).dim()
                } else {
                    style(smells.len().to_string()).red().bold()
                }
            );

            if let Some(ref pb) = pb {
                pb.set_message(name);
                pb.println(status);
                pb.inc(1);
            } else {
                info!("{status}");
            }
            all_smells.extend(smells);
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        Ok(all_smells)
    }

    #[must_use]
    pub fn get_active_detectors(
        &self,
        config: &Config,
        presets: &[FrameworkPreset],
    ) -> HashSet<String> {
        let registry = DetectorRegistry::new();
        let (enabled_detectors, _) =
            registry.get_enabled_full(config, presets, self.args.all_detectors);

        let active_detectors = self.filter_detectors(enabled_detectors, |(id, _)| id);
        active_detectors.into_iter().map(|(id, _)| id).collect()
    }

    pub fn filter_detectors<T, F: Fn(&T) -> &str>(
        &self,
        detectors: impl IntoIterator<Item = T>,
        id_extractor: F,
    ) -> Vec<T> {
        let include = self.args.detectors.as_ref().map(|s| {
            s.split(',')
                .map(str::trim)
                .filter(|id| !id.is_empty())
                .map(std::string::ToString::to_string)
                .collect::<HashSet<_>>()
        });
        let exclude = self
            .args
            .exclude_detectors
            .as_ref()
            .map(|s| {
                s.split(',')
                    .map(str::trim)
                    .filter(|id| !id.is_empty())
                    .map(std::string::ToString::to_string)
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        detectors
            .into_iter()
            .filter(|d| match include.as_ref() {
                Some(set) => set.contains(id_extractor(d)),
                None => true,
            })
            .filter(|d| !exclude.contains(id_extractor(d)))
            .collect()
    }
}

pub fn apply_arg_overrides(args: &ScanArgs, config: &mut Config) {
    if let Some(ref detectors) = args.detectors {
        for id in detectors.split(',').map(str::trim) {
            override_rule(config, id, RuleSeverity::High, true);
        }
    }
    if let Some(ref exclude) = args.exclude_detectors {
        for id in exclude.split(',').map(str::trim) {
            override_rule(config, id, RuleSeverity::Off, false);
        }
    }

    if args.no_git {
        config.git.enabled = false;
    }
    if let Some(ref period) = args.git_history_period {
        config.git.history_period = period.clone();
    }

    if let Some(max_size) = args.max_file_size {
        config.max_file_size = max_size;
    }
}

fn override_rule(config: &mut Config, id: &str, severity: RuleSeverity, enabled: bool) {
    config
        .rules
        .entry(id.to_string())
        .and_modify(|rule| match rule {
            RuleConfig::Full(ref mut full) => {
                full.severity = Some(severity);
                full.enabled = Some(enabled);
            }
            RuleConfig::Short(_) => {
                *rule = RuleConfig::Short(severity);
            }
        })
        .or_insert(RuleConfig::Short(severity));
}
