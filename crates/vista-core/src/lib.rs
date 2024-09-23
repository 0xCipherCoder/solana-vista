use std::sync::Arc;
use tokio::sync::RwLock;
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

pub mod traits;
pub mod models;

use models::{AccountInfo, TransactionInfo};
use traits::{Storage, Ingestor};

#[derive(Debug, Error)]
pub enum IndexerError {
    #[error("Solana client error: {0}")]
    SolanaClientError(#[from] solana_client::client_error::ClientError),
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Ingestor error: {0}")]
    IngestorError(String),
}

pub struct Indexer {
    storage: Arc<dyn Storage>,
    ingestors: Vec<Box<dyn Ingestor>>,
    tracked_accounts: Arc<RwLock<Vec<Pubkey>>>,
    tracked_programs: Arc<RwLock<Vec<Pubkey>>>,
}

impl Indexer {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self {
            storage,
            ingestors: Vec::new(),
            tracked_accounts: Arc::new(RwLock::new(Vec::new())),
            tracked_programs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_ingestor(&mut self, ingestor: Box<dyn Ingestor>) {
        self.ingestors.push(ingestor);
    }

    pub async fn track_account(&self, pubkey: Pubkey) -> Result<(), IndexerError> {
        let mut accounts = self.tracked_accounts.write().await;
        if !accounts.contains(&pubkey) {
            accounts.push(pubkey);
            for ingestor in &self.ingestors {
                ingestor.track_account(pubkey).await?;
            }
        }
        Ok(())
    }

    pub async fn untrack_account(&self, pubkey: &Pubkey) -> Result<(), IndexerError> {
        let mut accounts = self.tracked_accounts.write().await;
        if let Some(index) = accounts.iter().position(|x| x == pubkey) {
            accounts.remove(index);
            for ingestor in &self.ingestors {
                ingestor.untrack_account(pubkey).await?;
            }
        }
        Ok(())
    }

    pub async fn track_program(&self, pubkey: Pubkey) -> Result<(), IndexerError> {
        let mut programs = self.tracked_programs.write().await;
        if !programs.contains(&pubkey) {
            programs.push(pubkey);
            for ingestor in &self.ingestors {
                ingestor.track_program(pubkey).await?;
            }
        }
        Ok(())
    }

    pub async fn untrack_program(&self, pubkey: &Pubkey) -> Result<(), IndexerError> {
        let mut programs = self.tracked_programs.write().await;
        if let Some(index) = programs.iter().position(|x| x == pubkey) {
            programs.remove(index);
            for ingestor in &self.ingestors {
                ingestor.untrack_program(pubkey).await?;
            }
        }
        Ok(())
    }

    pub async fn process_account(&self, account: AccountInfo) -> Result<(), IndexerError> {
        let accounts = self.tracked_accounts.read().await;
        let programs = self.tracked_programs.read().await;
        
        if accounts.contains(&account.pubkey) || programs.contains(&account.owner) {
            self.storage.store_account(account).await?;
        }
        Ok(())
    }

    pub async fn process_transaction(&self, transaction: TransactionInfo) -> Result<(), IndexerError> {
        self.storage.store_transaction(transaction).await?;
        Ok(())
    }

    pub async fn start(&self) -> Result<(), IndexerError> {
        for ingestor in &self.ingestors {
            ingestor.start(self.storage.clone(), self.tracked_accounts.clone(), self.tracked_programs.clone()).await
                .map_err(|e| IndexerError::IngestorError(e.to_string()))?;
        }
        Ok(())
    }
}