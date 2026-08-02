//! Regression tests for the global `ignore:` config key (issue #38).
//!
//! These must go through the real analysis engine (`archlint::scan`): the fixture
//! helper in `tests/common` builds an `AnalysisContext` by hand and never runs
//! `AnalysisEngine::create_report`, so it cannot observe the report-level filter.

use archlint::api::Analyzer;
use archlint::config::{Config, RuleConfig, RuleFullConfig, RuleSeverity};
use archlint::detectors::SmellType;
use archlint::{scan, ScanOptions};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Writes `count` files under `dir/sub`, each importing `axios`.
fn write_consumers(root: &Path, sub: &str, count: usize) {
    let dir = root.join(sub);
    fs::create_dir_all(&dir).unwrap();
    for i in 1..=count {
        fs::write(
            dir.join(format!("m{i}.ts")),
            format!("import axios from 'axios';\nexport const v{i} = axios;\n"),
        )
        .unwrap();
    }
}

/// A project with `live` consumers under `src/` and `ignored` consumers under `legacy/`.
fn project(live: usize, ignored: usize) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    write_consumers(dir.path(), "src", live);
    write_consumers(dir.path(), "legacy", ignored);
    dir
}

fn rule(options: &str) -> RuleConfig {
    RuleConfig::Full(RuleFullConfig {
        enabled: Some(true),
        severity: Some(RuleSeverity::Medium),
        exclude: Vec::new(),
        options: serde_yaml::from_str(options).unwrap(),
    })
}

fn config_with(detector: &str, options: &str, ignore: &[&str]) -> Config {
    let mut config = Config {
        ignore: ignore.iter().map(|s| (*s).to_string()).collect(),
        ..Default::default()
    };
    config.rules.insert(detector.to_string(), rule(options));
    config
}

fn scan_smells(dir: &TempDir, config: Config) -> Vec<archlint::api::SmellWithExplanation> {
    let options = ScanOptions {
        config: Some(config),
        ..ScanOptions::default()
    };
    scan(dir.path(), options).expect("scan failed").smells
}

fn count_of(smells: &[archlint::api::SmellWithExplanation], want: &SmellType) -> usize {
    smells
        .iter()
        .filter(|s| std::mem::discriminant(&s.smell.smell_type) == std::mem::discriminant(want))
        .count()
}

const fn hub() -> SmellType {
    SmellType::HubDependency {
        package: String::new(),
    }
}

const fn vendor() -> SmellType {
    SmellType::VendorCoupling {
        package: String::new(),
    }
}

/// A `src/a.ts` <-> `legacy/b.ts` import cycle.
fn cyclic_project() -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::create_dir_all(dir.path().join("legacy")).unwrap();
    fs::write(
        dir.path().join("src/a.ts"),
        "import { b } from '../legacy/b';\nexport const a = () => b() + 1;\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("legacy/b.ts"),
        "import { a } from '../src/a';\nexport const b = () => a() + 1;\n",
    )
    .unwrap();
    dir
}

#[test]
fn vendor_coupling_ignores_files_from_the_global_ignore_list() {
    let dir = project(3, 12);
    let config = config_with(
        "vendor_coupling",
        "max_files_per_package: 10\nignore_packages: []",
        &["**/legacy/**"],
    );

    let smells = scan_smells(&dir, config);

    assert_eq!(
        count_of(&smells, &vendor()),
        0,
        "3 live consumers are below max_files_per_package: 10, so nothing should be reported"
    );
}

#[test]
fn hub_dependency_ignores_files_from_the_global_ignore_list() {
    let dir = project(3, 12);
    let config = config_with(
        "hub_dependency",
        "min_dependents: 10\nignore_packages: []",
        &["**/legacy/**"],
    );

    let smells = scan_smells(&dir, config);

    assert_eq!(
        count_of(&smells, &hub()),
        0,
        "3 live dependents are below min_dependents: 10, so nothing should be reported"
    );
}

#[test]
fn a_plain_directory_name_is_a_usable_ignore_pattern() {
    let dir = project(3, 12);
    let config = config_with(
        "vendor_coupling",
        "max_files_per_package: 10\nignore_packages: []",
        &["legacy"],
    );

    let smells = scan_smells(&dir, config);

    assert_eq!(count_of(&smells, &vendor()), 0);
}

