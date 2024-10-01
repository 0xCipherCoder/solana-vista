use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use crate::traits::{RpcProvider, RpcProviderType};
use crate::error::IngestionError;

pub struct BaseGeyserProvider {
    // Common Geyser fields
}

#[async_trait]
impl RpcProvider for BaseGeyserProvider {
    fn name(&self) -> &str {
        "base_geyser"
    }

    fn provider_type(&self) -> RpcProviderType {
        RpcProviderType::Geyser
    }

    async fn subscribe_account(&self, pubkey: &Pubkey) -> Result<(), IngestionError> {
        // Implement base Geyser account subscription
        todo!()
    }

    async fn subscribe_program(&self, program_id: &Pubkey) -> Result<(), IngestionError> {
        // Implement base Geyser program subscription
        todo!()
    }

    async fn start(&self) -> Result<(), IngestionError> {
        // Implement base Geyser start logic
        todo!()
    }
}