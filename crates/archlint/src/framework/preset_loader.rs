use super::preset_types::PresetYaml;
use super::presets::FrameworkPreset;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub struct PresetLoader;

impl PresetLoader {
    pub fn load_builtin(name: &str) -> Result<FrameworkPreset> {
        let content = match name {
            "nestjs" => include_str!("../../../../presets/nestjs.yaml"),
            "nextjs" => include_str!("../../../../presets/nextjs.yaml"),
            "react" => include_str!("../../../../presets/react.yaml"),
            "oclif" => include_str!("../../../../presets/oclif.yaml"),
            _ => return Err(anyhow!("Built-in preset not found: {}", name)),
        };

        let yaml: PresetYaml = serde_yaml::from_str(content)?;
        Ok(Self::convert(yaml))
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<FrameworkPreset> {
        let content = fs::read_to_string(path)?;
        let yaml: PresetYaml = serde_yaml::from_str(&content)?;
        Ok(Self::convert(yaml))
    }

    pub fn load_url(url: &str) -> Result<FrameworkPreset> {
        let response = reqwest::blocking::get(url)?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to load preset from URL: {}, status: {}",
                url,
                response.status()
            ));
        }
        let content = response.text()?;
        let yaml: PresetYaml = serde_yaml::from_str(&content)?;
        Ok(Self::convert(yaml))
    }

    pub fn load_any(name_or_path_or_url: &str) -> Result<FrameworkPreset> {
        if name_or_path_or_url.starts_with("http://") || name_or_path_or_url.starts_with("https://")
        {
            Self::load_url(name_or_path_or_url)
        } else if std::path::Path::new(name_or_path_or_url).exists()
            || name_or_path_or_url.contains('/')
            || name_or_path_or_url.contains('\\')
            || name_or_path_or_url.ends_with(".yaml")
            || name_or_path_or_url.ends_with(".yml")
        {
            Self::load_file(name_or_path_or_url)
        } else {
            Self::load_builtin(name_or_path_or_url)
        }
    }

    fn convert(yaml: PresetYaml) -> FrameworkPreset {
        FrameworkPreset {
            name: yaml.name,
            rules: yaml.rules,
            entry_points: yaml.entry_points,
            overrides: yaml.overrides,
        }
    }

    pub fn get_builtin_yaml(name: &str) -> Option<PresetYaml> {
        let content = match name {
            "nestjs" => Some(include_str!("../../../../presets/nestjs.yaml")),
            "nextjs" => Some(include_str!("../../../../presets/nextjs.yaml")),
            "react" => Some(include_str!("../../../../presets/react.yaml")),
            "oclif" => Some(include_str!("../../../../presets/oclif.yaml")),
            _ => None,
        }?;
        serde_yaml::from_str(content).ok()
    }

    pub fn get_all_builtin_names() -> Vec<&'static str> {
        vec!["nestjs", "nextjs", "react", "oclif"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_builtin_nestjs() {
        let preset = PresetLoader::load_builtin("nestjs").unwrap();
        assert_eq!(preset.name, "nestjs");
        assert!(preset.rules.contains_key("layer_violation"));
    }

    #[test]
    fn test_load_builtin_not_found() {
        let result = PresetLoader::load_builtin("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_builtin_yaml() {
        let yaml = PresetLoader::get_builtin_yaml("nestjs").unwrap();
        assert_eq!(yaml.name, "nestjs");
        assert_eq!(yaml.version, 1);
    }

    #[test]
    fn test_load_file() {
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        let yaml_content = r#"
name: Custom
version: 1
detect:
  packages:
    any_of: ["custom-pkg"]
rules:
  custom-detector: error
"#;
        temp_file.write_all(yaml_content.as_bytes()).unwrap();

        let preset = PresetLoader::load_file(temp_file.path()).unwrap();
        assert_eq!(preset.name, "Custom");
        assert!(preset.rules.contains_key("custom-detector"));
    }
}
