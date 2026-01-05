use super::{FileType, Framework};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FrameworkPreset {
    pub name: &'static str,
    pub enabled_detectors: Vec<&'static str>,
    pub disabled_detectors: Vec<&'static str>,
    pub file_rules: HashMap<FileType, FileRules>,
    pub vendor_ignore: Vec<String>,
    pub ignore_methods: Vec<&'static str>,
}

#[derive(Debug, Clone, Default)]
pub struct FileRules {
    pub skip_detectors: Vec<&'static str>,
    pub is_entry_point: bool,
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
    let mut file_rules = HashMap::new();

    file_rules.insert(
        FileType::Controller,
        FileRules {
            skip_detectors: vec!["lcom", "sdp_violation"],
            is_entry_point: true,
        },
    );
    file_rules.insert(
        FileType::Module,
        FileRules {
            skip_detectors: vec!["high_coupling", "lcom", "scattered_module", "sdp_violation"],
            is_entry_point: true,
        },
    );
    file_rules.insert(
        FileType::Entity,
        FileRules {
            skip_detectors: vec!["cycles", "lcom", "abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::DTO,
        FileRules {
            skip_detectors: vec!["abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Interface,
        FileRules {
            skip_detectors: vec!["abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Config,
        FileRules {
            skip_detectors: vec!["abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Event,
        FileRules {
            skip_detectors: vec!["abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Exception,
        FileRules {
            skip_detectors: vec!["abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Guard,
        FileRules {
            skip_detectors: vec!["lcom", "abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Pipe,
        FileRules {
            skip_detectors: vec!["lcom", "abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Interceptor,
        FileRules {
            skip_detectors: vec!["lcom", "abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Decorator,
        FileRules {
            skip_detectors: vec!["abstractness_violation"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Middleware,
        FileRules {
            skip_detectors: vec!["lcom"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Repository,
        FileRules {
            skip_detectors: vec!["lcom", "high_coupling"],
            is_entry_point: false,
        },
    );

    FrameworkPreset {
        name: "NestJS",
        enabled_detectors: vec!["layer_violation", "sdp_violation"],
        disabled_detectors: vec!["scattered_module"],
        file_rules,
        vendor_ignore: vec![
            "@nestjs/*".to_string(),
            "class-validator".to_string(),
            "class-transformer".to_string(),
            "typeorm".to_string(),
            "@mikro-orm/*".to_string(),
            "rxjs".to_string(),
            "fastify".to_string(),
            "@fastify/*".to_string(),
            "reflect-metadata".to_string(),
        ],
        ignore_methods: vec![
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
        ],
    }
}

fn nextjs_preset() -> FrameworkPreset {
    let mut file_rules = HashMap::new();

    file_rules.insert(
        FileType::Page,
        FileRules {
            skip_detectors: vec!["lcom", "high_coupling"],
            is_entry_point: true,
        },
    );
    file_rules.insert(
        FileType::ApiRoute,
        FileRules {
            skip_detectors: vec!["lcom"],
            is_entry_point: true,
        },
    );

    FrameworkPreset {
        name: "Next.js",
        enabled_detectors: vec![],
        disabled_detectors: vec!["layer_violation", "barrel_file_abuse"],
        file_rules,
        vendor_ignore: vec!["next/*".to_string()],
        ignore_methods: vec!["getServerSideProps", "getStaticProps", "getStaticPaths"],
    }
}

fn react_preset() -> FrameworkPreset {
    let mut file_rules = HashMap::new();

    file_rules.insert(
        FileType::Component,
        FileRules {
            skip_detectors: vec!["abstractness_violation", "lcom"],
            is_entry_point: false,
        },
    );
    file_rules.insert(
        FileType::Hook,
        FileRules {
            skip_detectors: vec!["lcom"],
            is_entry_point: false,
        },
    );

    FrameworkPreset {
        name: "React",
        enabled_detectors: vec![],
        disabled_detectors: vec!["lcom", "scattered_module", "layer_violation"],
        file_rules,
        vendor_ignore: vec!["react/*".to_string()],
        ignore_methods: vec![
            "render",
            "componentDidMount",
            "componentDidUpdate",
            "componentWillUnmount",
            "shouldComponentUpdate",
        ],
    }
}

fn oclif_preset() -> FrameworkPreset {
    let mut file_rules = HashMap::new();

    file_rules.insert(
        FileType::CliCommand,
        FileRules {
            skip_detectors: vec!["lcom", "abstractness_violation"],
            is_entry_point: true,
        },
    );
    file_rules.insert(
        FileType::CliHook,
        FileRules {
            skip_detectors: vec!["lcom"],
            is_entry_point: true,
        },
    );

    FrameworkPreset {
        name: "oclif",
        enabled_detectors: vec![],
        disabled_detectors: vec![],
        file_rules,
        vendor_ignore: vec!["@oclif/*".to_string()],
        ignore_methods: vec!["run", "init", "finally", "catch"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_presets_empty() {
        let presets = get_presets(&[]);
        assert!(presets.is_empty());
    }

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
        assert_eq!(presets[0].name, "NestJS");
        assert_eq!(presets[1].name, "Next.js");
        assert_eq!(presets[2].name, "React");
        assert_eq!(presets[3].name, "oclif");
    }

    #[test]
    fn test_nestjs_preset_rules() {
        let preset = nestjs_preset();
        let controller_rules = preset.file_rules.get(&FileType::Controller).unwrap();
        assert!(controller_rules.is_entry_point);
        assert!(controller_rules.skip_detectors.contains(&"lcom"));

        let entity_rules = preset.file_rules.get(&FileType::Entity).unwrap();
        assert!(!entity_rules.is_entry_point);
        assert!(entity_rules.skip_detectors.contains(&"cycles"));
    }

    #[test]
    fn test_nextjs_preset_rules() {
        let preset = nextjs_preset();
        let page_rules = preset.file_rules.get(&FileType::Page).unwrap();
        assert!(page_rules.is_entry_point);
        assert!(page_rules.skip_detectors.contains(&"lcom"));
        assert!(preset.disabled_detectors.contains(&"layer_violation"));
    }

    #[test]
    fn test_get_presets_unsupported() {
        // Express, Angular, Vue, TypeORM, Prisma are currently not mapped to presets in get_presets
        let frameworks = vec![Framework::Express, Framework::Angular];
        let presets = get_presets(&frameworks);
        assert!(presets.is_empty());
    }
}
