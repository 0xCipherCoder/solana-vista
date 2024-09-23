use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::TransactionStatus;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexerError {
    #[error("Solana client error: {0}")]
    SolanaClientError(#[from] solana_client::client_error::ClientError),
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),
    #[error("Storage error: {0}")]
    StorageError(String),
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub owner: Pubkey,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TransactionInfo {
    pub signature: Signature,
    pub status: TransactionStatus,
}

pub trait IndexerStorage: Send + Sync {
    async fn store_account(&self, account: AccountInfo) -> Result<(), IndexerError>;
    async fn store_transaction(&self, transaction: TransactionInfo) -> Result<(), IndexerError>;
    async fn get_account(&self, pubkey: &Pubkey) -> Result<Option<AccountInfo>, IndexerError>;
    async fn get_transaction(&self, signature: &Signature) -> Result<Option<TransactionInfo>, IndexerError>;
}

pub struct Indexer<S: IndexerStorage> {
    storage: S,
    tracked_accounts: HashMap<Pubkey, ()>,
    tracked_programs: HashMap<Pubkey, ()>,
}

impl<S: IndexerStorage> Indexer<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            tracked_accounts: HashMap::new(),
            tracked_programs: HashMap::new(),
        }
    }

    pub fn track_account(&mut self, pubkey: Pubkey) {
        self.tracked_accounts.insert(pubkey, ());
    }

    pub fn track_program(&mut self, pubkey: Pubkey) {
        self.tracked_programs.insert(pubkey, ());
    }

    pub async fn process_account(&self, account: AccountInfo) -> Result<(), IndexerError> {
        if self.tracked_accounts.contains_key(&account.pubkey) || self.tracked_programs.contains_key(&account.owner) {
            self.storage.store_account(account).await?;
        }
        Ok(())
    }

    pub async fn process_transaction(&self, transaction: TransactionInfo) -> Result<(), IndexerError> {
        self.storage.store_transaction(transaction).await
    }
}