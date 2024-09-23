use async_graphql::{Context, Object};
use vista_core::{IndexerStorage, Pubkey, Signature};
use std::sync::Arc;
use super::schema::{Account, Transaction};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn account(&self, ctx: &Context<'_>, pubkey: String) -> async_graphql::Result<Option<Account>> {
        let storage = ctx.data::<Arc<dyn IndexerStorage>>()?;
        let pubkey = Pubkey::from_str(&pubkey)?;
        let account_info = storage.get_account(&pubkey).await?;
        Ok(account_info.map(Account::from))
    }

    async fn transaction(&self, ctx: &Context<'_>, signature: String) -> async_graphql::Result<Option<Transaction>> {
        let storage = ctx.data::<Arc<dyn IndexerStorage>>()?;
        let signature = Signature::from_str(&signature)?;
        let transaction_info = storage.get_transaction(&signature).await?;
        Ok(transaction_info.map(Transaction::from))
    }
}