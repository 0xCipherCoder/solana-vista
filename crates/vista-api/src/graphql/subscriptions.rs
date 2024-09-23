use async_graphql::{Context, Subscription};
use futures::Stream;
use vista_core::{AccountInfo, Pubkey, TransactionInfo};
use tokio::sync::broadcast;
use super::schema::{Account, Transaction};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn account_updates(&self, ctx: &Context<'_>, pubkey: String) -> impl Stream<Item = Account> {
        let pubkey = Pubkey::from_str(&pubkey).expect("Invalid pubkey");
        let mut receiver = ctx.data::<broadcast::Sender<AccountInfo>>().unwrap().subscribe();
        
        async_stream::stream! {
            while let Ok(account_info) = receiver.recv().await {
                if account_info.pubkey == pubkey {
                    yield Account::from(account_info);
                }
            }
        }
    }

    async fn transaction_updates(&self, ctx: &Context<'_>) -> impl Stream<Item = Transaction> {
        let mut receiver = ctx.data::<broadcast::Sender<TransactionInfo>>().unwrap().subscribe();
        
        async_stream::stream! {
            while let Ok(transaction_info) = receiver.recv().await {
                yield Transaction::from(transaction_info);
            }
        }
    }
}