use assert_cmd::cargo_bin;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::{tempdir, TempDir};

fn setup_test_project() -> (TempDir, std::path::PathBuf) {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;").unwrap();
    let project_path = dir.path().to_path_buf();
    (dir, project_path)
}

fn run_archlint_snapshot(project_path: &std::path::Path, output_path: &std::path::Path) {
    let mut cmd = Command::new(cargo_bin!("archlint"));
    cmd.arg("snapshot")
        .arg("-o")
        .arg(output_path)
        .arg("-p")
        .arg(project_path)
        .assert()
        .success();
}

#[test]
fn test_snapshot_command() {
    let (dir, project_path) = setup_test_project();
    let output_path = dir.path().join("snapshot.json");

    run_archlint_snapshot(&project_path, &output_path);

    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("schemaVersion"));
}

#[test]
fn test_diff_command_no_regressions() {
    let (_dir, project_path) = setup_test_project();

    // Create baseline
    let baseline = project_path.join("baseline.json");
    run_archlint_snapshot(&project_path, &baseline);

    // Diff (no changes)
    let mut cmd2 = Command::new(cargo_bin!("archlint"));
    cmd2.arg("diff")
        .arg(&baseline)
        .arg("-p")
        .arg(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("No architectural changes"));
}

#[test]
fn test_diff_command_with_regression_exits_1() {
    let (dir, project_path) = setup_test_project();

    // Baseline: no cycle
    fs::write(dir.path().join("src/a.ts"), "export const a = 1;").unwrap();
    fs::write(dir.path().join("src/b.ts"), "export const b = 2;").unwrap();

    let baseline = project_path.join("baseline.json");
    run_archlint_snapshot(&project_path, &baseline);

    // Add cycle
    fs::write(
        dir.path().join("src/a.ts"),
        "import { b } from './b'; export const a = b;",
    )
    .unwrap();
    fs::write(
        dir.path().join("src/b.ts"),
        "import { a } from './a'; export const b = a;",
    )
    .unwrap();

    // Diff should exit 1
    let mut cmd2 = Command::new(cargo_bin!("archlint"));
    cmd2.arg("diff")
        .arg(&baseline)
        .arg("-p")
        .arg(&project_path)
        .assert()
        .code(1)
        .stdout(predicate::str::contains("REGRESSIONS"));
}

#[test]
fn test_diff_json_output() {
    let (_dir, project_path) = setup_test_project();

    let baseline = project_path.join("baseline.json");
    run_archlint_snapshot(&project_path, &baseline);

    let mut cmd2 = Command::new(cargo_bin!("archlint"));
    cmd2.arg("diff")
        .arg(&baseline)
        .arg("--json")
        .arg("-p")
        .arg(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("hasRegressions"));
}
