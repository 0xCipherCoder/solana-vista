use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use std::sync::Arc;
use tokio::sync::broadcast;
use vista_core::{Indexer, IndexerStorage, AccountInfo, TransactionInfo};

use super::{queries::QueryRoot, mutations::MutationRoot, subscriptions::SubscriptionRoot};

pub type SolanaVistaSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema(
    indexer: Arc<Indexer<dyn IndexerStorage>>,
    account_sender: broadcast::Sender<AccountInfo>,
    transaction_sender: broadcast::Sender<TransactionInfo>,
) -> SolanaVistaSchema {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(indexer)
        .data(account_sender)
        .data(transaction_sender)
        .finish()
}