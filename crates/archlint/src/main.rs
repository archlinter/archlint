use archlint::args::{OutputFormat, ScanArgs};
use archlint::framework::detector::FrameworkDetector;
use archlint::framework::Framework;
use archlint::{
    cache, cli, config, detectors, engine, glob_expand, report, watch, AnalysisError, Result,
};
use clap::{CommandFactory, Parser};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use console::style;
use log::info;
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::Instant;

// Use mimalloc as global allocator for better performance
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let cli = cli::Cli::parse();
    let mut builder = env_logger::Builder::new();

    configure_logger(&mut builder, &cli);
    builder.format(format_log_record);
    builder.init();

    if let Err(e) = run(cli) {
        eprintln!("{} {}", style("error:").red().bold(), e);
        std::process::exit(1);
    }
}

fn configure_logger(builder: &mut env_logger::Builder, cli: &cli::Cli) {
    set_initial_log_level(builder, cli);

    if std::env::var("RUST_LOG").is_err() {
        set_final_log_level(builder, cli);
    }
}

fn set_initial_log_level(builder: &mut env_logger::Builder, cli: &cli::Cli) {
    match &cli.command {
        Some(cli::Command::Scan(args)) => {
            if args.verbose {
                builder.filter_level(log::LevelFilter::Debug);
            } else if args.is_quiet() {
                builder.filter_level(log::LevelFilter::Error);
            }
        }
        Some(cli::Command::Watch(args)) => {
            if args.scan.verbose {
                builder.filter_level(log::LevelFilter::Debug);
            } else if args.scan.is_quiet() {
                builder.filter_level(log::LevelFilter::Error);
            }
        }
        None => {
            let args = cli.to_scan_args();
            if args.verbose {
                builder.filter_level(log::LevelFilter::Debug);
            } else if args.is_quiet() {
                builder.filter_level(log::LevelFilter::Error);
            }
        }
        Some(cli::Command::Snapshot(_)) => {
            // Default log level is handled by the common initialization
        }
        Some(cli::Command::Diff(args)) => {
            if args.json {
                builder.filter_level(log::LevelFilter::Error);
            } else if args.verbose {
                builder.filter_level(log::LevelFilter::Debug);
            }
        }
        _ => {}
    }
}

fn set_final_log_level(builder: &mut env_logger::Builder, cli: &cli::Cli) {
    match &cli.command {
        Some(cli::Command::Scan(args)) if args.is_quiet() => {
            builder.filter_level(log::LevelFilter::Error)
        }
        Some(cli::Command::Scan(args)) if args.verbose => {
            builder.filter_level(log::LevelFilter::Debug)
        }
        Some(cli::Command::Watch(args)) if args.scan.is_quiet() => {
            builder.filter_level(log::LevelFilter::Error)
        }
        Some(cli::Command::Watch(args)) if args.scan.verbose => {
            builder.filter_level(log::LevelFilter::Debug)
        }
        Some(cli::Command::Snapshot(_)) => builder.filter_level(log::LevelFilter::Info),
        Some(cli::Command::Diff(args)) => {
            if args.json {
                builder.filter_level(log::LevelFilter::Error)
            } else if args.verbose {
                builder.filter_level(log::LevelFilter::Debug)
            } else {
                builder.filter_level(log::LevelFilter::Info)
            }
        }
        None => {
            let args = cli.to_scan_args();
            if args.is_quiet() {
                builder.filter_level(log::LevelFilter::Error)
            } else if args.verbose {
                builder.filter_level(log::LevelFilter::Debug)
            } else {
                builder.filter_level(log::LevelFilter::Info)
            }
        }
        _ => builder.filter_level(log::LevelFilter::Info),
    };
}

fn format_log_record(
    buf: &mut env_logger::fmt::Formatter,
    record: &log::Record,
) -> std::io::Result<()> {
    match record.level() {
        log::Level::Info => writeln!(buf, "{}", record.args()),
        log::Level::Error => writeln!(buf, "{} {}", style("error:").red().bold(), record.args()),
        log::Level::Warn => writeln!(
            buf,
            "{} {}",
            style("warning:").yellow().bold(),
            record.args()
        ),
        log::Level::Debug => writeln!(buf, "{} {}", style("debug:").magenta(), record.args()),
        _ => writeln!(buf, "{:5} {}", record.level(), record.args()),
    }
}

