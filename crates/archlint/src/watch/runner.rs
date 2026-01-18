use crate::engine::runner::AnalysisEngine;
use crate::report::AnalysisReport;
use crate::watch::diff::ReportDiff;
use crate::watch::ui::WatchUI;
use crate::watch::FileWatcher;
use crate::Result;
use std::path::PathBuf;

pub struct WatchRunner {
    engine: AnalysisEngine,
    ui: WatchUI,
    last_report: Option<AnalysisReport>,
    clear_screen: bool,
}

impl WatchRunner {
    #[must_use]
    pub fn new(engine: AnalysisEngine, clear_screen: bool) -> Self {
        Self {
            engine,
            ui: WatchUI::new(),
            last_report: None,
            clear_screen,
        }
    }

    pub fn run(&mut self, watcher: FileWatcher) -> Result<()> {
        // Initial analysis
        self.run_analysis()?;

        // Watch for changes
        self.ui.start();

        watcher.watch(|changed_files| self.on_files_changed(changed_files))
    }

    fn on_files_changed(&mut self, files: Vec<PathBuf>) -> Result<()> {
        if self.clear_screen {
            print!("\x1B[2J\x1B[1;1H"); // Clear terminal
        }

        println!("🔄 Files changed: {}", files.len());
        for file in &files {
            println!("   {}", file.display());
        }

        self.run_analysis()?;

        Ok(())
    }

    fn run_analysis(&mut self) -> Result<()> {
        self.ui.show_analyzing();

        let report = self.engine.run()?;

        let diff = self
            .last_report
            .as_ref()
            .map(|prev| ReportDiff::calculate(prev, &report));

        self.ui.show_results(&report, diff.as_ref());

        self.last_report = Some(report);
        Ok(())
    }
}
