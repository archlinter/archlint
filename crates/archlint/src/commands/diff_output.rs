#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::modifiers::UTF8_ROUND_CORNERS;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::presets::UTF8_FULL;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;

#[cfg(feature = "cli")]
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
#[cfg(feature = "cli")]
use comfy_table::presets::UTF8_FULL;
#[cfg(feature = "cli")]
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
#[cfg(feature = "cli")]
use console::style;

use crate::diff::{DiffResult, Regression, RegressionType};

pub fn print_diff_result(result: &DiffResult, verbose: bool) {
    if !result.has_regressions && result.improvements.is_empty() {
        println!("{}", style("✓ No architectural changes detected.").green());
        return;
    }

    // Header
    if result.has_regressions {
        println!("{}", style("━".repeat(50)).dim());
        println!(
            "  {} ({})",
            style("ARCHITECTURAL REGRESSIONS DETECTED").red().bold(),
            result.regressions.len()
        );
        println!("{}", style("━".repeat(50)).dim());
        println!();

        if !verbose {
            print_regressions_table(&result.regressions);
        } else {
            // Detailed regressions with explanations
            for reg in &result.regressions {
                print_regression(reg, verbose);
            }
        }
    }

    // Improvements (brief)
    if !result.improvements.is_empty() {
        if result.has_regressions {
            println!("{}", style("─".repeat(50)).dim());
        }
        println!(
            "{} {} improvements (smells fixed or reduced)",
            style("✓").green(),
            result.improvements.len()
        );
    }

    // Footer
    println!();
    println!("{}", style("━".repeat(50)).dim());
    if let (Some(base), Some(curr)) = (&result.baseline_commit, &result.current_commit) {
        println!("Baseline: {}", style(base).dim());
        println!("Current:  {}", style(curr).dim());
    }
    println!("{}", style("━".repeat(50)).dim());

    if !verbose && result.has_regressions {
        println!();
        println!(
            "Run with {} for detailed fix guidance.",
            style("--explain").cyan()
        );
    }
}

fn print_regressions_table(regressions: &[Regression]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Status").add_attribute(Attribute::Bold),
            Cell::new("Severity").add_attribute(Attribute::Bold),
            Cell::new("Smell").add_attribute(Attribute::Bold),
            Cell::new("Location").add_attribute(Attribute::Bold),
        ]);

    for reg in regressions {
        let (status_text, color) = match &reg.regression_type {
            RegressionType::NewSmell => ("NEW", Color::Red),
            RegressionType::SeverityIncrease { .. } => ("UPGRADED", Color::Red),
            RegressionType::MetricWorsening { .. } => ("WORSENED", Color::Yellow),
        };

        let severity_cell = format_severity_cell(&reg.smell.severity);
        let locations_str = format_reg_locations(reg);

        table.add_row(vec![
            Cell::new(status_text)
                .fg(color)
                .add_attribute(Attribute::Bold),
            severity_cell,
            Cell::new(&reg.smell.smell_type),
            Cell::new(locations_str),
        ]);
    }

    println!("{}", table);
}

fn format_severity_cell(severity: &str) -> Cell {
    let (text, color) = match severity {
        "Critical" => ("🔴 CRITICAL", Color::Red),
        "High" => ("🟠 HIGH", Color::Red),
        "Medium" => ("🟡 MEDIUM", Color::Yellow),
        "Low" => ("🔵 LOW", Color::Cyan),
        _ => (severity, Color::White),
    };

    let mut cell = Cell::new(text).fg(color);
    if severity == "Critical" || severity == "High" {
        cell = cell.add_attribute(Attribute::Bold);
    }
    cell
}

fn format_reg_locations(reg: &Regression) -> String {
    if !reg.smell.locations.is_empty() {
        reg.smell
            .locations
            .iter()
            .map(format_snapshot_location)
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        reg.smell.files.join("\n")
    }
}

fn format_snapshot_location(loc: &crate::snapshot::types::Location) -> String {
    let mut base = if loc.line > 0 {
        match loc.column {
            Some(col) => format!("{}:{}:{}", loc.file, loc.line, col),
            None => format!("{}:{}", loc.file, loc.line),
        }
    } else {
        loc.file.clone()
    };

    if let Some(desc) = &loc.description {
        base = format!("{} ({})", base, desc);
    }
    base
}

fn print_regression(reg: &Regression, verbose: bool) {
    let (icon, _color) = match &reg.regression_type {
        RegressionType::NewSmell => ("❌ NEW", Color::Red),
        RegressionType::SeverityIncrease { .. } => ("⬆️ SEVERITY", Color::Red),
        RegressionType::MetricWorsening { .. } => ("⚠️ WORSENED", Color::Yellow),
    };

    let mut styled_type = style(&reg.smell.smell_type).yellow();
    if verbose {
        styled_type = styled_type.bold();
    }

    println!("{}: {}", style(icon).bold(), styled_type);

    // Locations
    let locations = format_reg_locations(reg);
    for loc in locations.lines() {
        println!("   {}", loc);
    }

    // Type-specific info
    match &reg.regression_type {
        RegressionType::SeverityIncrease { from, to } => {
            println!("   Severity: {} → {}", from, style(to).red());
        }
        RegressionType::MetricWorsening {
            metric,
            from,
            to,
            change_percent,
        } => {
            println!(
                "   {}: {:.2} → {:.2} (+{:.0}%)",
                metric, from, to, change_percent
            );
        }
        _ => {}
    }

    // Explain (if verbose)
    if verbose {
        if let Some(explain) = &reg.explain {
            println!();
            println!("   {}", style("WHY BAD:").cyan().bold());
            for line in explain.why_bad.lines() {
                println!("   {}", line);
            }
            println!();
            println!("   {}", style("HOW TO FIX:").green().bold());
            for line in explain.how_to_fix.lines() {
                println!("   {}", line);
            }
        }
    }

    println!();
}
