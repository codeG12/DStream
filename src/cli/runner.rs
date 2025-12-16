use crate::cli::commands::{CatalogAction, Commands, StateAction};
use crate::core::catalog::Catalog;
use crate::core::config::{TapConfig, TargetConfig};
use crate::core::state::StateManager;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub async fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Discover { config, output } => run_discover(&config, output.as_deref()).await,
        Commands::Sync {
            tap_config,
            target_config,
            catalog,
            state,
        } => {
            run_sync(
                &tap_config,
                &target_config,
                catalog.as_deref(),
                state.as_deref(),
            )
            .await
        }
        Commands::Tap {
            config,
            catalog,
            state,
            output,
        } => {
            run_tap(
                &config,
                catalog.as_deref(),
                state.as_deref(),
                output.as_deref(),
            )
            .await
        }
        Commands::Target {
            config,
            input,
            state,
        } => run_target(&config, input.as_deref(), state.as_deref()).await,
        Commands::State { action } => run_state_action(action).await,
        Commands::Catalog { action } => run_catalog_action(action).await,
    }
}

async fn run_discover(config_path: &Path, output_path: Option<&Path>) -> Result<()> {
    let config = TapConfig::from_file(config_path).context("Failed to load tap configuration")?;

    config.validate().context("Invalid tap configuration")?;

    tracing::info!("Running discovery for tap: {}", config.name);
    tracing::warn!("Discovery implementation pending - tap connectors need to be implemented");

    let catalog = Catalog::new();
    let output = output_path.unwrap_or_else(|| Path::new("catalog.json"));

    fs::write(output, catalog.to_json()?)?;
    tracing::info!("Catalog written to: {}", output.display());

    Ok(())
}

async fn run_sync(
    tap_config_path: &Path,
    target_config_path: &Path,
    catalog_path: Option<&Path>,
    state_path: Option<&Path>,
) -> Result<()> {
    let tap_config = TapConfig::from_file(tap_config_path)?;
    let target_config = TargetConfig::from_file(target_config_path)?;

    tap_config.validate()?;
    target_config.validate()?;

    let catalog = if let Some(path) = catalog_path {
        let json = fs::read_to_string(path)?;
        Catalog::from_json(&json)?
    } else {
        Catalog::new()
    };

    let mut state_manager = if let Some(path) = state_path {
        let mut mgr = StateManager::new(path);
        mgr.load()?;
        mgr
    } else {
        StateManager::new("state.json")
    };

    tracing::info!(
        "Starting sync: {} -> {}",
        tap_config.name,
        target_config.name
    );
    tracing::info!("Selected streams: {}", catalog.selected_streams().len());

    tracing::warn!(
        "Sync implementation pending - tap and target connectors need to be implemented"
    );

    state_manager.save()?;

    Ok(())
}

async fn run_tap(
    config_path: &Path,
    catalog_path: Option<&Path>,
    state_path: Option<&Path>,
    output_path: Option<&Path>,
) -> Result<()> {
    let config = TapConfig::from_file(config_path)?;
    config.validate()?;

    let catalog = if let Some(path) = catalog_path {
        let json = fs::read_to_string(path)?;
        Catalog::from_json(&json)?
    } else {
        Catalog::new()
    };

    let mut state_manager = if let Some(path) = state_path {
        let mut mgr = StateManager::new(path);
        mgr.load()?;
        mgr
    } else {
        StateManager::new("state.json")
    };

    tracing::info!("Running tap: {}", config.name);
    tracing::info!("Selected streams: {}", catalog.selected_streams().len());

    if let Some(output) = output_path {
        tracing::info!("Output will be written to: {}", output.display());
    } else {
        tracing::info!("Output will be written to stdout");
    }

    tracing::warn!("Tap implementation pending - tap connectors need to be implemented");

    state_manager.save()?;

    Ok(())
}

async fn run_target(
    config_path: &Path,
    input_path: Option<&Path>,
    state_path: Option<&Path>,
) -> Result<()> {
    let config = TargetConfig::from_file(config_path)?;
    config.validate()?;

    let mut state_manager = if let Some(path) = state_path {
        let mut mgr = StateManager::new(path);
        mgr.load()?;
        mgr
    } else {
        StateManager::new("state.json")
    };

    tracing::info!("Running target: {}", config.name);

    if let Some(input) = input_path {
        tracing::info!("Reading input from: {}", input.display());
    } else {
        tracing::info!("Reading input from stdin");
    }

    tracing::warn!("Target implementation pending - target connectors need to be implemented");

    state_manager.save()?;

    Ok(())
}

async fn run_state_action(action: StateAction) -> Result<()> {
    match action {
        StateAction::View { path } => {
            let mut manager = StateManager::new(&path);
            manager.load()?;

            let state = manager.get_state();
            let json = serde_json::to_string_pretty(state)?;
            println!("{}", json);

            Ok(())
        }
        StateAction::Clear { path } => {
            let mut manager = StateManager::new(&path);
            manager.load()?;
            manager.clear();
            manager.save()?;

            tracing::info!("State cleared: {}", path.display());
            Ok(())
        }
        StateAction::Set {
            path,
            stream,
            value,
        } => {
            let mut manager = StateManager::new(&path);
            manager.load()?;

            let value_json: serde_json::Value =
                serde_json::from_str(&value).context("Value must be valid JSON")?;

            manager.set_bookmark(stream.clone(), value_json);
            manager.save()?;

            tracing::info!("Bookmark set for stream: {}", stream);
            Ok(())
        }
    }
}

async fn run_catalog_action(action: CatalogAction) -> Result<()> {
    match action {
        CatalogAction::View { path } => {
            let json = fs::read_to_string(&path)?;
            let catalog = Catalog::from_json(&json)?;

            println!("Catalog: {} streams", catalog.streams.len());
            println!("\nStreams:");
            for entry in &catalog.streams {
                let status = if entry.selected { "✓" } else { " " };
                println!(
                    "  [{}] {} ({})",
                    status,
                    entry.stream,
                    format!("{:?}", entry.replication_method)
                );
            }

            Ok(())
        }
        CatalogAction::Select { path, streams } => {
            let json = fs::read_to_string(&path)?;
            let mut catalog = Catalog::from_json(&json)?;

            for stream_name in &streams {
                if let Some(entry) = catalog
                    .streams
                    .iter_mut()
                    .find(|e| &e.stream == stream_name)
                {
                    entry.selected = true;
                    tracing::info!("Selected stream: {}", stream_name);
                } else {
                    tracing::warn!("Stream not found: {}", stream_name);
                }
            }

            fs::write(&path, catalog.to_json()?)?;
            tracing::info!("Catalog updated: {}", path.display());

            Ok(())
        }
        CatalogAction::Deselect { path, streams } => {
            let json = fs::read_to_string(&path)?;
            let mut catalog = Catalog::from_json(&json)?;

            for stream_name in &streams {
                if let Some(entry) = catalog
                    .streams
                    .iter_mut()
                    .find(|e| &e.stream == stream_name)
                {
                    entry.selected = false;
                    tracing::info!("Deselected stream: {}", stream_name);
                } else {
                    tracing::warn!("Stream not found: {}", stream_name);
                }
            }

            fs::write(&path, catalog.to_json()?)?;
            tracing::info!("Catalog updated: {}", path.display());

            Ok(())
        }
    }
}
