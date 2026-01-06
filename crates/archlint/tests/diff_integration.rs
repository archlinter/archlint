use archlint::{
    api::options::ScanOptions,
    api::Analyzer,
    diff::DiffEngine,
    snapshot::{read_snapshot, write_snapshot, SnapshotGenerator},
};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_full_snapshot_diff_cycle() {
    let dir = tempdir().unwrap();
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();

    // Create baseline files (no cycle)
    fs::write(src_dir.join("a.ts"), "export const a = 1;").unwrap();
    fs::write(src_dir.join("b.ts"), "export const b = 2;").unwrap();

    // Generate baseline
    let mut analyzer = Analyzer::new(dir.path(), ScanOptions::default()).unwrap();
    let scan1 = analyzer.scan().unwrap();
    let baseline = SnapshotGenerator::new(dir.path().to_path_buf())
        .with_commit(false)
        .generate(&scan1);

    let baseline_path = dir.path().join("baseline.json");
    write_snapshot(&baseline, &baseline_path).unwrap();

    // Add cycle
    fs::write(
        src_dir.join("a.ts"),
        "import { b } from './b';\nexport const a = b;",
    )
    .unwrap();
    fs::write(
        src_dir.join("b.ts"),
        "import { a } from './a';\nexport const b = a;",
    )
    .unwrap();

    // Generate current (re-scan)
    let scan2 = analyzer.rescan().unwrap();
    let current = SnapshotGenerator::new(dir.path().to_path_buf())
        .with_commit(false)
        .generate(&scan2);

    // Diff
    let loaded_baseline = read_snapshot(&baseline_path).unwrap();
    let result = DiffEngine::default().diff(&loaded_baseline, &current);

    // Should detect new cycle
    assert!(result.has_regressions);
    assert!(result
        .regressions
        .iter()
        .any(|r| r.smell.smell_type.contains("Cyclic")));
}

#[test]
fn test_snapshot_determinism() {
    let dir = tempdir().unwrap();
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();

    fs::write(src_dir.join("a.ts"), "export const a = 1;").unwrap();
    fs::write(src_dir.join("b.ts"), "import { a } from './a';").unwrap();

    let mut analyzer = Analyzer::new(dir.path(), ScanOptions::default()).unwrap();

    // Generate twice
    let scan1 = analyzer.scan().unwrap();
    let snap1 = SnapshotGenerator::new(dir.path().to_path_buf())
        .with_commit(false)
        .generate(&scan1);

    let scan2 = analyzer.rescan().unwrap();
    let snap2 = SnapshotGenerator::new(dir.path().to_path_buf())
        .with_commit(false)
        .generate(&scan2);

    // IDs should be identical
    assert_eq!(snap1.smells.len(), snap2.smells.len());
    for (s1, s2) in snap1.smells.iter().zip(snap2.smells.iter()) {
        assert_eq!(s1.id, s2.id);
    }
}
