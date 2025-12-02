use clap::Parser;
use dstreams::cli::{Cli, run};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("dstreams=debug,info")
    } else {
        EnvFilter::new("dstreams=info")
    };

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    if let Err(e) = run(cli.command).await {
        tracing::error!("Error: {:#}", e);
        std::process::exit(1);
    }
}
