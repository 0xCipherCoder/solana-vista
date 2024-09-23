use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use vista_core::traits::{Ingestor, Storage};
use vista_core::models::{AccountInfo, TransactionInfo};
use vista_core::IndexerError;
use async_trait::async_trait;

pub struct RpcIngestor {
    rpc_client: RpcClient,
    poll_interval: Duration,
}

impl RpcIngestor {
    pub fn new(rpc_url: &str, poll_interval: Duration) -> Self {
        let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        Self {
            rpc_client,
            poll_interval,
        }
    }

    async fn fetch_and_process_account(
        &self,
        pubkey: &Pubkey,
        storage: &Arc<dyn Storage>,
    ) -> Result<(), IndexerError> {
        match self.rpc_client.get_account(pubkey) {
            Ok(account) => {
                let account_info = AccountInfo {
                    pubkey: *pubkey,
                    lamports: account.lamports,
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                    data: account.data,
                };
                storage.store_account(account_info).await?;
            }
            Err(e) => {
                eprintln!("Failed to fetch account {}: {}", pubkey, e);
            }
        }
        Ok(())
    }

    async fn fetch_and_process_program_accounts(
        &self,
        program_id: &Pubkey,
        storage: &Arc<dyn Storage>,
    ) -> Result<(), IndexerError> {
        match self.rpc_client.get_program_accounts(program_id) {
            Ok(accounts) => {
                for (pubkey, account) in accounts {
                    let account_info = AccountInfo {
                        pubkey,
                        lamports: account.lamports,
                        owner: account.owner,
                        executable: account.executable,
                        rent_epoch: account.rent_epoch,
                        data: account.data,
                    };
                    storage.store_account(account_info).await?;
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch program accounts for {}: {}", program_id, e);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Ingestor for RpcIngestor {
    async fn start(
        &self,
        storage: Arc<dyn Storage>,
        tracked_accounts: Arc<RwLock<Vec<Pubkey>>>,
        tracked_programs: Arc<RwLock<Vec<Pubkey>>>,
    ) -> Result<(), IndexerError> {
        let mut interval = interval(self.poll_interval);

        loop {
            interval.tick().await;

            // Process tracked accounts
            let accounts = tracked_accounts.read().await;
            for account in accounts.iter() {
                self.fetch_and_process_account(account, &storage).await?;
            }

            // Process tracked programs
            let programs = tracked_programs.read().await;
            for program in programs.iter() {
                self.fetch_and_process_program_accounts(program, &storage).await?;
            }
        }
    }

    async fn track_account(&self, _pubkey: Pubkey) -> Result<(), IndexerError> {
        // No need to do anything here as we're using the shared tracked_accounts
        Ok(())
    }

    async fn untrack_account(&self, _pubkey: &Pubkey) -> Result<(), IndexerError> {
        // No need to do anything here as we're using the shared tracked_accounts
        Ok(())
    }

    async fn track_program(&self, _pubkey: Pubkey) -> Result<(), IndexerError> {
        // No need to do anything here as we're using the shared tracked_programs
        Ok(())
    }

    async fn untrack_program(&self, _pubkey: &Pubkey) -> Result<(), IndexerError> {
        // No need to do anything here as we're using the shared tracked_programs
        Ok(())
    }
}