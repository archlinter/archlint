#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;
#[cfg(feature = "cli")]
use console::style;

#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::comfy_table::{Attribute, Color};
#[cfg(feature = "cli")]
use console::{Attribute, Color};

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
    }

    // Regressions
    for reg in &result.regressions {
        print_regression(reg, verbose);
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

fn print_regression(reg: &Regression, verbose: bool) {
    let (icon, icon_color) = match &reg.regression_type {
        RegressionType::NewSmell => ("❌ NEW", Color::Red),
        RegressionType::SeverityIncrease { .. } => ("⬆️ SEVERITY", Color::Red),
        RegressionType::MetricWorsening { .. } => ("⚠️ WORSENED", Color::Yellow),
    };

    println!(
        "{}: {}",
        style(icon).attr(Attribute::Bold).fg(icon_color),
        style(&reg.smell.smell_type).yellow()
    );

    // Files
    for file in &reg.smell.files {
        println!("   {}", file);
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
                "   {}: {} → {} (+{:.0}%)",
                metric,
                from,
                style(to).red(),
                change_percent
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
