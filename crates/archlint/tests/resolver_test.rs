use archlint::config::Config;
use archlint::resolver::PathResolver;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_resolve_relative_import() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    // Create test files
    fs::write(root.join("foo.ts"), "").unwrap();
    fs::write(root.join("bar.ts"), "").unwrap();

    let resolver = PathResolver::new(root, &Config::default());
    let result = resolver.resolve("./bar", &root.join("foo.ts")).unwrap();

    assert!(result.is_some());
    assert_eq!(result.unwrap().file_name().unwrap(), "bar.ts");
}

#[test]
fn test_resolve_index_file() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    // Create directory and index file
    let dir = root.join("utils");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("index.ts"), "").unwrap();
    fs::write(root.join("main.ts"), "").unwrap();

    let resolver = PathResolver::new(root, &Config::default());
    let result = resolver.resolve("./utils", &root.join("main.ts")).unwrap();

    assert!(result.is_some());
    assert_eq!(result.unwrap().file_name().unwrap(), "index.ts");
}

#[test]
fn test_resolve_alias() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    // Create directory and file
    let src = root.join("src");
    fs::create_dir(&src).unwrap();
    let components = src.join("components");
    fs::create_dir(&components).unwrap();
    fs::write(components.join("Button.tsx"), "").unwrap();

    let mut config = Config::default();
    config
        .aliases
        .insert("@components/*".to_string(), "src/components/*".to_string());

    let resolver = PathResolver::new(root, &config);
    let result = resolver
        .resolve("@components/Button", &root.join("main.ts"))
        .unwrap();

    assert!(result.is_some());
    assert_eq!(result.unwrap().file_name().unwrap(), "Button.tsx");
}

#[test]
fn test_resolve_with_extensions() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    // Create files with different extensions
    fs::write(root.join("script.js"), "").unwrap();
    fs::write(root.join("style.jsx"), "").unwrap();

    let resolver = PathResolver::new(root, &Config::default());

    let result_js = resolver.resolve("./script", &root.join("main.ts")).unwrap();
    assert!(result_js.is_some());
    assert_eq!(result_js.unwrap().extension().unwrap(), "js");

    let result_jsx = resolver.resolve("./style", &root.join("main.ts")).unwrap();
    assert!(result_jsx.is_some());
    assert_eq!(result_jsx.unwrap().extension().unwrap(), "jsx");
}
