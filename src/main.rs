// src/main.rs
/*
 * Main executable for FutureMotion
 */

use clap::Parser;
use futuremotion::{Result, run};

#[derive(Parser)]
#[command(version, about = "FutureMotion - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
