use crate::report::AnalysisReport;
use crate::watch::diff::ReportDiff;
use chrono::Local;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

pub struct WatchUI {
    spinner: Option<ProgressBar>,
    #[allow(dead_code)]
    last_update: Instant,
}

impl Default for WatchUI {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchUI {
    #[must_use]
    pub fn new() -> Self {
        Self {
            spinner: None,
            last_update: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Watching for changes...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        self.spinner = Some(pb);
    }

    pub fn show_analyzing(&mut self) {
        if let Some(pb) = &self.spinner {
            pb.set_message("Analyzing...");
        }
    }

    pub fn show_results(&mut self, report: &AnalysisReport, diff: Option<&ReportDiff>) {
        if let Some(pb) = self.spinner.take() {
            pb.finish_and_clear();
        }

        println!("\n{}", style("═".repeat(60)).dim());
        println!(
            "{} Analysis complete at {}",
            style("✓").green().bold(),
            Local::now().format("%H:%M:%S")
        );

        if let Some(diff) = diff {
            if !diff.fixed_smells.is_empty() {
                println!(
                    "{} {} issues fixed!",
                    style("🎉").green(),
                    style(diff.fixed_smells.len().to_string()).bold().green()
                );
            }
            if !diff.new_smells.is_empty() {
                println!(
                    "{} {} new issues detected",
                    style("⚠️").yellow(),
                    style(diff.new_smells.len().to_string()).bold().yellow()
                );
            }
        }

        // Show summary
        println!(
            "\nTotal: {} smells",
            style(report.smells.len().to_string()).bold()
        );
        println!("{}", style("═".repeat(60)).dim());
        println!("\n(Ctrl+C to stop)\n");

        self.start();
    }
}
