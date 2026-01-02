use archlint::glob_expand::expand_glob;
use archlint::project_root::detect_project_root;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_integration_root_detection_scenarios() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();

    // 1. Project with package.json
    let project_a = root.join("project-a");
    fs::create_dir_all(project_a.join("src/components")).unwrap();
    fs::write(project_a.join("package.json"), "{}").unwrap();
    let file_a = project_a.join("src/components/Button.ts");
    fs::write(&file_a, "").unwrap();

    assert_eq!(detect_project_root(&file_a), project_a);
    assert_eq!(detect_project_root(&project_a.join("src")), project_a);

    // 2. Nested project with .git
    let project_b = root.join("project-b");
    fs::create_dir_all(project_b.join(".git")).unwrap();
    fs::create_dir_all(project_b.join("packages/core/src")).unwrap();
    let file_b = project_b.join("packages/core/src/index.ts");
    fs::write(&file_b, "").unwrap();

    assert_eq!(detect_project_root(&file_b), project_b);
}

#[test]
fn test_integration_glob_expansion_and_root() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();

    // Setup project structure
    let project = root.join("my-app");
    fs::create_dir_all(project.join("src/api")).unwrap();
    fs::create_dir_all(project.join("src/utils")).unwrap();
    fs::write(project.join("package.json"), "{}").unwrap();

    let file1 = project.join("src/api/user.ts");
    let file2 = project.join("src/utils/helper.ts");
    let ignored = project.join("src/utils/test.spec.ts");

    fs::write(&file1, "").unwrap();
    fs::write(&file2, "").unwrap();
    fs::write(&ignored, "").unwrap();

    // Test glob expansion from inside the project
    // Note: expand_glob uses glob crate which works relative to CWD if path is relative.
    // For testing, we'll use absolute patterns or change CWD.

    let pattern = project.join("src/**/*.ts").to_string_lossy().into_owned();
    let expansion = expand_glob(&pattern, &["ts"]).unwrap();

    // Check that we found the files
    assert!(expansion.files.contains(&file1.canonicalize().unwrap()));
    assert!(expansion.files.contains(&file2.canonicalize().unwrap()));
    assert!(expansion.files.contains(&ignored.canonicalize().unwrap()));

    // Check base path extraction for root detection
    // For "/absolute/path/src/**/*.ts", base path should be "/absolute/path/src/"
    assert!(expansion.base_path.to_string_lossy().contains("my-app/src"));

    // Verify that project root can be detected from expansion.base_path
    let detected_root = detect_project_root(&expansion.base_path);
    assert_eq!(detected_root, project);
}

#[test]
fn test_glob_with_non_existent_base() {
    let dir = tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();

    let pattern = root
        .join("non-existent/**/*.ts")
        .to_string_lossy()
        .into_owned();
    let expansion = expand_glob(&pattern, &["ts"]).unwrap();

    assert!(expansion.files.is_empty());
    // Base path should still be extracted as much as possible
    assert!(expansion
        .base_path
        .to_string_lossy()
        .contains("non-existent"));
}
