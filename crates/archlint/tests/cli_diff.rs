use assert_cmd::cargo_bin;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_snapshot_command() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;").unwrap();

    let output_path = dir.path().join("snapshot.json");

    let mut cmd = Command::new(cargo_bin!("archlint"));
    cmd.arg("snapshot")
        .arg("-o")
        .arg(&output_path)
        .arg("-p")
        .arg(dir.path())
        .assert()
        .success();

    assert!(output_path.exists());

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("schemaVersion"));
}

#[test]
fn test_diff_command_no_regressions() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;").unwrap();

    // Create baseline
    let baseline = dir.path().join("baseline.json");
    let mut cmd1 = Command::new(cargo_bin!("archlint"));
    cmd1.arg("snapshot")
        .arg("-o")
        .arg(&baseline)
        .arg("-p")
        .arg(dir.path())
        .assert()
        .success();

    // Diff (no changes)
    let mut cmd2 = Command::new(cargo_bin!("archlint"));
    cmd2.arg("diff")
        .arg(&baseline)
        .arg("-p")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No architectural changes"));
}

#[test]
fn test_diff_command_with_regression_exits_1() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();

    // Baseline: no cycle
    fs::write(dir.path().join("src/a.ts"), "export const a = 1;").unwrap();
    fs::write(dir.path().join("src/b.ts"), "export const b = 2;").unwrap();

    let baseline = dir.path().join("baseline.json");
    let mut cmd1 = Command::new(cargo_bin!("archlint"));
    cmd1.arg("snapshot")
        .arg("-o")
        .arg(&baseline)
        .arg("-p")
        .arg(dir.path())
        .assert()
        .success();

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
        .arg(dir.path())
        .assert()
        .code(1)
        .stdout(predicate::str::contains("REGRESSIONS"));
}

#[test]
fn test_diff_json_output() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/index.ts"), "export const x = 1;").unwrap();

    let baseline = dir.path().join("baseline.json");
    let mut cmd1 = Command::new(cargo_bin!("archlint"));
    cmd1.arg("snapshot")
        .arg("-o")
        .arg(&baseline)
        .arg("-p")
        .arg(dir.path())
        .assert()
        .success();

    let mut cmd2 = Command::new(cargo_bin!("archlint"));
    cmd2.arg("diff")
        .arg(&baseline)
        .arg("--json")
        .arg("-p")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("hasRegressions"));
}
