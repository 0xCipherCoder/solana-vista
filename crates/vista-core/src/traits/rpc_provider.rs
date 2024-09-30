use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::models::{AccountInfo, TransactionInfo};
use crate::IndexerError;

#[async_trait]
pub trait RpcProvider: Send + Sync {
    fn name(&self) -> &str;
    fn provider_type(&self) -> RpcProviderType;
    async fn get_account(&self, pubkey: &Pubkey) -> Result<Option<AccountInfo>, IndexerError>;
    async fn get_transaction(&self, signature: &Signature) -> Result<Option<TransactionInfo>, IndexerError>;
    async fn subscribe_account_updates(&self, pubkey: &Pubkey) -> Result<(), IndexerError>;
    async fn subscribe_program_updates(&self, program_id: &Pubkey) -> Result<(), IndexerError>;
    async fn process_updates(&self) -> Result<(), IndexerError>;
}

pub enum RpcProviderType {
    Geyser,
    WebSocket,
    Http,
}