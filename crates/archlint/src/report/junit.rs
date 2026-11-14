//! `JUnit` XML reports for CI test-report consumers.
//!
//! Each architectural smell becomes a failed `<testcase>`. A scan with no
//! smells emits a single passing test case so pipelines still record a result.
//! GitLab, Jenkins, and Azure DevOps accept this shape.

use crate::detectors::{ArchSmell, SmellType};
use crate::explain::Explanation;
use crate::report::sarif::normalize_path;
use crate::report::AnalysisReport;
use crate::Result;
use std::fs;
use std::path::Path;

/// Generates a `JUnit` XML document from an `AnalysisReport`.
///
/// # Arguments
/// * `report` - The analysis report containing detected smells and explanations.
/// * `_config` - Severity configuration (reserved for future customization).
/// * `scan_root` - Optional root path used to make file paths relative.
#[must_use]
pub fn generate_junit(
    report: &AnalysisReport,
    _config: &crate::config::SeverityConfig,
    scan_root: Option<&Path>,
) -> String {
    let suites = group_suites(report, scan_root);
    render_document(&suites)
}

/// Writes a `JUnit` XML report to `path`.
///
/// # Errors
/// Returns an error when the file cannot be written.
pub fn write_report<P: AsRef<Path>>(
    report: &AnalysisReport,
    path: P,
    config: &crate::config::SeverityConfig,
    scan_root: Option<&Path>,
) -> Result<()> {
    let output = generate_junit(report, config, scan_root);
    fs::write(path, output)?;
    Ok(())
}

struct Suite {
    name: String,
    cases: Vec<String>,
    failures: usize,
}

fn group_suites(report: &AnalysisReport, scan_root: Option<&Path>) -> Vec<Suite> {
    if report.smells.is_empty() {
        return vec![passing_suite()];
    }

    let mut suites: Vec<Suite> = Vec::new();
    for (smell, explanation) in &report.smells {
        let name = smell.smell_type.category().to_id().to_string();
        let case_xml = render_case(smell, explanation, scan_root);
        if let Some(suite) = suites.iter_mut().find(|suite| suite.name == name) {
            suite.cases.push(case_xml);
            suite.failures += 1;
        } else {
            suites.push(Suite {
                name,
                cases: vec![case_xml],
                failures: 1,
            });
        }
    }
    suites
}

fn passing_suite() -> Suite {
    Suite {
        name: "archlint".to_string(),
        cases: vec![
            r#"    <testcase name="no architectural smells" classname="archlint"/>"#.to_string(),
        ],
        failures: 0,
    }
}

fn render_document(suites: &[Suite]) -> String {
    let tests: usize = suites.iter().map(|suite| suite.cases.len()).sum();
    let failures: usize = suites.iter().map(|suite| suite.failures).sum();
    let mut out = String::new();
    out.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    out.push('\n');
    out.push_str(&format!(
        r#"<testsuites name="archlint" tests="{tests}" failures="{failures}" errors="0" time="0">"#
    ));
    out.push('\n');
    for suite in suites {
        render_suite(&mut out, suite);
    }
    out.push_str("</testsuites>\n");
    out
}

fn render_suite(out: &mut String, suite: &Suite) {
    out.push_str(&format!(
        r#"  <testsuite name="{}" tests="{}" failures="{}" errors="0" time="0">"#,
        escape_attr(&suite.name),
        suite.cases.len(),
        suite.failures,
    ));
    out.push('\n');
    for case in &suite.cases {
        out.push_str(case);
        out.push('\n');
    }
    out.push_str("  </testsuite>\n");
}

