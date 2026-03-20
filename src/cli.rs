use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// A bulk file organizer that sorts files into subdirectories by extension.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The directory to organize
    #[arg(short, long, default_value = ".")]
    pub target_dir: PathBuf,

    /// Print what would happen without actually moving files
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,

    /// Recurse into subdirectories
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,

    /// Output format for summaries and run reports
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}
