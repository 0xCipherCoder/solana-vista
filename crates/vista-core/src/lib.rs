use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use thiserror::Error;

pub mod traits;
pub mod models;
pub mod plugin_registry;
pub mod config;

pub use plugin_registry::RpcProviderRegistry;
pub use config::Config;

use traits::{Storage, RpcProvider};
use models::{AccountInfo, TransactionInfo};
use vista_anchor::AnchorParser;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("RPC error: {0}")]
    RpcError(String),
    #[error("Anchor error: {0}")]
    AnchorError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub struct Indexer {
    storage: Arc<dyn StoragePlugin>,
    provider_registry: Arc<RpcProviderRegistry>,
    tracked_accounts: Arc<RwLock<Vec<Pubkey>>>,
    tracked_programs: Arc<RwLock<Vec<Pubkey>>>,
    update_channel: mpsc::Sender<UpdateEvent>,
    anchor_parser: Arc<RwLock<AnchorParser>>,
}

pub enum UpdateEvent {
    AccountUpdate(AccountInfo),
    TransactionUpdate(TransactionInfo),
}

impl Indexer {
    pub fn new(storage: Arc<dyn StoragePlugin>, provider_registry: Arc<RpcProviderRegistry>) -> Self {
        let (tx, rx) = mpsc::channel(1000);
        let indexer = Self {
            storage,
            provider_registry,
            tracked_accounts: Arc::new(RwLock::new(Vec::new())),
            tracked_programs: Arc::new(RwLock::new(Vec::new())),
            update_channel: tx,
            anchor_parser: Arc::new(RwLock::new(AnchorParser::new())),
        };
        
        tokio::spawn(indexer.process_updates(rx));
        indexer
    }

    async fn process_updates(self: Arc<Self>, mut rx: mpsc::Receiver<UpdateEvent>) {
        while let Some(event) = rx.recv().await {
            match event {
                UpdateEvent::AccountUpdate(account_info) => {
                    if let Err(e) = self.process_account_update(&account_info).await {
                        eprintln!("Failed to process account update: {}", e);
                    }
                },
                UpdateEvent::TransactionUpdate(transaction_info) => {
                    if let Err(e) = self.storage.store_transaction(transaction_info).await {
                        eprintln!("Failed to store transaction update: {}", e);
                    }
                },
            }
        }
    }

    async fn process_account_update(&self, account_info: &AccountInfo) -> Result<(), IndexerError> {
        let programs = self.tracked_programs.read().await;
        if programs.contains(&account_info.owner) {
            let parser = self.anchor_parser.read().await;
            if let Ok(parsed_data) = parser.parse_account_data(
                &account_info.owner.to_string(),
                "account", // You might need to determine the actual account type
                &account_info.data
            ) {
                self.storage.store_parsed_account(&account_info.owner.to_string(), "account", &parsed_data).await?;
            } else {
                // Fall back to storing raw account data if parsing fails
                self.storage.store_account(account_info.clone()).await?;
            }
        } else {
            self.storage.store_account(account_info.clone()).await?;
        }
        Ok(())
    }

    pub async fn track_account(&self, pubkey: Pubkey) -> Result<(), IndexerError> {
        self.tracked_accounts.write().await.push(pubkey);
        for provider in self.provider_registry.get_providers() {
            provider.subscribe_account_updates(&pubkey).await
                .map_err(|e| IndexerError::RpcError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn track_program(&self, pubkey: Pubkey) -> Result<(), IndexerError> {
        self.tracked_programs.write().await.push(pubkey);
        for provider in self.provider_registry.get_providers() {
            provider.subscribe_program_updates(&pubkey).await
                .map_err(|e| IndexerError::RpcError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn add_program_idl(&self, program_id: &str, idl_json: &str) -> Result<(), IndexerError> {
        let mut parser = self.anchor_parser.write().await;
        parser.add_idl(program_id, idl_json)
            .map_err(|e| IndexerError::AnchorError(e.to_string()))
    }

    pub fn get_update_channel(&self) -> mpsc::Sender<UpdateEvent> {
        self.update_channel.clone()
    }

    pub async fn start(&self) -> Result<(), IndexerError> {
        for provider in self.provider_registry.get_providers() {
            provider.start().await
                .map_err(|e| IndexerError::RpcError(e.to_string()))?;
        }
        Ok(())
    }
}