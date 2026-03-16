mod cli;
mod models;
mod organizer;

use clap::Parser;
use colored::Colorize;

use cli::Args;
use organizer::{build_plan, execute_plan};

fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    println!(
        "{}",
        format!("Organizing files in: {}", args.target_dir.display()).bold()
    );

    let plan = build_plan(&args)?;

    if plan.is_empty() {
        println!(
            "{}",
            "Nothing to organize — all files are in place.".dimmed()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Found {} file(s) to organize.", plan.len()).cyan()
    );

    execute_plan(plan, args.dry_run)?;

    println!("{}", "Done.".green().bold());
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {err:?}", "Error:".red().bold());
        std::process::exit(1);
    }
}
