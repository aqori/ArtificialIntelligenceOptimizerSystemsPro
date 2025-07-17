// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizerSystemsPro
 */

use clap::Parser;
use artificialintelligenceoptimizersystemspro::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizerSystemsPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
