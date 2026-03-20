mod cli;
mod models;
mod organizer;

use clap::Parser;
use colored::Colorize;

use cli::{Args, OutputFormat};
use models::RunReport;
use organizer::{build_plan, execute_plan};

fn emit_report(format: OutputFormat, report: &RunReport) -> anyhow::Result<()> {
    match format {
        OutputFormat::Text => {
            if report.action_count == 0 {
                println!(
                    "{}",
                    "Nothing to organize — all files are in place.".dimmed()
                );
                return Ok(());
            }

            println!("{}", "Done.".green().bold());

            if report.summary.copied_across_filesystems > 0 {
                println!(
                    "{}",
                    format!(
                        "Used copy-and-delete fallback for {} file(s) moved across filesystems.",
                        report.summary.copied_across_filesystems
                    )
                    .dimmed()
                );
            }

            Ok(())
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(report)?);
            Ok(())
        }
    }
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();
    let plan = build_plan(&args)?;

    if matches!(args.format, OutputFormat::Text) {
        println!(
            "{}",
            format!("Organizing files in: {}", args.target_dir.display()).bold()
        );

        if !plan.is_empty() {
            println!(
                "{}",
                format!("Found {} file(s) to organize.", plan.len()).cyan()
            );
        }
    }

    let report = execute_plan(&args, plan)?;
    emit_report(args.format, &report)
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {err:?}", "Error:".red().bold());
        std::process::exit(1);
    }
}
