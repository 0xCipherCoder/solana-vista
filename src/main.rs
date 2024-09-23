use vista_core::{Config, RpcProviderRegistry};
use vista_ingestion::providers::{GeyserRpcProvider, WebSocketRpcProvider, HttpRpcProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("config.json")?;
    let mut provider_registry = RpcProviderRegistry::new();

    // Load plugins
    provider_registry.load_plugins(&config)?;

    // Register built-in providers
    for (name, provider_config) in &config.providers {
        match provider_config.provider_type.as_str() {
            "geyser" => provider_registry.register_provider(Box::new(GeyserRpcProvider::new(provider_config.url.clone()))),
            "websocket" => provider_registry.register_provider(Box::new(WebSocketRpcProvider::new(provider_config.url.clone()))),
            "http" => provider_registry.register_provider(Box::new(HttpRpcProvider::new(provider_config.url.clone()))),
            _ => println!("Unknown provider type: {}", provider_config.provider_type),
        }
    }

    // Create indexer and add ingestor
    let indexer = Indexer::new(Arc::new(provider_registry));
    
    // Start indexing
    indexer.start().await?;

    Ok(())
}