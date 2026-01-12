use archlint::{
    api::options::ScanOptions,
    api::Analyzer,
    detectors::SmellType,
    diff::DiffEngine,
    snapshot::{
        read_snapshot, write_snapshot, Location, Snapshot, SnapshotGenerator, SnapshotSmell,
        SnapshotSummary, SCHEMA_VERSION,
    },
};
use std::collections::HashMap;
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

fn make_complexity_smell(id: &str, file: &str, func: &str, line: usize) -> SnapshotSmell {
    SnapshotSmell {
        id: id.to_string(),
        smell_type: "HighComplexity".to_string(),
        severity: "Medium".to_string(),
        files: vec![file.to_string()],
        metrics: HashMap::new(),
        details: Some(SmellType::HighComplexity {
            name: func.to_string(),
            line,
            complexity: 15,
        }),
        locations: vec![Location {
            file: file.to_string(),
            line,
            column: None,
            range: None,
            description: Some(format!("Function '{}' (complexity: 15)", func)),
        }],
    }
}

fn make_snapshot(smells: Vec<SnapshotSmell>) -> Snapshot {
    Snapshot {
        schema_version: SCHEMA_VERSION,
        archlint_version: "0.11.0".to_string(),
        generated_at: "2026-01-11T12:00:00Z".to_string(),
        commit: None,
        smells,
        summary: SnapshotSummary::default(),
        grade: "B".to_string(),
    }
}

#[test]
fn test_shifted_smell_not_regression() {
    // Baseline: complexity smell at line 10
    let baseline = make_snapshot(vec![make_complexity_smell(
        "cmplx:src/service.ts:processData:10",
        "src/service.ts",
        "processData",
        10,
    )]);

    // Current: same smell shifted to line 15 (someone added 5 lines above)
    let current = make_snapshot(vec![make_complexity_smell(
        "cmplx:src/service.ts:processData:15",
        "src/service.ts",
        "processData",
        15,
    )]);

    let result = DiffEngine::default().diff(&baseline, &current);

    // Should NOT report as regression or improvement - it's the same smell, just shifted
    assert!(
        !result.has_regressions,
        "Shifted smell should not be a regression"
    );
    assert!(
        result.improvements.is_empty(),
        "Shifted smell should not be an improvement"
    );
}

#[test]
fn test_renamed_function_is_not_regression() {
    // Baseline: complexity in funcA
    let baseline = make_snapshot(vec![make_complexity_smell(
        "cmplx:src/service.ts:funcA:10",
        "src/service.ts",
        "funcA",
        10,
    )]);

    // Current: complexity in funcB (same location, different name - rename)
    let current = make_snapshot(vec![make_complexity_smell(
        "cmplx:src/service.ts:funcB:12",
        "src/service.ts",
        "funcB",
        12,
    )]);

    let result = DiffEngine::default().diff(&baseline, &current);

    // Should NOT report regressions if renamed and close enough
    assert!(
        !result.has_regressions,
        "Renamed function smell should be matched by proximity"
    );
    assert_eq!(result.summary.new_smells, 0);
    assert_eq!(result.summary.fixed_smells, 0);
}

#[test]
fn test_too_far_shift_is_regression() {
    // Baseline: complexity at line 10
    let baseline = make_snapshot(vec![make_complexity_smell(
        "cmplx:src/service.ts:processData:10",
        "src/service.ts",
        "processData",
        10,
    )]);

    // Current: same function but at line 200 (way too far - likely different occurrence)
    let current = make_snapshot(vec![make_complexity_smell(
        "cmplx:src/service.ts:processData:200",
        "src/service.ts",
        "processData",
        200,
    )]);

    // Use small tolerance to test the boundary
    let result = DiffEngine::default()
        .with_line_tolerance(10)
        .diff(&baseline, &current);

    // With 10 line tolerance, 190 line shift should NOT match
    assert!(
        result.has_regressions,
        "Far shift should be treated as new smell"
    );
    assert_eq!(result.summary.new_smells, 1);
    assert_eq!(result.summary.fixed_smells, 1);
}
