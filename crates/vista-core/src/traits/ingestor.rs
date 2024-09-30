use async_trait::async_trait;
use std::sync::Arc;
use crate::plugin_registry::RpcProviderRegistry;

#[async_trait]
pub trait Ingestor: Send + Sync {
    async fn start(&self, provider_registry: Arc<RpcProviderRegistry>) -> Result<(), IndexerError>;
    // ... other methods ...
}