fn determine_exit_code(report: &report::AnalysisReport) -> i32 {
    let has_critical = report
        .smells
        .iter()
        .any(|(s, _)| s.severity == detectors::Severity::Critical);

    let has_high = report
        .smells
        .iter()
        .any(|(s, _)| s.severity == detectors::Severity::High);

    if has_critical {
        2 // Critical issues found
    } else if has_high {
        1 // High severity issues found
    } else {
        0 // Only medium/low or no issues
    }
}

fn resolve_scan_args(args: ScanArgs) -> Result<ScanArgs> {
    let path_str = args.path.to_string_lossy();

    // If path exists as file or directory, use it as is
    if args.path.exists() {
        return Ok(args);
    }

    // If it contains glob characters, expand it
    if path_str.contains('*') || path_str.contains('?') || path_str.contains('[') {
        let expansion = glob_expand::expand_glob(&path_str, archlint::args::SUPPORTED_EXTENSIONS)?;

        if expansion.files.is_empty() {
            return Err(AnalysisError::PathResolution(format!(
                "No files found matching pattern: {}",
                path_str
            )));
        }

        return Ok(ScanArgs {
            path: expansion.base_path,
            files: Some(expansion.files),
            ..args
        });
    }

    // Otherwise error out
    Err(AnalysisError::PathResolution(format!(
        "Path does not exist: {}",
        path_str
    )))
}

fn run(cli: cli::Cli) -> Result<()> {
    match cli.command {
        Some(cli::Command::Scan(args)) => handle_scan_command(args),
        Some(cli::Command::Watch(args)) => handle_watch_command(args),
        Some(cli::Command::Detectors(args)) => handle_detectors_command(args),
        Some(cli::Command::Cache(args)) => handle_cache_command(args),
        Some(cli::Command::Completions(args)) => handle_completions_command(args),
        Some(cli::Command::Snapshot(args)) => handle_snapshot_command(args),
        Some(cli::Command::Diff(args)) => handle_diff_command(args),
        Some(cli::Command::Init(args)) => handle_init_command(args),
        None => handle_default_command(cli),
    }
}

fn handle_snapshot_command(args: cli::SnapshotArgs) -> Result<()> {
    archlint::commands::run_snapshot(args.output, args.include_commit, args.path)
}

fn handle_diff_command(args: cli::DiffArgs) -> Result<()> {
    let exit_code = archlint::commands::run_diff(
        args.baseline,
        args.current,
        args.explain,
        args.json,
        args.fail_on,
        args.path,
    )?;

    if exit_code != 0 {
        process::exit(exit_code);
    }
    Ok(())
}

fn handle_scan_command(args: ScanArgs) -> Result<()> {
    let args = resolve_scan_args(args)?;
    let start = Instant::now();
    let engine = engine::AnalysisEngine::new_with_args(args.clone())?;
    let config = engine.config.clone();
    let report = engine.run()?;

    write_report(&args, &report, &config, &engine.project_root)?;
    print_scan_results(&args, &report, &config, start);
    exit_with_code(&report)
}

fn handle_watch_command(args: cli::WatchArgs) -> Result<()> {
    let engine = engine::AnalysisEngine::new_with_args(args.scan.clone())?;
    let config = engine.config.clone();

    let debounce_ms = if args.debounce != 300 {
        args.debounce
    } else {
        config.watch.debounce_ms
    };

    let clear_screen = if args.clear {
        true
    } else {
        config.watch.clear_screen
    };

    let mut ignore_patterns = config.watch.ignore.clone();
    ignore_patterns.extend(args.ignore.clone());

    let extensions = archlint::args::SUPPORTED_EXTENSIONS
        .iter()
        .map(|&e| e.to_string())
        .collect();

    let watch_config = watch::WatchConfig {
        debounce_ms,
        ignore_patterns,
        clear_screen,
        extensions,
    };
    let watcher = watch::FileWatcher::new(args.scan.path.clone(), watch_config);
    let mut runner = watch::runner::WatchRunner::new(engine, clear_screen);

    runner.run(watcher)
}

