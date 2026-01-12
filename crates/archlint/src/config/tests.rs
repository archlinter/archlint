use super::*;
use tempfile::tempdir;

#[test]
fn test_deserialize_extends_single_string() {
    let yaml = "extends: nestjs";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.extends, vec!["nestjs"]);
}

#[test]
fn test_deserialize_extends_list() {
    let yaml = "extends:\n  - nestjs\n  - react";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.extends, vec!["nestjs", "react"]);
}

#[test]
fn test_deserialize_extends_missing() {
    let yaml = "{}";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert!(config.extends.is_empty());
}

#[test]
fn test_deserialize_extends_null() {
    let yaml = "extends: null";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert!(config.extends.is_empty());
}

#[test]
fn test_tsconfig_disabled_boolean() {
    let yaml = "tsconfig: false";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert!(matches!(
        config.tsconfig,
        Some(TsConfigConfig::Boolean(false))
    ));
}

#[test]
fn test_tsconfig_disabled_null() {
    let yaml = "tsconfig: null";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    assert!(config.tsconfig.is_none());
}

#[test]
fn test_tsconfig_path() {
    let yaml = "tsconfig: ./custom.json";
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    if let Some(TsConfigConfig::Path(p)) = &config.tsconfig {
        assert_eq!(p, "./custom.json");
    } else {
        panic!("Expected TsConfigConfig::Path");
    }
}

#[test]
fn test_enrich_from_tsconfig() -> Result<()> {
    let dir = tempdir()?;
    fs::write(
        dir.path().join("tsconfig.json"),
        r#"{
            "compilerOptions": {
                "baseUrl": "./src",
                "paths": {
                    "@app/*": ["app/*"],
                    "@shared/*": ["shared/*"]
                },
                "outDir": "dist"
            },
            "exclude": ["temp"]
        }"#,
    )?;

    let mut config = Config::default();
    config
        .aliases
        .insert("@app/*".to_string(), "custom/app/*".to_string());

    config.enrich_from_tsconfig(dir.path())?;

    // @app/* should be kept from config (priority)
    assert_eq!(config.aliases.get("@app/*").unwrap(), "custom/app/*");
    // @shared/* should be loaded from tsconfig
    assert_eq!(config.aliases.get("@shared/*").unwrap(), "./src/shared/*");

    // dist and temp should be in ignore
    assert!(config.ignore.contains(&"**/dist/**".to_string()));
    assert!(config.ignore.contains(&"**/temp/**".to_string()));

    Ok(())
}

#[test]
fn test_enrich_from_tsconfig_with_extends() -> Result<()> {
    let dir = tempdir()?;

    // Create base tsconfig
    fs::write(
        dir.path().join("tsconfig.base.json"),
        r#"{
            "compilerOptions": {
                "baseUrl": ".",
                "paths": {
                    "@core/*": ["core/*"],
                    "@utils/*": ["utils/*"]
                },
                "outDir": "build"
            },
            "exclude": ["node_modules"]
        }"#,
    )?;

    // Create main tsconfig that extends base
    fs::write(
        dir.path().join("tsconfig.json"),
        r#"{
            "extends": "./tsconfig.base.json",
            "compilerOptions": {
                "paths": {
                    "@app/*": ["src/app/*"]
                }
            },
            "exclude": ["tmp"]
        }"#,
    )?;

    let mut config = Config::default();
    config.enrich_from_tsconfig(dir.path())?;

    // Should have paths from both base and main tsconfig
    assert_eq!(config.aliases.get("@core/*").unwrap(), "./core/*");
    assert_eq!(config.aliases.get("@utils/*").unwrap(), "./utils/*");
    assert_eq!(config.aliases.get("@app/*").unwrap(), "./src/app/*");

    // Should have excludes from both
    assert!(config.ignore.contains(&"**/node_modules/**".to_string()));
    assert!(config.ignore.contains(&"**/tmp/**".to_string()));
    assert!(config.ignore.contains(&"**/build/**".to_string()));

    Ok(())
}

#[test]
fn test_enrich_from_tsconfig_real_project_structure() -> Result<()> {
    let dir = tempdir()?;

    // Simulate real project structure: packages/plugin-api/tsconfig.json extends ../../tsconfig.base.json
    fs::create_dir_all(dir.path().join("packages/plugin-api"))?;

    // Create base tsconfig at root
    fs::write(
        dir.path().join("tsconfig.base.json"),
        r#"{
            "compilerOptions": {
                "target": "ES2022",
                "strict": true,
                "outDir": "dist",
                "baseUrl": ".",
                "paths": {
                    "@shared/*": ["shared/*"]
                }
            },
            "exclude": ["node_modules", "dist"]
        }"#,
    )?;

    // Create package tsconfig that extends base with relative path
    fs::write(
        dir.path().join("packages/plugin-api/tsconfig.json"),
        r#"{
            "extends": "../../tsconfig.base.json",
            "compilerOptions": {
                "rootDir": "src",
                "paths": {
                    "@plugin/*": ["src/plugin/*"]
                }
            }
        }"#,
    )?;

    // Load config from package directory
    let mut config = Config::default();
    let package_dir = dir.path().join("packages/plugin-api");
    config.enrich_from_tsconfig(&package_dir)?;

    // Should have paths from both root base and package tsconfig
    assert_eq!(config.aliases.get("@shared/*").unwrap(), "./shared/*");
    assert_eq!(config.aliases.get("@plugin/*").unwrap(), "./src/plugin/*");

    // Should have excludes from base
    assert!(config.ignore.contains(&"**/node_modules/**".to_string()));
    assert!(config.ignore.contains(&"**/dist/**".to_string()));

    Ok(())
}
