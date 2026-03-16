mod cli;
mod models;
mod organizer;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    println!("Target directory is: {:?}", args.target_dir);
}
