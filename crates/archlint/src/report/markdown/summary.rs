use crate::config::SeverityConfig;
use crate::report::AnalysisReport;

pub fn generate(report: &AnalysisReport, severity_config: &SeverityConfig) -> String {
    let mut output = String::new();

    output.push_str("# Architecture Smell Report\n\n");
    output.push_str("## Summary\n\n");

    let grade = report.grade(severity_config);
    let total_score = report.total_score(severity_config);

    output.push_str("| Metric | Value |\n");
    output.push_str("| :--- | :--- |\n");
    output.push_str(&format!(
        "| **Architecture Quality** | **{:.1}/10 ({})** |\n",
        grade.score, grade.level
    ));
    output.push_str(&format!(
        "| **Total Score** | **{} points** (density: {:.2}) |\n",
        total_score, grade.density
    ));
    output.push_str(&format!("| Files analyzed | {} |\n", report.files_analyzed));
    output.push_str(&format!(
        "| Cyclic dependencies | {} |\n",
        report.cyclic_dependencies
    ));
    output.push_str(&format!("| God modules | {} |\n", report.god_modules));
    output.push_str(&format!("| Dead code files | {} |\n", report.dead_code));
    output.push_str(&format!("| Dead symbols | {} |\n", report.dead_symbols));
    output.push_str(&format!(
        "| High complexity functions | {} |\n",
        report.high_complexity_functions
    ));
    output.push_str(&format!("| Large files | {} |\n", report.large_files));
    output.push_str(&format!(
        "| Unstable interfaces | {} |\n",
        report.unstable_interfaces
    ));
    output.push_str(&format!("| Feature envy | {} |\n", report.feature_envy));
    output.push_str(&format!(
        "| Shotgun surgery | {} |\n",
        report.shotgun_surgery
    ));
    output.push_str(&format!(
        "| Hub dependencies | {} |\n",
        report.hub_dependencies
    ));
    output.push('\n');

    output
}
