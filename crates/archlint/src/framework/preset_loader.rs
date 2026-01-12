use super::preset_types::PresetYaml;
use super::presets::FrameworkPreset;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub struct PresetLoader;

use include_dir::{include_dir, Dir};

static PRESETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../presets");

impl PresetLoader {
    pub fn load_builtin(name: &str) -> Result<FrameworkPreset> {
        let content = Self::get_builtin_content(name)
            .ok_or_else(|| anyhow!("Built-in preset not found: {}", name))?;

        let yaml: PresetYaml = serde_yaml::from_str(content)
            .map_err(|e| anyhow!("Failed to parse built-in preset '{}': {}", name, e))?;
        Ok(Self::convert(yaml))
    }

    fn get_builtin_content(name: &str) -> Option<&'static str> {
        PRESETS_DIR
            .get_file(format!("{}.yaml", name))
            .or_else(|| PRESETS_DIR.get_file(format!("{}.yml", name)))
            .and_then(|f| f.contents_utf8())
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<FrameworkPreset> {
        let path_ref = path.as_ref();
        let content = fs::read_to_string(path_ref)
            .map_err(|e| anyhow!("Failed to read preset file '{:?}': {}", path_ref, e))?;
        let yaml: PresetYaml = serde_yaml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse preset file '{:?}': {}", path_ref, e))?;
        Ok(Self::convert(yaml))
    }

    pub fn load_url(url: &str) -> Result<FrameworkPreset> {
        let parsed_url =
            reqwest::Url::parse(url).map_err(|e| anyhow!("Invalid URL '{}': {}", url, e))?;

        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err(anyhow!(
                "Only http and https schemes are allowed for preset URLs"
            ));
        }

        if let Some(host) = parsed_url.host_str() {
            if Self::is_blocked_host(host) {
                return Err(anyhow!(
                    "Presets from local or private networks are not allowed for security reasons"
                ));
            }
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        let response = client
            .get(url)
            .send()
            .map_err(|e| anyhow!("Failed to fetch preset from URL '{}': {}", url, e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to load preset from URL: {}, status: {}",
                url,
                response.status()
            ));
        }

        // Check content length if available
        const MAX_PRESET_SIZE: u64 = 10 * 1024 * 1024; // 10MB
        if let Some(content_length) = response.content_length() {
            if content_length > MAX_PRESET_SIZE {
                return Err(anyhow!(
                    "Preset file from URL '{}' is too large: {} bytes (max: {} bytes)",
                    url,
                    content_length,
                    MAX_PRESET_SIZE
                ));
            }
        }

        use std::io::Read;
        let mut content = String::new();
        response
            .take(MAX_PRESET_SIZE + 1)
            .read_to_string(&mut content)
            .map_err(|e| anyhow!("Failed to read response body from URL '{}': {}", url, e))?;

        if content.len() > MAX_PRESET_SIZE as usize {
            return Err(anyhow!(
                "Preset file from URL '{}' exceeded size limit of {} bytes",
                url,
                MAX_PRESET_SIZE
            ));
        }

        let yaml: PresetYaml = serde_yaml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse preset from URL '{}': {}", url, e))?;
        Ok(Self::convert(yaml))
    }

    pub fn load_any(name_or_path_or_url: &str) -> Result<FrameworkPreset> {
        if name_or_path_or_url.starts_with("http://") || name_or_path_or_url.starts_with("https://")
        {
            Self::load_url(name_or_path_or_url)
        } else if Path::new(name_or_path_or_url).exists()
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

    fn is_blocked_host(host: &str) -> bool {
        if host == "localhost" {
            return true;
        }

        use std::net::IpAddr;
        if let Ok(ip) = host.parse::<IpAddr>() {
            return match ip {
                IpAddr::V4(v4) => {
                    v4.is_loopback()
                        || v4.is_private()
                        || v4.is_link_local()
                        || v4.is_unspecified()
                        || (v4.octets()[0] == 100 && (v4.octets()[1] & 0b1100_0000 == 64))
                    // CGNAT 100.64.0.0/10
                }
                IpAddr::V6(v6) => {
                    v6.is_loopback()
                        || v6.is_unspecified()
                        || (v6.segments()[0] & 0xffc0) == 0xfe80 // Link-local fe80::/10
                        || (v6.segments()[0] & 0xfe00) == 0xfc00 // ULA fc00::/7
                        || v6.to_ipv4_mapped().is_some_and(|v4| {
                            v4.is_loopback()
                                || v4.is_private()
                                || v4.is_link_local()
                                || v4.is_unspecified()
                        })
                }
            };
        }

        false
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
        let content = Self::get_builtin_content(name)?;
        serde_yaml::from_str(content).ok()
    }

    pub fn get_all_builtin_names() -> Vec<&'static str> {
        PRESETS_DIR
            .files()
            .filter_map(|f| {
                let name = f.path().file_stem()?.to_str()?;
                let ext = f.path().extension()?.to_str()?;
                if ext == "yaml" || ext == "yml" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
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
  custom-detector: high
"#;
        temp_file.write_all(yaml_content.as_bytes()).unwrap();

        let preset = PresetLoader::load_file(temp_file.path()).unwrap();
        assert_eq!(preset.name, "Custom");
        assert!(preset.rules.contains_key("custom-detector"));
    }

    #[test]
    fn test_is_blocked_host() {
        assert!(PresetLoader::is_blocked_host("localhost"));
        assert!(PresetLoader::is_blocked_host("127.0.0.1"));
        assert!(PresetLoader::is_blocked_host("::1"));
        assert!(PresetLoader::is_blocked_host("10.0.0.1"));
        assert!(PresetLoader::is_blocked_host("192.168.1.1"));
        assert!(PresetLoader::is_blocked_host("172.16.0.1"));
        assert!(PresetLoader::is_blocked_host("172.31.255.255"));
        assert!(PresetLoader::is_blocked_host("169.254.1.1"));
        assert!(PresetLoader::is_blocked_host("100.64.0.1"));
        assert!(PresetLoader::is_blocked_host("fe80::1"));
        assert!(PresetLoader::is_blocked_host("fc00::1"));
        assert!(PresetLoader::is_blocked_host("::ffff:127.0.0.1"));

        assert!(!PresetLoader::is_blocked_host("google.com"));
        assert!(!PresetLoader::is_blocked_host("8.8.8.8"));
        assert!(!PresetLoader::is_blocked_host("172.15.255.255"));
        assert!(!PresetLoader::is_blocked_host("172.32.0.1"));
    }
}