fn handle_default_command(cli: cli::Cli) -> Result<()> {
    let args = resolve_scan_args(cli.to_scan_args())?;
    let start = Instant::now();
    let engine = engine::AnalysisEngine::new_with_args(args.clone())?;
    let config = engine.config.clone();
    let report = engine.run()?;

    write_report(&args, &report, &config, &engine.project_root)?;
    print_scan_results(&args, &report, &config, start);
    exit_with_code(&report)
}

fn write_report(
    args: &ScanArgs,
    report: &report::AnalysisReport,
    config: &config::Config,
    project_root: &Path,
) -> Result<()> {
    if args.report.is_some() {
        info!("{}  Generating report...", style("📝").dim());
    }
    report.write(
        args.report.as_deref(),
        args.output_format(),
        args.no_diagram,
        &config.scoring,
        Some(project_root),
    )?;

    if let Some(path) = &args.report {
        info!(
            "{}  Report written to: {}",
            style("✔").green(),
            style(path.display()).bold()
        );
    }
    Ok(())
}

fn print_scan_results(
    args: &ScanArgs,
    report: &report::AnalysisReport,
    config: &config::Config,
    start: Instant,
) {
    if !args.is_quiet() && args.output_format() == OutputFormat::Table {
        let duration = start.elapsed();
        info!(
            "\n{}  (in {:.2}s)",
            style("✅ Analysis complete!").green().bold(),
            duration.as_secs_f64()
        );

        let total_smells = report.smells.len();
        let total_score = report.total_score(&config.scoring);
        let grade = report.grade(&config.scoring);

        info!(
            "{} Total smells found: {} (Total Score: {} pts)",
            style("🔍").bold(),
            style(total_smells.to_string()).red().bold(),
            style(total_score.to_string()).yellow().bold()
        );

        let (grade_styled, emoji) = format_grade(&grade);
        let level_styled = format_grade_level(&grade);

        info!(
            "{} Architecture Quality: {} ({})",
            style(emoji).bold(),
            grade_styled,
            level_styled
        );
    }
}

fn format_grade(
    grade: &report::ArchitectureGrade,
) -> (console::StyledObject<String>, &'static str) {
    match grade.level {
        report::GradeLevel::Excellent | report::GradeLevel::Good => {
            (style(format!("{:.1}/10", grade.score)).green().bold(), "✨")
        }
        report::GradeLevel::Fair | report::GradeLevel::Moderate => (
            style(format!("{:.1}/10", grade.score)).yellow().bold(),
            "⚠️",
        ),
        report::GradeLevel::Poor | report::GradeLevel::Critical => {
            (style(format!("{:.1}/10", grade.score)).red().bold(), "🚨")
        }
    }
}

fn format_grade_level(grade: &report::ArchitectureGrade) -> console::StyledObject<String> {
    match grade.level {
        report::GradeLevel::Excellent | report::GradeLevel::Good => {
            style(grade.level.to_string()).green()
        }
        report::GradeLevel::Fair | report::GradeLevel::Moderate => {
            style(grade.level.to_string()).yellow()
        }
        report::GradeLevel::Poor | report::GradeLevel::Critical => {
            style(grade.level.to_string()).red()
        }
    }
}

fn exit_with_code(report: &report::AnalysisReport) -> Result<()> {
    let exit_code = determine_exit_code(report);
    if exit_code != 0 {
        process::exit(exit_code);
    }
    Ok(())
}

fn handle_detectors_command(args: cli::DetectorArgs) -> Result<()> {
    match args.command {
        cli::DetectorCommand::List => {
            let registry = detectors::DetectorRegistry::new();
            let all_detectors = registry.list_all();

            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("ID").add_attribute(Attribute::Bold),
                    Cell::new("NAME").add_attribute(Attribute::Bold),
                    Cell::new("DEFAULT").add_attribute(Attribute::Bold),
                    Cell::new("DESCRIPTION").add_attribute(Attribute::Bold),
                ]);

            for info in all_detectors {
                table.add_row(vec![
                    Cell::new(info.id).fg(Color::Cyan),
                    Cell::new(info.name),
                    if info.default_enabled {
                        Cell::new("✅ enabled").fg(Color::Green)
                    } else {
                        Cell::new("❌ disabled").fg(Color::DarkGrey)
                    },
                    Cell::new(info.description),
                ]);
            }

            println!(
                "\n{}\n{}",
                style("Available Architectural Smell Detectors")
                    .bold()
                    .underlined(),
                table
            );
            Ok(())
        }
    }
}

