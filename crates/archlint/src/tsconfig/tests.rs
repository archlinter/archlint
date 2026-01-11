use super::*;
use tempfile::tempdir;

#[test]
fn test_load_basic_tsconfig() -> Result<()> {
    let dir = tempdir()?;
    let tsconfig_path = dir.path().join("tsconfig.json");
    fs::write(
        &tsconfig_path,
        r#"{
            "compilerOptions": {
                "baseUrl": "./src",
                "paths": { "@app/*": ["app/*"] },
                "outDir": "dist"
            },
            "exclude": ["node_modules"]
        }"#,
    )?;

    let config = TsConfig::load(&tsconfig_path)?;
    let opts = config.compiler_options.unwrap();
    assert_eq!(opts.base_url.unwrap(), "./src");
    assert_eq!(opts.out_dir.unwrap(), "dist");
    assert_eq!(opts.paths.unwrap().get("@app/*").unwrap()[0], "app/*");
    assert_eq!(config.exclude, vec!["node_modules"]);

    Ok(())
}

#[test]
fn test_load_with_comments() -> Result<()> {
    let dir = tempdir()?;
    let tsconfig_path = dir.path().join("tsconfig.json");
    fs::write(
        &tsconfig_path,
        r#"{
            // This is a comment
            "compilerOptions": {
                /* Multi-line comment */
                "baseUrl": "."
            }
        }"#,
    )?;

    let config = TsConfig::load(&tsconfig_path)?;
    assert_eq!(config.compiler_options.unwrap().base_url.unwrap(), ".");
    Ok(())
}

#[test]
fn test_extends_and_merge() -> Result<()> {
    let dir = tempdir()?;

    fs::write(
        dir.path().join("tsconfig.base.json"),
        r#"{
            "compilerOptions": {
                "baseUrl": ".",
                "paths": {
                    "@base/*": ["base/*"],
                    "@overridden/*": ["base-overridden/*"]
                }
            },
            "exclude": ["node_modules", "dist"]
        }"#,
    )?;

    let tsconfig_path = dir.path().join("tsconfig.json");
    fs::write(
        &tsconfig_path,
        r#"{
            "extends": "./tsconfig.base.json",
            "compilerOptions": {
                "paths": {
                    "@app/*": ["app/*"],
                    "@overridden/*": ["app-overridden/*"]
                }
            },
            "exclude": ["custom-exclude"]
        }"#,
    )?;

    let config = TsConfig::load(&tsconfig_path)?;
    let opts = config.compiler_options.unwrap();
    let paths = opts.paths.unwrap();

    // Check merged paths
    assert_eq!(paths.get("@base/*").unwrap()[0], "base/*");
    assert_eq!(paths.get("@app/*").unwrap()[0], "app/*");
    assert_eq!(paths.get("@overridden/*").unwrap()[0], "app-overridden/*");

    // Check merged excludes
    assert!(config.exclude.contains(&"node_modules".to_string()));
    assert!(config.exclude.contains(&"dist".to_string()));
    assert!(config.exclude.contains(&"custom-exclude".to_string()));

    Ok(())
}

#[test]
fn test_find_and_load() -> Result<()> {
    let dir = tempdir()?;

    // Should return None if no tsconfig.json exists
    assert!(TsConfig::find_and_load(dir.path(), None)?.is_none());

    // Should find tsconfig.json when it exists
    fs::write(
        dir.path().join("tsconfig.json"),
        r#"{"compilerOptions": {"baseUrl": "."}}"#,
    )?;

    let config = TsConfig::find_and_load(dir.path(), None)?.unwrap();
    assert_eq!(config.compiler_options.unwrap().base_url.unwrap(), ".");

    // Explicit path should take precedence
    fs::write(
        dir.path().join("tsconfig.build.json"),
        r#"{"compilerOptions": {"baseUrl": "build"}}"#,
    )?;

    let config = TsConfig::find_and_load(dir.path(), Some("tsconfig.build.json"))?.unwrap();
    assert_eq!(config.compiler_options.unwrap().base_url.unwrap(), "build");

    Ok(())
}

#[test]
fn test_circular_extends() -> Result<()> {
    let dir = tempdir()?;
    let path1 = dir.path().join("tsconfig.1.json");
    let path2 = dir.path().join("tsconfig.2.json");

    fs::write(&path1, r#"{"extends": "./tsconfig.2.json"}"#)?;
    fs::write(&path2, r#"{"extends": "./tsconfig.1.json"}"#)?;

    let result = TsConfig::load(&path1);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Circular extends"));

    Ok(())
}

#[test]
fn test_invalid_json5() -> Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("tsconfig.json");
    fs::write(&path, r#"{"compilerOptions": { "baseUrl": "." "#)?; // Missing closing braces

    let result = TsConfig::load(&path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_non_existent_extends() -> Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("tsconfig.json");
    fs::write(&path, r#"{"extends": "./non-existent.json"}"#)?;

    // Should error if it doesn't exist
    let result = TsConfig::load(&path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_missing_parent_directory() -> Result<()> {
    let dir = tempdir()?;
    let tsconfig_path = dir.path().join("tsconfig.json");
    fs::write(&tsconfig_path, r#"{"extends": "./base.json"}"#)?;

    // Should error if base.json doesn't exist
    let result = TsConfig::load(&tsconfig_path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_extends_from_node_modules() -> Result<()> {
    let dir = tempdir()?;
    let project_root = dir.path();

    let node_modules = project_root.join("node_modules");
    let pkg_dir = node_modules.join("@my-org/config");
    fs::create_dir_all(&pkg_dir)?;

    fs::write(
        pkg_dir.join("base.json"),
        r#"{"compilerOptions": {"baseUrl": "from-pkg"}}"#,
    )?;

    let tsconfig_path = project_root.join("tsconfig.json");
    fs::write(&tsconfig_path, r#"{"extends": "@my-org/config/base.json"}"#)?;

    let config = TsConfig::load(&tsconfig_path)?;
    assert_eq!(
        config.compiler_options.unwrap().base_url.unwrap(),
        "from-pkg"
    );

    Ok(())
}
