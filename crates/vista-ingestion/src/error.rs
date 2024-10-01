use thiserror::Error;

#[derive(Error, Debug)]
pub enum IngestionError {
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
    #[error("Plugin load error: {0}")]
    PluginLoadError(String),
}