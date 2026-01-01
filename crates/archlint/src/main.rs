use archlint::*;
use clap::Parser;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use console::style;
use log::{error, info};
use std::io::Write;
use std::process;
use std::time::Instant;

// Use mimalloc as global allocator for better performance
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let cli = cli::Cli::parse();

    let mut builder = env_logger::Builder::new();

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
        _ => {}
    }

    if std::env::var("RUST_LOG").is_err() {
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

    // Custom format for clean CLI output
    builder.format(|buf, record| match record.level() {
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
    });

    builder.init();

    if let Err(e) = run(cli) {
        error!("{}", e);
        process::exit(1);
    }
}

fn determine_exit_code(report: &report::AnalysisReport, config: &config::Config) -> i32 {
    let severity_config = &config.severity;

    let has_critical = report
        .smells
        .iter()
        .any(|(s, _)| s.effective_severity(severity_config) == detectors::Severity::Critical);

    let has_high = report
        .smells
        .iter()
        .any(|(s, _)| s.effective_severity(severity_config) == detectors::Severity::High);

    if has_critical {
        2 // Critical issues found
    } else if has_high {
        1 // High severity issues found
    } else {
        0 // Only medium/low or no issues
    }
}

fn run(cli: cli::Cli) -> Result<()> {
    match cli.command {
        Some(cli::Command::Scan(args)) => {
            let start = Instant::now();
            let engine = engine::AnalysisEngine::new(args.clone())?;
            let config = engine.config.clone();
            let report = engine.run()?;

            if args.report.is_some() {
                info!("{}  Generating report...", style("📝").dim());
            }
            report.write(
                args.report.as_deref(),
                args.output_format(),
                args.no_diagram,
                &config.severity,
                Some(&args.path),
            )?;

            if let Some(path) = &args.report {
                info!(
                    "{}  Report written to: {}",
                    style("✔").green(),
                    style(path.display()).bold()
                );
            }

            if !args.is_quiet() && args.output_format() == cli::OutputFormat::Table {
                let duration = start.elapsed();
                info!(
                    "\n{}  (in {:.2}s)",
                    style("✅ Analysis complete!").green().bold(),
                    duration.as_secs_f64()
                );

                let total_smells = report.smells.len();
                let total_score = report.total_score(&config.severity);
                let grade = report.grade(&config.severity);

                info!(
                    "{} Total smells found: {} (Total Score: {} pts)",
                    style("🔍").bold(),
                    style(total_smells.to_string()).red().bold(),
                    style(total_score.to_string()).yellow().bold()
                );

                let (grade_styled, emoji) = match grade.level {
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
                };

                let level_styled = match grade.level {
                    report::GradeLevel::Excellent | report::GradeLevel::Good => {
                        style(grade.level.to_string()).green()
                    }
                    report::GradeLevel::Fair | report::GradeLevel::Moderate => {
                        style(grade.level.to_string()).yellow()
                    }
                    report::GradeLevel::Poor | report::GradeLevel::Critical => {
                        style(grade.level.to_string()).red()
                    }
                };

                info!(
                    "{} Architecture Quality: {} ({})",
                    style(emoji).bold(),
                    grade_styled,
                    level_styled
                );
            }

            let exit_code = determine_exit_code(&report, &config);
            if exit_code != 0 {
                process::exit(exit_code);
            }
        }
        None => {
            let args = cli.to_scan_args();
            let start = Instant::now();
            let engine = engine::AnalysisEngine::new(args.clone())?;
            let config = engine.config.clone();
            let report = engine.run()?;

            if args.report.is_some() {
                info!("{}  Generating report...", style("📝").dim());
            }
            report.write(
                args.report.as_deref(),
                args.output_format(),
                args.no_diagram,
                &config.severity,
                Some(&args.path),
            )?;

            if let Some(path) = &args.report {
                info!(
                    "{}  Report written to: {}",
                    style("✔").green(),
                    style(path.display()).bold()
                );
            }

            if !args.is_quiet() && args.output_format() == cli::OutputFormat::Table {
                let duration = start.elapsed();
                info!(
                    "\n{}  (in {:.2}s)",
                    style("✅ Analysis complete!").green().bold(),
                    duration.as_secs_f64()
                );

                let total_smells = report.smells.len();
                let total_score = report.total_score(&config.severity);
                let grade = report.grade(&config.severity);

                info!(
                    "{} Total smells found: {} (Total Score: {} pts)",
                    style("🔍").bold(),
                    style(total_smells.to_string()).red().bold(),
                    style(total_score.to_string()).yellow().bold()
                );

                let (grade_styled, emoji) = match grade.level {
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
                };

                let level_styled = match grade.level {
                    report::GradeLevel::Excellent | report::GradeLevel::Good => {
                        style(grade.level.to_string()).green()
                    }
                    report::GradeLevel::Fair | report::GradeLevel::Moderate => {
                        style(grade.level.to_string()).yellow()
                    }
                    report::GradeLevel::Poor | report::GradeLevel::Critical => {
                        style(grade.level.to_string()).red()
                    }
                };

                info!(
                    "{} Architecture Quality: {} ({})",
                    style(emoji).bold(),
                    grade_styled,
                    level_styled
                );
            }

            let exit_code = determine_exit_code(&report, &config);
            if exit_code != 0 {
                process::exit(exit_code);
            }
        }
        Some(cli::Command::Watch(args)) => {
            let engine = engine::AnalysisEngine::new(args.scan.clone())?;
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

            let extensions = match args.scan.lang {
                cli::Language::TypeScript => vec!["ts".to_string(), "tsx".to_string()],
                cli::Language::JavaScript => vec!["js".to_string(), "jsx".to_string()],
            };

            let watch_config = watch::WatchConfig {
                debounce_ms,
                ignore_patterns,
                clear_screen,
                extensions,
            };
            let watcher = watch::FileWatcher::new(args.scan.path.clone(), watch_config);
            let mut runner = watch::runner::WatchRunner::new(engine, clear_screen);

            runner.run(watcher)?;
        }
        Some(cli::Command::Detectors(args)) => match args.command {
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
            }
        },
        Some(cli::Command::Cache(args)) => match args.command {
            cli::CacheCommand::Clear => {
                cache::AnalysisCache::clear(std::path::Path::new("."))?;
                info!("{}  Cache cleared successfully", style("✔").green());
            }
        },
    }

    Ok(())
}
