use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};
use vista_core::{AccountInfo, IndexerError, IndexerStorage, Pubkey, Signature, TransactionInfo, TransactionStatus};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostgresStorageError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub async fn new(database_url: &str) -> Result<Self, PostgresStorageError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl IndexerStorage for PostgresStorage {
    async fn store_account(&self, account: AccountInfo) -> Result<(), IndexerError> {
        sqlx::query!(
            r#"
            INSERT INTO accounts (pubkey, lamports, owner, data)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (pubkey) DO UPDATE
            SET lamports = $2, owner = $3, data = $4
            "#,
            account.pubkey.to_string(),
            account.lamports as i64,
            account.owner.to_string(),
            base64::encode(&account.data)
        )
        .execute(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn store_transaction(&self, transaction: TransactionInfo) -> Result<(), IndexerError> {
        sqlx::query!(
            r#"
            INSERT INTO transactions (signature, status)
            VALUES ($1, $2)
            ON CONFLICT (signature) DO UPDATE
            SET status = $2
            "#,
            transaction.signature.to_string(),
            format!("{:?}", transaction.status)
        )
        .execute(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn get_account(&self, pubkey: &Pubkey) -> Result<Option<AccountInfo>, IndexerError> {
        let result = sqlx::query!(
            r#"
            SELECT pubkey, lamports, owner, data
            FROM accounts
            WHERE pubkey = $1
            "#,
            pubkey.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        match result {
            Some(row) => Ok(Some(AccountInfo {
                pubkey: Pubkey::from_str(&row.pubkey).map_err(|e| IndexerError::StorageError(e.to_string()))?,
                lamports: row.lamports as u64,
                owner: Pubkey::from_str(&row.owner).map_err(|e| IndexerError::StorageError(e.to_string()))?,
                data: base64::decode(&row.data).map_err(|e| IndexerError::StorageError(e.to_string()))?,
            })),
            None => Ok(None),
        }
    }

    async fn get_transaction(&self, signature: &Signature) -> Result<Option<TransactionInfo>, IndexerError> {
        let result = sqlx::query!(
            r#"
            SELECT signature, status
            FROM transactions
            WHERE signature = $1
            "#,
            signature.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        match result {
            Some(row) => Ok(Some(TransactionInfo {
                signature: Signature::from_str(&row.signature).map_err(|e| IndexerError::StorageError(e.to_string()))?,
                status: serde_json::from_str(&row.status).map_err(|e| IndexerError::StorageError(e.to_string()))?,
            })),
            None => Ok(None),
        }
    }
}