use std::sync::Arc;
use tokio::signal;
use vista_core::{Config, RpcProviderRegistry, Indexer, IndexerError};
use vista_storage::PostgresStorage;
use vista_ingestion::providers::{GeyserRpcProvider, WebSocketRpcProvider, HttpRpcProvider};
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_file("config.json")?;

     // Load storage plugin
     let storage_plugin = load_storage_plugin(&config.storage)?;
     storage_plugin.init(&config.storage.config).await?;

    // Initialize RPC provider registry
    let provider_registry = Arc::new(RpcProviderRegistry::new());

    // Create indexer
    let indexer = Arc::new(Indexer::new(Arc::new(storage_plugin), provider_registry));
    let update_channel = indexer.get_update_channel();

    // Register RPC providers
    if let Some(geyser_config) = config.providers.get("geyser") {
        let geyser_provider = GeyserRpcProvider::new(&geyser_config.url, update_channel.clone());
        provider_registry.register_provider(Box::new(geyser_provider));
    }

    if let Some(ws_config) = config.providers.get("websocket") {
        let ws_provider = WebSocketRpcProvider::new(&ws_config.url, update_channel.clone());
        provider_registry.register_provider(Box::new(ws_provider));
    }

    if let Some(http_config) = config.providers.get("http") {
        let http_provider = HttpRpcProvider::new(&http_config.url, update_channel.clone());
        provider_registry.register_provider(Box::new(http_provider));
    }

    // Load plugins
    provider_registry.load_plugins(&config).await?;

    // Add tracked accounts and programs
    for account in &config.tracked_accounts {
        let pubkey = Pubkey::from_str(account)?;
        indexer.track_account(pubkey).await?;
    }

    // Load IDLs and track programs
    for program in &config.tracked_programs {
        let pubkey = Pubkey::from_str(&program.address)?;
        indexer.track_program(pubkey).await?;

        let idl_json = std::fs::read_to_string(&program.idl_path)?;
        indexer.add_program_idl(&program.address, &idl_json).await?;
    }

    // Start the indexer
    indexer.start().await?;

    println!("SolanaVista indexer is running. Press Ctrl+C to stop.");

    // Wait for interrupt signal
    signal::ctrl_c().await?;

    println!("Shutting down indexer...");

    Ok(())
}

fn handle_error(error: IndexerError) {
    eprintln!("An error occurred: {}", error);
    // Implement any error handling logic here
}

fn load_storage_plugin(config: &StorageConfig) -> Result<Box<dyn StoragePlugin>, IndexerError> {
    let library = unsafe { Library::new(&config.plugin)? };
    let create_fn: Symbol<fn() -> Box<dyn StoragePlugin>> = unsafe {
        library.get(b"create_storage_plugin")?
    };
    Ok(create_fn())
}