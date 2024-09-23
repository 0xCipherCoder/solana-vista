use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use vista_core::{AccountInfo, Indexer, IndexerError, IndexerStorage};

pub struct RealTimeIngestion {
    rpc_client: RpcClient,
}

impl RealTimeIngestion {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_client: RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed()),
        }
    }

    pub async fn start<S: IndexerStorage>(&self, indexer: &Indexer<S>) -> Result<(), IndexerError> {
        // In a real implementation, you'd set up a websocket connection here
        // For the hackathon MVP, we'll use polling as a simple solution
        loop {
            for pubkey in indexer.tracked_accounts.keys() {
                let account = self.rpc_client.get_account(pubkey)?;
                let account_info = AccountInfo {
                    pubkey: *pubkey,
                    lamports: account.lamports,
                    owner: account.owner,
                    data: account.data,
                };
                indexer.process_account(account_info).await?;
            }

            // Sleep for a short duration to avoid hammering the RPC
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}