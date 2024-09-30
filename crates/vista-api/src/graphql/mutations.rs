use async_graphql::{Context, Object};
use vista_core::{Indexer, IndexerStorage, Pubkey};
use std::sync::Arc;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn track_account(&self, ctx: &Context<'_>, pubkey: String) -> async_graphql::Result<bool> {
        let indexer = ctx.data::<Arc<Indexer<dyn IndexerStorage>>>()?;
        let pubkey = Pubkey::from_str(&pubkey)?;
        indexer.track_account(pubkey);
        Ok(true)
    }

    async fn track_program(&self, ctx: &Context<'_>, pubkey: String) -> async_graphql::Result<bool> {
        let indexer = ctx.data::<Arc<Indexer<dyn IndexerStorage>>>()?;
        let pubkey = Pubkey::from_str(&pubkey)?;
        indexer.track_program(pubkey);
        Ok(true)
    }
}