#[test]
fn the_plain_and_glob_spellings_of_a_directory_agree() {
    // `dead_code` decides which importers count as usage from its own copy of the
    // ignore list, so both spellings have to reach it in the same normalized form.
    let report_for = |ignore: &[&str]| {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("legacy")).unwrap();
        fs::write(dir.path().join("src/index.ts"), "export const entry = 1;\n").unwrap();
        fs::write(dir.path().join("src/util.ts"), "export const util = 1;\n").unwrap();
        fs::write(
            dir.path().join("legacy/consumer.ts"),
            "import { util } from '../src/util';\nexport const consumer = util;\n",
        )
        .unwrap();

        let mut reported: Vec<String> = scan_smells(&dir, config_with("dead_code", "", ignore))
            .iter()
            .flat_map(|s| s.smell.files.clone())
            .map(|f| {
                f.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        reported.sort();
        reported
    };

    let with_glob = report_for(&["**/legacy/**"]);
    assert!(
        with_glob.iter().any(|f| f == "util.ts"),
        "the only importer of util.ts is ignored, so it is dead: {with_glob:?}"
    );
    assert_eq!(report_for(&["legacy"]), with_glob);
}

#[test]
fn ignored_files_count_towards_neither_metrics_nor_locations() {
    let dir = project(3, 12);
    let config = config_with(
        "hub_dependency",
        "min_dependents: 3\nignore_packages: []",
        &["**/legacy/**"],
    );

    let smells = scan_smells(&dir, config);
    let hub_smells: Vec<_> = smells
        .iter()
        .filter(|s| matches!(s.smell.smell_type, SmellType::HubDependency { .. }))
        .collect();

    assert_eq!(
        hub_smells.len(),
        1,
        "3 live dependents reach min_dependents"
    );
    let smell = &hub_smells[0].smell;

    assert_eq!(smell.dependent_count(), Some(3));
    assert_eq!(smell.fan_in(), Some(3));
    assert_eq!(smell.locations.len(), 3);
    assert!(
        !smell
            .locations
            .iter()
            .any(|loc| loc.file.to_string_lossy().contains("legacy")),
        "ignored files must not be listed as locations: {:?}",
        smell.locations
    );
}

#[test]
fn a_smell_in_an_ignored_file_is_still_dropped() {
    let dir = project(1, 1);
    let smells = scan_smells(&dir, config_with("dead_code", "", &["**/legacy/**"]));

    assert!(
        !smells.iter().any(|s| s
            .smell
            .files
            .iter()
            .any(|f| f.to_string_lossy().contains("legacy"))),
        "no smell may be attributed to an ignored file: {:?}",
        smells.iter().map(|s| &s.smell.files).collect::<Vec<_>>()
    );
    assert!(
        smells.iter().any(|s| s
            .smell
            .files
            .iter()
            .any(|f| f.to_string_lossy().contains("src"))),
        "the same smell in a live file must still be reported"
    );
}

#[test]
fn package_cycles_respect_the_global_ignore_list() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("auth")).unwrap();
    fs::create_dir_all(dir.path().join("users")).unwrap();
    fs::write(
        dir.path().join("auth/auth.ts"),
        "import { user } from '../users/user';\nexport const auth = () => user();\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("users/user.ts"),
        "import { auth } from '../auth/auth';\nexport const user = () => auth();\n",
    )
    .unwrap();
    let config = config_with("package_cycles", "package_depth: 1", &["auth"]);

    let smells = scan_smells(&dir, config);

    assert_eq!(
        count_of(
            &smells,
            &SmellType::PackageCycle {
                packages: Vec::new()
            }
        ),
        0,
        "a package cycle carries no files at all, so it used to be unignorable"
    );
}

#[test]
fn scattered_config_counts_only_live_files() {
    let dir = tempfile::tempdir().unwrap();
    for (sub, count) in [("src", 3), ("legacy", 4)] {
        fs::create_dir_all(dir.path().join(sub)).unwrap();
        for i in 1..=count {
            fs::write(
                dir.path().join(sub).join(format!("f{i}.ts")),
                "export const x = process.env.API_KEY;\n",
            )
            .unwrap();
        }
    }
    let config = config_with("scattered_config", "max_files: 3", &["**/legacy/**"]);

    let smells = scan_smells(&dir, config);

    assert_eq!(
        count_of(
            &smells,
            &SmellType::ScatteredConfiguration {
                env_var: String::new(),
                files_count: 0,
            }
        ),
        0,
        "3 live files do not exceed max_files: 3"
    );
}

#[test]
fn the_incremental_path_honours_the_global_ignore_list() {
    let dir = project(3, 12);
    let config = config_with(
        "vendor_coupling",
        "max_files_per_package: 2\nignore_packages: []",
        &["**/legacy/**"],
    );

    let mut analyzer = Analyzer::new(
        dir.path(),
        ScanOptions {
            config: Some(config),
            ..ScanOptions::default()
        },
    )
    .unwrap();
    analyzer.scan().unwrap();

    let changed = vec![dir.path().join("src/m1.ts").canonicalize().unwrap()];
    let result = analyzer.scan_incremental(changed).unwrap();

    assert!(
        result
            .smells
            .iter()
            .any(|s| matches!(s.smell.smell_type, SmellType::VendorCoupling { .. })),
        "the live smell must still be reported"
    );
    assert!(
        !result.smells.iter().any(|s| s
            .smell
            .files
            .iter()
            .any(|f| f.to_string_lossy().contains("legacy"))),
        "the incremental path never applied the global ignore list before"
    );
}

#[test]
fn a_cycle_through_an_ignored_file_is_still_reported() {
    let dir = cyclic_project();
    let config = config_with("cyclic_dependency", "", &["**/legacy/**"]);

    let smells = scan_smells(&dir, config);

    assert_eq!(
        count_of(&smells, &SmellType::CyclicDependencyCluster),
        1,
        "a cycle is a topological fact: it stays reportable while one member is live"
    );
}

#[test]
fn a_cycle_entirely_inside_an_ignored_directory_is_dropped() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("legacy")).unwrap();
    fs::write(
        dir.path().join("legacy/a.ts"),
        "import { b } from './b';\nexport const a = () => b() + 1;\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("legacy/b.ts"),
        "import { a } from './a';\nexport const b = () => a() + 1;\n",
    )
    .unwrap();
    let config = config_with("cyclic_dependency", "", &["**/legacy/**"]);

    let smells = scan_smells(&dir, config);

    assert_eq!(count_of(&smells, &SmellType::CyclicDependencyCluster), 0);
}
