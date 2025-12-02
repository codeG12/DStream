use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dstreams")]
#[command(about = "DStream ETL - Extract, Transform, Load data streams", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Discover {
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,

        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    Sync {
        #[arg(long, value_name = "FILE")]
        tap_config: PathBuf,

        #[arg(long, value_name = "FILE")]
        target_config: PathBuf,

        #[arg(long, value_name = "FILE")]
        catalog: Option<PathBuf>,

        #[arg(long, value_name = "FILE")]
        state: Option<PathBuf>,
    },

    Tap {
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,

        #[arg(long, value_name = "FILE")]
        catalog: Option<PathBuf>,

        #[arg(long, value_name = "FILE")]
        state: Option<PathBuf>,

        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    Target {
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,

        #[arg(short, long, value_name = "FILE")]
        input: Option<PathBuf>,

        #[arg(long, value_name = "FILE")]
        state: Option<PathBuf>,
    },

    State {
        #[command(subcommand)]
        action: StateAction,
    },

    Catalog {
        #[command(subcommand)]
        action: CatalogAction,
    },
}

#[derive(Subcommand)]
pub enum StateAction {
    View {
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },

    Clear {
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },

    Set {
        #[arg(value_name = "FILE")]
        path: PathBuf,

        #[arg(value_name = "STREAM")]
        stream: String,

        #[arg(value_name = "VALUE")]
        value: String,
    },
}

#[derive(Subcommand)]
pub enum CatalogAction {
    View {
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },

    Select {
        #[arg(value_name = "FILE")]
        path: PathBuf,

        #[arg(value_name = "STREAMS")]
        streams: Vec<String>,
    },

    Deselect {
        #[arg(value_name = "FILE")]
        path: PathBuf,

        #[arg(value_name = "STREAMS")]
        streams: Vec<String>,
    },
}
