use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use solana_sdk::pubkey::Pubkey;
use crate::{Storage, IndexerError, models::{AccountInfo, TransactionInfo}};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn store_account(&self, account: AccountInfo) -> Result<(), IndexerError>;
    async fn store_transaction(&self, transaction: TransactionInfo) -> Result<(), IndexerError>;
    async fn get_account(&self, pubkey: &Pubkey) -> Result<Option<AccountInfo>, IndexerError>;
    async fn get_transaction(&self, signature: &Pubkey) -> Result<Option<TransactionInfo>, IndexerError>;
}

#[async_trait]
pub trait Ingestor: Send + Sync {
    async fn start(
        &self,
        storage: Arc<dyn Storage>,
        tracked_accounts: Arc<RwLock<Vec<Pubkey>>>,
        tracked_programs: Arc<RwLock<Vec<Pubkey>>>,
    ) -> Result<(), IndexerError>;

    async fn track_account(&self, pubkey: Pubkey) -> Result<(), IndexerError>;
    async fn untrack_account(&self, pubkey: &Pubkey) -> Result<(), IndexerError>;
    async fn track_program(&self, pubkey: Pubkey) -> Result<(), IndexerError>;
    async fn untrack_program(&self, pubkey: &Pubkey) -> Result<(), IndexerError>;
}