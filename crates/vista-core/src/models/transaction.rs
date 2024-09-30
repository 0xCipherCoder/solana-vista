use solana_sdk::signature::Signature;
use solana_transaction_status::TransactionStatus;

#[derive(Debug, Clone)]
pub struct TransactionInfo {
    pub signature: Signature,
    pub status: TransactionStatus,
}