use async_trait::async_trait;
use crate::models::{AccountInfo, TransactionInfo};
use crate::IndexerError;
use serde_json::Value;

#[async_trait]
pub trait StoragePlugin: Send + Sync {
    fn name(&self) -> &str;
    async fn init(&self, config: &Value) -> Result<(), IndexerError>;
    async fn store_account(&self, account: AccountInfo) -> Result<(), IndexerError>;
    async fn store_transaction(&self, transaction: TransactionInfo) -> Result<(), IndexerError>;
    async fn get_account(&self, pubkey: &Pubkey) -> Result<Option<AccountInfo>, IndexerError>;
    async fn get_transaction(&self, signature: &Signature) -> Result<Option<TransactionInfo>, IndexerError>;
    async fn store_parsed_account(&self, program_id: &str, account_type: &str, data: &Value) -> Result<(), IndexerError>;
}