fn handle_cache_command(args: cli::CacheArgs) -> Result<()> {
    match args.command {
        cli::CacheCommand::Clear => {
            cache::AnalysisCache::clear(Path::new("."))?;
            info!("{}  Cache cleared successfully", style("✔").green());
            Ok(())
        }
    }
}

fn handle_completions_command(args: cli::CompletionsArgs) -> Result<()> {
    let mut cmd = cli::Cli::command();
    clap_complete::generate(args.shell, &mut cmd, "archlint", &mut std::io::stdout());
    Ok(())
}

fn handle_init_command(args: cli::InitArgs) -> Result<()> {
    #[cfg(feature = "cli")]
    cliclack::intro(style(" Archlint Initialization ").on_cyan().black().bold())
        .map_err(|e| anyhow::anyhow!("UI error: {}", e))?;

    let config_path = Path::new(".archlint.yaml");

    if config_path.exists() && !args.force {
        let msg = "Config already exists. Use --force to overwrite.";
        #[cfg(feature = "cli")]
        cliclack::outro_cancel(msg).map_err(|e| anyhow::anyhow!("UI error: {}", e))?;
        return Err(anyhow::anyhow!(msg).into());
    }

    let cwd = std::env::current_dir()?;
    let mut config = config::Config::default();

    // 1. Determine frameworks
    let selected_presets = if !args.presets.is_empty() {
        args.presets.clone()
    } else {
        let detected = FrameworkDetector::detect(&cwd);
        if args.no_interactive {
            detected
                .iter()
                .map(|f| f.as_preset_name().to_string())
                .collect()
        } else {
            prompt_framework_selection(detected)?
        }
    };

    // 2. Apply presets (set extends only, merge happens at runtime)
    if !selected_presets.is_empty() {
        config.extends = selected_presets.clone();
    }

    // 3. Write config
    let yaml = serde_yaml::to_string(&config)?;
    std::fs::write(config_path, yaml)?;

    let success_msg = format!("Created {}", style(".archlint.yaml").bold());
    #[cfg(feature = "cli")]
    cliclack::outro(style(&success_msg).green()).map_err(|e| anyhow::anyhow!("UI error: {}", e))?;
    #[cfg(not(feature = "cli"))]
    println!("✓ {}", success_msg);

    Ok(())
}

fn prompt_framework_selection(detected: Vec<Framework>) -> Result<Vec<String>> {
    #[cfg(feature = "cli")]
    {
        use archlint::framework::preset_loader::PresetLoader;

        let detected_names: Vec<String> = detected.iter().map(|f| f.as_preset_name()).collect();

        let mut options: Vec<(&str, String)> = Vec::new();
        for name in PresetLoader::get_all_builtin_names() {
            if let Some(yaml) = PresetLoader::get_builtin_yaml(name) {
                let label = if let Some(desc) = yaml.description {
                    desc
                } else {
                    yaml.name.clone()
                };
                options.push((name, label));
            }
        }
        options.sort_by_key(|(name, _)| *name);

        let mut multiselect = cliclack::multiselect("Select framework presets to include:");

        for (id, label) in &options {
            multiselect = multiselect.item(*id, label, "");
        }

        let selected: Vec<&str> = multiselect
            .initial_values(detected_names.iter().map(|s| s.as_str()).collect())
            .required(false)
            .interact()
            .map_err(|e| anyhow::anyhow!("Interaction error: {}", e))?;

        Ok(selected.into_iter().map(|s| s.to_string()).collect())
    }
    #[cfg(not(feature = "cli"))]
    {
        Ok(detected.iter().map(|f| f.as_preset_name()).collect())
    }
}
