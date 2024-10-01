use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::error::IngestionError;

#[async_trait]
pub trait RpcProvider: Send + Sync {
    fn name(&self) -> &str;
    fn provider_type(&self) -> RpcProviderType;
    async fn subscribe_account(&self, pubkey: &Pubkey) -> Result<(), IngestionError>;
    async fn subscribe_program(&self, program_id: &Pubkey) -> Result<(), IngestionError>;
    async fn start(&self) -> Result<(), IngestionError>;
}

pub enum RpcProviderType {
    Geyser,
    WebSocket,
    Grpc,
    Http,
}