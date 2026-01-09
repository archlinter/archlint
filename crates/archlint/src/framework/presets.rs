use super::Framework;
use crate::config::{Override, RuleConfig, RuleSeverity};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FrameworkPreset {
    pub name: &'static str,
    pub rules: HashMap<String, RuleConfig>,
    pub entry_points: Vec<String>,
    pub overrides: Vec<Override>,
}

pub fn get_presets(frameworks: &[Framework]) -> Vec<FrameworkPreset> {
    frameworks
        .iter()
        .filter_map(|&f| match f {
            Framework::NestJS => Some(nestjs_preset()),
            Framework::NextJS => Some(nextjs_preset()),
            Framework::React => Some(react_preset()),
            Framework::Oclif => Some(oclif_preset()),
            _ => None,
        })
        .collect()
}

fn nestjs_preset() -> FrameworkPreset {
    let mut rules = HashMap::new();

    // Default rules
    rules.insert(
        "layer_violation".to_string(),
        RuleConfig::Short(RuleSeverity::Error),
    );
    rules.insert(
        "module_cohesion".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );

    // Dead symbols with ignored methods
    let mut dead_symbols_opts = serde_yaml::Mapping::new();
    dead_symbols_opts.insert(
        serde_yaml::Value::String("ignore_methods".to_string()),
        serde_yaml::Value::Sequence(
            vec![
                "onModuleInit",
                "onApplicationBootstrap",
                "onModuleDestroy",
                "beforeApplicationShutdown",
                "onApplicationShutdown",
                "intercept",
                "transform",
                "canActivate",
                "resolve",
                "validate",
            ]
            .into_iter()
            .map(|s| serde_yaml::Value::String(s.to_string()))
            .collect(),
        ),
    );
    rules.insert(
        "dead_symbols".to_string(),
        RuleConfig::Full(crate::config::RuleFullConfig {
            severity: None,
            enabled: None,
            exclude: Vec::new(),
            options: serde_yaml::Value::Mapping(dead_symbols_opts),
        }),
    );

    // Vendor ignore via rules
    for rule_name in ["vendor_coupling", "hub_dependency"] {
        let mut opts = serde_yaml::Mapping::new();
        opts.insert(
            serde_yaml::Value::String("ignore_packages".to_string()),
            serde_yaml::Value::Sequence(
                vec![
                    "@nestjs/*",
                    "class-validator",
                    "class-transformer",
                    "rxjs",
                    "reflect-metadata",
                ]
                .into_iter()
                .map(|s| serde_yaml::Value::String(s.to_string()))
                .collect(),
            ),
        );
        rules.insert(
            rule_name.to_string(),
            RuleConfig::Full(crate::config::RuleFullConfig {
                severity: None,
                enabled: None,
                exclude: Vec::new(),
                options: serde_yaml::Value::Mapping(opts),
            }),
        );
    }

    let entry_points = vec![
        "**/*.controller.ts".to_string(),
        "**/*.controller.js".to_string(),
        "**/*.module.ts".to_string(),
        "**/*.module.js".to_string(),
    ];

    let mut overrides = Vec::new();

    // Controllers
    let mut controller_rules = HashMap::new();
    controller_rules.insert("lcom".to_string(), RuleConfig::Short(RuleSeverity::Off));
    controller_rules.insert(
        "sdp_violation".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );
    overrides.push(Override {
        files: vec![
            "**/*.controller.ts".to_string(),
            "**/*.controller.js".to_string(),
        ],
        rules: controller_rules,
    });

    // Modules
    let mut module_rules = HashMap::new();
    module_rules.insert(
        "high_coupling".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );
    module_rules.insert("lcom".to_string(), RuleConfig::Short(RuleSeverity::Off));
    module_rules.insert(
        "module_cohesion".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );
    module_rules.insert(
        "sdp_violation".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );
    overrides.push(Override {
        files: vec!["**/*.module.ts".to_string(), "**/*.module.js".to_string()],
        rules: module_rules,
    });

    // Entities, DTOs, etc. (simplified for hardcoded)
    let mut data_rules = HashMap::new();
    data_rules.insert(
        "abstractness".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );
    overrides.push(Override {
        files: vec![
            "**/*.dto.ts".to_string(),
            "**/*.entity.ts".to_string(),
            "**/dto/**".to_string(),
        ],
        rules: data_rules,
    });

    FrameworkPreset {
        name: "nestjs",
        rules,
        entry_points,
        overrides,
    }
}

fn nextjs_preset() -> FrameworkPreset {
    let mut rules = HashMap::new();
    rules.insert(
        "layer_violation".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );
    rules.insert(
        "barrel_file".to_string(),
        RuleConfig::Short(RuleSeverity::Off),
    );

    let entry_points = vec![
        "**/pages/**".to_string(),
        "**/app/**/page.tsx".to_string(),
        "**/app/**/route.ts".to_string(),
    ];

    FrameworkPreset {
        name: "nextjs",
        rules,
        entry_points,
        overrides: Vec::new(),
    }
}

fn react_preset() -> FrameworkPreset {
    let mut rules = HashMap::new();
    rules.insert("lcom".to_string(), RuleConfig::Short(RuleSeverity::Off));

    FrameworkPreset {
        name: "react",
        rules,
        entry_points: Vec::new(),
        overrides: Vec::new(),
    }
}

fn oclif_preset() -> FrameworkPreset {
    let mut rules = HashMap::new();
    let mut dead_symbols_opts = serde_yaml::Mapping::new();
    dead_symbols_opts.insert(
        serde_yaml::Value::String("ignore_methods".to_string()),
        serde_yaml::Value::Sequence(
            vec!["run", "init", "finally", "catch"]
                .into_iter()
                .map(|s| serde_yaml::Value::String(s.to_string()))
                .collect(),
        ),
    );
    rules.insert(
        "dead_symbols".to_string(),
        RuleConfig::Full(crate::config::RuleFullConfig {
            severity: None,
            enabled: None,
            exclude: Vec::new(),
            options: serde_yaml::Value::Mapping(dead_symbols_opts),
        }),
    );

    FrameworkPreset {
        name: "oclif",
        rules,
        entry_points: vec!["**/src/commands/**".to_string()],
        overrides: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_presets_known_frameworks() {
        let frameworks = vec![
            Framework::NestJS,
            Framework::NextJS,
            Framework::React,
            Framework::Oclif,
        ];
        let presets = get_presets(&frameworks);
        assert_eq!(presets.len(), 4);
        assert_eq!(presets[0].name, "nestjs");
    }
}