fn render_case(smell: &ArchSmell, explanation: &Explanation, scan_root: Option<&Path>) -> String {
    let rule_id = smell.smell_type.category().to_id();
    let location = primary_location(smell, scan_root);
    let name = case_name(
        &explanation.problem,
        location.as_ref(),
        case_discriminator(smell, scan_root).as_deref(),
    );
    let mut attrs = format!(
        r#"name="{}" classname="{}""#,
        escape_attr(&name),
        escape_attr(rule_id),
    );
    if let Some(location) = &location {
        attrs.push_str(&format!(r#" file="{}""#, escape_attr(&location.file)));
        if let Some(line) = location.line {
            attrs.push_str(&format!(r#" line="{line}""#));
        }
    }

    let failure_type: &'static str = (&smell.severity).into();
    format!(
        "    <testcase {attrs}>\n      <failure type=\"{failure_type}\" message=\"{}\">{}</failure>\n    </testcase>",
        escape_attr(&explanation.problem),
        escape_text(&failure_body(explanation, smell, scan_root)),
    )
}

struct CaseLocation {
    file: String,
    line: Option<usize>,
}

fn primary_location(smell: &ArchSmell, scan_root: Option<&Path>) -> Option<CaseLocation> {
    if let Some(loc) = smell.locations.first() {
        let line = (loc.line > 0).then_some(loc.line);
        return Some(CaseLocation {
            file: normalize_path(&loc.file, scan_root),
            line,
        });
    }
    smell.files.first().map(|file| CaseLocation {
        file: normalize_path(file, scan_root),
        line: None,
    })
}

fn case_discriminator(smell: &ArchSmell, scan_root: Option<&Path>) -> Option<String> {
    if let Some(name) = dead_symbol_name(smell) {
        return Some(name.to_string());
    }
    smell.locations.first().and_then(|location| {
        (!location.description.is_empty())
            .then(|| strip_scan_root(&location.description, scan_root))
    })
}

/// Removes the scan root prefix from paths embedded in free-form text so that
/// test case names stay stable across machines and CI workspaces.
///
/// The root is only stripped where it starts a path (not in the middle of
/// another one), and Windows separators are normalized to `/`.
///
/// NOTE: detector descriptions are free-form; detectors should eventually emit
/// scan-root-relative paths themselves.
fn strip_scan_root(text: &str, scan_root: Option<&Path>) -> String {
    let Some(root) = scan_root else {
        return text.to_string();
    };
    let native = root.to_string_lossy();
    let native = native.trim_end_matches(['/', '\\']);
    if native.is_empty() {
        return text.to_string();
    }
    let slashed = native.replace('\\', "/");
    let text = if slashed == native {
        std::borrow::Cow::Borrowed(text)
    } else {
        std::borrow::Cow::Owned(text.replace('\\', "/"))
    };
    let needle = format!("{slashed}/");

    let mut out = String::with_capacity(text.len());
    let mut rest: &str = &text;
    while let Some(pos) = rest.find(&needle) {
        let (head, tail) = rest.split_at(pos);
        out.push_str(head);
        let starts_path = match out.chars().next_back() {
            None => true,
            Some(c) => !(c.is_alphanumeric() || matches!(c, '/' | '.' | '_' | '-')),
        };
        if !starts_path {
            out.push_str(&needle);
        }
        rest = &tail[needle.len()..];
    }
    out.push_str(rest);
    out
}

const fn dead_symbol_name(smell: &ArchSmell) -> Option<&str> {
    match &smell.smell_type {
        SmellType::DeadSymbol { name, .. } if !name.is_empty() => Some(name.as_str()),
        _ => None,
    }
}

fn case_name(problem: &str, location: Option<&CaseLocation>, symbol: Option<&str>) -> String {
    let problem = if problem.is_empty() {
        "Architectural smell"
    } else {
        problem
    };
    let labeled = match symbol {
        Some(symbol) => format!("{problem}: {symbol}"),
        None => problem.to_string(),
    };
    match location {
        Some(CaseLocation {
            file,
            line: Some(line),
        }) => format!("{labeled} ({file}:{line})"),
        Some(CaseLocation { file, line: None }) => format!("{labeled} ({file})"),
        None => labeled,
    }
}

fn failure_body(explanation: &Explanation, smell: &ArchSmell, scan_root: Option<&Path>) -> String {
    let mut body = String::new();
    body.push_str(&explanation.problem);
    body.push_str("\n\nReason:\n");
    body.push_str(&explanation.reason);
    push_list(&mut body, "Risks", &explanation.risks);
    push_list(&mut body, "Recommendations", &explanation.recommendations);
    push_locations(&mut body, smell, scan_root);
    body
}

fn push_list(body: &mut String, title: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    body.push_str(&format!("\n\n{title}:\n"));
    for item in items {
        body.push_str("- ");
        body.push_str(item);
        body.push('\n');
    }
}

fn push_locations(body: &mut String, smell: &ArchSmell, scan_root: Option<&Path>) {
    if smell.locations.is_empty() && smell.files.is_empty() {
        return;
    }
    body.push_str("\n\nLocations:\n");
    if smell.locations.is_empty() {
        for file in &smell.files {
            body.push_str("- ");
            body.push_str(&normalize_path(file, scan_root));
            body.push('\n');
        }
        return;
    }
    for loc in &smell.locations {
        body.push_str("- ");
        body.push_str(&normalize_path(&loc.file, scan_root));
        if loc.line > 0 {
            body.push(':');
            body.push_str(&loc.line.to_string());
        }
        if !loc.description.is_empty() {
            body.push_str(" (");
            body.push_str(&strip_scan_root(&loc.description, scan_root));
            body.push(')');
        }
        body.push('\n');
    }
}

fn escape_attr(input: &str) -> String {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    escape_xml(&collapsed, true)
}

fn escape_text(input: &str) -> String {
    escape_xml(input, false)
}

fn escape_xml(input: &str, attr: bool) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' if attr => out.push_str("&quot;"),
            '\'' if attr => out.push_str("&apos;"),
            c if is_xml_char(c) => out.push(c),
            _ => {}
        }
    }
    out
}

const fn is_xml_char(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | '\u{20}'..='\u{FFFD}' | '\u{10000}'..=char::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detectors::smell::{ArchSmell, LocationDetail};
    use crate::detectors::types::{Severity, SmellType};
    use crate::report::AnalysisReportBuilder;
    use std::path::PathBuf;

    fn report_with(smells: Vec<ArchSmell>) -> AnalysisReport {
        AnalysisReportBuilder::new().with_smells(smells).build()
    }

    #[test]
    fn empty_report_is_a_passing_suite() {
        let report = report_with(vec![]);
        let xml = generate_junit(&report, &crate::config::SeverityConfig::default(), None);

        assert!(xml.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
        assert!(xml.contains(r#"tests="1" failures="0" errors="0""#));
        assert!(xml.contains(r#"name="no architectural smells" classname="archlint""#));
        assert!(!xml.contains("<failure"));
    }

    #[test]
    fn smell_becomes_failed_testcase() {
        let smell = ArchSmell {
            smell_type: SmellType::CyclicDependency,
            severity: Severity::High,
            files: vec![PathBuf::from("a.ts"), PathBuf::from("b.ts")],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                PathBuf::from("a.ts"),
                4,
                "import b".to_string(),
            )],
            cluster: None,
        };
        let report = report_with(vec![smell]);
        let xml = generate_junit(&report, &crate::config::SeverityConfig::default(), None);

        assert!(xml.contains(r#"<testsuite name="cyclic_dependency" tests="1" failures="1""#));
        assert!(xml.contains(r#"classname="cyclic_dependency" file="a.ts" line="4""#));
        assert!(xml.contains(r#"type="high""#));
        assert!(xml.contains("Reason:"));
        assert!(xml.contains("Locations:"));
        assert!(xml.contains("a.ts:4 (import b)"));
        assert!(xml
            .contains(r#"name="Circular dependency detected between 0 files: import b (a.ts:4)""#));
    }

    #[test]
    fn side_effect_imports_on_the_same_line_get_distinct_names() {
        let smells = vec![
            ArchSmell::new_side_effect_import(PathBuf::from("a.ts"), "polyfill".to_string(), 2),
            ArchSmell::new_side_effect_import(
                PathBuf::from("a.ts"),
                "reflect-metadata".to_string(),
                2,
            ),
        ];
        let report = report_with(smells);
        let xml = generate_junit(&report, &crate::config::SeverityConfig::default(), None);

        assert!(xml.contains(
            r#"name="Side-Effect Import: Side-effect import of &apos;polyfill&apos; (a.ts:2)""#
        ));
        assert!(xml.contains(
            r#"name="Side-Effect Import: Side-effect import of &apos;reflect-metadata&apos; (a.ts:2)""#
        ));
    }

    #[test]
    fn dead_symbols_on_the_same_line_get_distinct_names() {
        let smells = vec![
            ArchSmell::new_dead_symbol_with_line(
                PathBuf::from("a.ts"),
                "foo".to_string(),
                "Function".to_string(),
                3,
            ),
            ArchSmell::new_dead_symbol_with_line(
                PathBuf::from("a.ts"),
                "bar".to_string(),
                "Function".to_string(),
                3,
            ),
        ];
        let report = report_with(smells);
        let xml = generate_junit(&report, &crate::config::SeverityConfig::default(), None);

        assert!(xml.contains(r#"name="Unused Function detected: foo (a.ts:3)""#));
        assert!(xml.contains(r#"name="Unused Function detected: bar (a.ts:3)""#));
    }

    #[test]
    fn groups_smells_by_detector_and_relativizes_paths() {
        let scan_root = PathBuf::from("/project");
        let smells = vec![
            ArchSmell {
                smell_type: SmellType::DeadCode,
                severity: Severity::Low,
                files: vec![PathBuf::from("/project/src/unused.ts")],
                metrics: vec![],
                locations: vec![],
                cluster: None,
            },
            ArchSmell {
                smell_type: SmellType::GodModule,
                severity: Severity::Critical,
                files: vec![PathBuf::from("/project/src/app.ts")],
                metrics: vec![],
                locations: vec![LocationDetail::new(
                    PathBuf::from("/project/src/app.ts"),
                    10,
                    "too many exports".to_string(),
                )],
                cluster: None,
            },
            ArchSmell {
                smell_type: SmellType::DeadCode,
                severity: Severity::Medium,
                files: vec![PathBuf::from("/project/src/old.ts")],
                metrics: vec![],
                locations: vec![],
                cluster: None,
            },
        ];
        let report = report_with(smells);
        let xml = generate_junit(
            &report,
            &crate::config::SeverityConfig::default(),
            Some(&scan_root),
        );

        assert!(xml.contains(r#"<testsuites name="archlint" tests="3" failures="3""#));
        assert!(xml.contains(r#"<testsuite name="dead_code" tests="2" failures="2""#));
        assert!(xml.contains(r#"<testsuite name="god_module" tests="1" failures="1""#));
        assert!(xml.contains(r#"file="src/unused.ts""#));
        assert!(xml.contains(r#"file="src/app.ts" line="10""#));
        assert!(xml.contains(r#"type="critical""#));
        let dead_code = xml.find(r#"name="dead_code""#).unwrap();
        let god_module = xml.find(r#"name="god_module""#).unwrap();
        assert!(dead_code < god_module);
    }

    #[test]
    fn escapes_xml_and_normalizes_windows_separators() {
        let file_path = if cfg!(windows) {
            PathBuf::from(r"C:\project\src\a&b.ts")
        } else {
            PathBuf::from("/project/src/a&b.ts")
        };
        let scan_root = if cfg!(windows) {
            PathBuf::from(r"C:\project")
        } else {
            PathBuf::from("/project")
        };
        let smell = ArchSmell {
            smell_type: SmellType::GodModule,
            severity: Severity::Medium,
            files: vec![file_path.clone()],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                file_path,
                2,
                r#"uses <img> & "quotes""#.to_string(),
            )],
            cluster: None,
        };
        let report = report_with(vec![smell]);
        let xml = generate_junit(
            &report,
            &crate::config::SeverityConfig::default(),
            Some(&scan_root),
        );

        assert!(xml.contains(r#"file="src/a&amp;b.ts" line="2""#));
        assert!(xml.contains(r#"uses &lt;img&gt; &amp; "quotes""#));
        assert!(!xml.contains("<img>"));
    }

    #[test]
    fn case_names_do_not_contain_the_scan_root() {
        let scan_root = PathBuf::from("/project");
        let smell = ArchSmell {
            smell_type: SmellType::GodModule,
            severity: Severity::Medium,
            files: vec![PathBuf::from("/project/src/a.ts")],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                PathBuf::from("/project/src/a.ts"),
                1,
                "imports from '/project/src/b.ts'".to_string(),
            )],
            cluster: None,
        };
        let report = report_with(vec![smell]);
        let xml = generate_junit(
            &report,
            &crate::config::SeverityConfig::default(),
            Some(&scan_root),
        );

        assert!(xml.contains("imports from &apos;src/b.ts&apos; (src/a.ts:1)"));
        assert!(!xml.contains("/project/"));
    }

    #[test]
    fn strip_scan_root_handles_edge_cases() {
        assert_eq!(strip_scan_root("/p/a.ts", None), "/p/a.ts");
        assert_eq!(strip_scan_root("/p/a.ts", Some(Path::new(""))), "/p/a.ts");
        assert_eq!(strip_scan_root("/p/a.ts", Some(Path::new("/p/"))), "a.ts");
        assert_eq!(
            strip_scan_root(r"C:\p\a.ts", Some(Path::new(r"C:\p"))),
            "a.ts"
        );
        assert_eq!(
            strip_scan_root("C:/p/a.ts", Some(Path::new(r"C:\p"))),
            "a.ts"
        );
    }

    #[test]
    fn strip_scan_root_only_strips_path_starts() {
        let root = Some(Path::new("/p"));
        assert_eq!(strip_scan_root("imports '/p/b.ts'", root), "imports 'b.ts'");
        assert_eq!(strip_scan_root("/x/p/y", root), "/x/p/y");
        assert_eq!(strip_scan_root("a /p/b and (/p/c)", root), "a b and (c)");
        assert_eq!(
            strip_scan_root(r"C:\p\src\b.ts", Some(Path::new(r"C:\p"))),
            "src/b.ts"
        );
    }

    #[test]
    fn case_name_falls_back_for_empty_problem() {
        assert_eq!(case_name("", None, None), "Architectural smell");
    }

    #[test]
    fn smell_without_locations_or_files_has_no_file_attributes() {
        let smell = ArchSmell {
            smell_type: SmellType::GodModule,
            severity: Severity::Low,
            files: vec![],
            metrics: vec![],
            locations: vec![],
            cluster: None,
        };
        let report = report_with(vec![smell]);
        let xml = generate_junit(&report, &crate::config::SeverityConfig::default(), None);

        assert!(xml.contains("<testcase name="));
        assert!(!xml.contains(" file="));
        assert!(!xml.contains("Locations:"));
    }

    #[test]
    fn push_list_skips_empty_lists() {
        let mut body = String::new();
        push_list(&mut body, "Risks", &[]);
        assert_eq!(body, "");
    }

    #[test]
    fn invalid_xml_chars_are_dropped() {
        assert_eq!(
            escape_text("a\u{FFFE}b\u{FFFF}c\u{0}d\u{10000}"),
            "abcd\u{10000}"
        );
    }

    #[test]
    fn write_report_persists_xml() {
        let report = report_with(vec![]);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("archlint-junit.xml");
        write_report(
            &report,
            &path,
            &crate::config::SeverityConfig::default(),
            None,
        )
        .unwrap();
        let written = fs::read_to_string(path).unwrap();
        assert!(written.contains("<testsuites"));
    }
}
