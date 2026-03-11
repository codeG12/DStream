mod client;
mod config;
mod entities;
mod tap;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::AcumaticaConfig;

/// tap-acumatica — Extract data from Acumatica ERP via its REST API.
#[derive(Parser)]
#[command(name = "tap-acumatica", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose / debug logging
    #[arg(long, short, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the catalog of available streams (no connection needed).
    Discover,

    /// Sync data from Acumatica and output JSONL to stdout.
    Sync {
        /// Path to the JSON config file.
        #[arg(short, long)]
        config: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialise tracing
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .with_target(false)
        .with_writer(std::io::stderr) // logs go to stderr, data to stdout
        .init();

    match cli.command {
        Commands::Discover => {
            tap::discover()?;
        }
        Commands::Sync { config } => {
            let cfg = AcumaticaConfig::from_file(&config)?;
            tap::sync(cfg)?;
        }
    }

    Ok(())
}
