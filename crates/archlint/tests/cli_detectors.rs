//! Detector tests that drive the real CLI.
//!
//! The shared `common::analyze_fixture` harness builds an `AnalysisContext` by hand
//! and puts *every* scanned file into the dependency graph. The production runner
//! does not — it drops files without runtime code. Detectors that silently never
//! fire in production can therefore stay green under the harness, so the cases
//! below go through the binary instead.

use assert_cmd::cargo_bin;
use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

/// Creates a project directory with the given `(relative path, contents)` files.
fn project_with(files: &[(&str, &str)]) -> (TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    let root = dir.path().to_path_buf();
    fs::write(
        root.join("package.json"),
        r#"{"name":"fixture","version":"1.0.0"}"#,
    )
    .unwrap();

    for (relative, contents) in files {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    (dir, root)
}

/// Runs a scan and returns the `type` of every reported smell.
///
/// Reads the `smells` array rather than the raw stdout: the `summary` object always
/// carries a zeroed counter for several detectors, so a substring check against the
/// whole document passes whether or not anything was actually detected.
fn scanned_smell_types(project_path: &Path) -> Vec<String> {
    let output = Command::new(cargo_bin!("archlint"))
        .arg("scan")
        .arg(project_path)
        .args(["--format", "json", "--no-cache", "-q"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let report: serde_json::Value = serde_json::from_slice(&output).unwrap();

    report["smells"]
        .as_array()
        .expect("scan report must contain a smells array")
        .iter()
        .map(|smell| smell["type"].as_str().unwrap_or_default().to_string())
        .collect()
}

fn has_smell(project_path: &Path, needle: &str) -> bool {
    scanned_smell_types(project_path)
        .iter()
        .any(|smell_type| smell_type.contains(needle))
}

/// A function whose cyclomatic complexity is well past the default threshold of 15.
fn complex_function() -> String {
    use std::fmt::Write;

    let mut source = "export function big(a: number): number {\n".to_string();
    for i in 0..20 {
        writeln!(source, "  if (a === {i}) return {i};").unwrap();
    }
    source.push_str("  return 0;\n}\n");
    source
}

#[test]
fn test_cyclomatic_complexity_honours_rule_exclude() {
    let (_dir, project_path) = project_with(&[
        ("src/legacy/big.ts", &complex_function()),
        (
            ".archlint.yaml",
            "rules:\n  cyclomatic_complexity:\n    enabled: true\n    exclude: [\"**/legacy/**\"]\n",
        ),
    ]);

    assert!(
        !has_smell(&project_path, "high_cyclomatic_complexity"),
        "an excluded file must not be reported"
    );
}

#[test]
fn test_cyclomatic_complexity_honours_path_override() {
    let (_dir, project_path) = project_with(&[
        ("src/legacy/big.ts", &complex_function()),
        (
            ".archlint.yaml",
            "rules:\n  cyclomatic_complexity:\n    enabled: true\noverrides:\n  - files: [\"**/legacy/**\"]\n    rules:\n      cyclomatic_complexity: off\n",
        ),
    ]);

    assert!(
        !has_smell(&project_path, "high_cyclomatic_complexity"),
        "a path override switching the rule off must be honoured"
    );
}

#[test]
fn test_legacy_complexity_rule_name_still_configures_the_detector() {
    let (_dir, project_path) = project_with(&[
        ("src/legacy/big.ts", &complex_function()),
        (
            ".archlint.yaml",
            "rules:\n  complexity:\n    enabled: true\n    exclude: [\"**/legacy/**\"]\n",
        ),
    ]);

    assert!(
        !has_smell(&project_path, "high_cyclomatic_complexity"),
        "the legacy `complexity` rule name must still configure the detector"
    );
}

fn git(root: &Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?} failed");
}

/// Initialises a repository and records `commits` revisions, each touching every file.
fn git_history(root: &Path, files: &[&str], commits: usize) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "fixture@example.com"]);
    git(root, &["config", "user.name", "fixture"]);
    git(root, &["config", "commit.gpgsign", "false"]);

    for revision in 0..commits {
        for file in files {
            let path = root.join(file);
            let previous = fs::read_to_string(&path).unwrap_or_default();
            fs::write(
                path,
                format!("{previous}export const v{revision} = {revision};\n"),
            )
            .unwrap();
        }
        git(root, &["add", "-A"]);
        git(root, &["commit", "-q", "-m", "change"]);
    }
}

#[test]
fn test_shotgun_surgery_detected_from_git_history() {
    let files = ["src/a.ts", "src/b.ts", "src/c.ts", "src/d.ts"];
    let (_dir, project_path) = project_with(&[
        ("src/a.ts", ""),
        ("src/b.ts", ""),
        ("src/c.ts", ""),
        ("src/d.ts", ""),
        (
            ".archlint.yaml",
            "rules:\n  shotgun_surgery:\n    enabled: true\n",
        ),
    ]);

    // Six commits touching all four files: 6 co-changing commits per file, three
    // co-changed files each — past the default thresholds of 5 and 3.
    git_history(&project_path, &files, 6);

    assert!(
        has_smell(&project_path, "shotgun_surgery"),
        "files that always change together must be reported"
    );
}

#[test]
fn test_circular_type_deps_detected_between_type_only_files() {
    // Neither file has runtime code, which is the normal shape of a `types.ts`
    // pair — and exactly the case this detector exists for.
    let (_dir, project_path) = project_with(&[
        (
            "src/a.ts",
            "import type { B } from './b';\nexport type A = { b: B };\n",
        ),
        (
            "src/b.ts",
            "import type { A } from './a';\nexport type B = { a: A };\n",
        ),
        (
            ".archlint.yaml",
            "rules:\n  circular_type_deps:\n    enabled: true\n",
        ),
    ]);

    assert!(
        has_smell(&project_path, "CircularTypeDependency"),
        "type-only cycle between type-only files must be reported"
    );
}

#[test]
fn test_circular_type_deps_ignores_value_imports() {
    let (_dir, project_path) = project_with(&[
        (
            "src/a.ts",
            "import { b } from './b';\nexport const a = () => b;\n",
        ),
        (
            "src/b.ts",
            "import { a } from './a';\nexport const b = () => a;\n",
        ),
        (
            ".archlint.yaml",
            "rules:\n  circular_type_deps:\n    enabled: true\n",
        ),
    ]);

    assert!(
        !has_smell(&project_path, "CircularTypeDependency"),
        "a runtime cycle is not a type-only cycle"
    );
}
