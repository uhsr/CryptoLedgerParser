// src/main.rs
/*
 * Main executable for CryptoLedgerParser
 */

use clap::Parser;
use cryptoledgerparser::{Result, run};

#[derive(Parser)]
#[command(version, about = "CryptoLedgerParser - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
