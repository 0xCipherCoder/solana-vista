use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};
use vista_core::traits::StoragePlugin;
use vista_core::models::{AccountInfo, TransactionInfo};
use vista_core::IndexerError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use serde_json::Value;
use std::str::FromStr;

pub struct PostgresStoragePlugin {
    pool: PgPool,
}

impl PostgresStoragePlugin {
    pub async fn new(config: &Value) -> Result<Self, IndexerError> {
        let database_url = config["url"].as_str()
            .ok_or_else(|| IndexerError::ConfigError("Missing database URL".to_string()))?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl StoragePlugin for PostgresStoragePlugin {
    fn name(&self) -> &str {
        "postgres"
    }

    async fn init(&self, _config: &Value) -> Result<(), IndexerError> {
        // Initialization is done in new(), so this is a no-op
        Ok(())
    }

    async fn store_account(&self, account: AccountInfo) -> Result<(), IndexerError> {
        sqlx::query!(
            r#"
            INSERT INTO accounts (pubkey, lamports, owner, executable, rent_epoch, data)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (pubkey) DO UPDATE
            SET lamports = $2, owner = $3, executable = $4, rent_epoch = $5, data = $6
            "#,
            account.pubkey.to_string(),
            account.lamports as i64,
            account.owner.to_string(),
            account.executable,
            account.rent_epoch as i64,
            account.data
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
            serde_json::to_value(transaction.status).map_err(|e| IndexerError::StorageError(e.to_string()))?
        )
        .execute(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn get_account(&self, pubkey: &Pubkey) -> Result<Option<AccountInfo>, IndexerError> {
        let result = sqlx::query!(
            r#"
            SELECT pubkey, lamports, owner, executable, rent_epoch, data
            FROM accounts
            WHERE pubkey = $1
            "#,
            pubkey.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        result.map(|row| {
            Ok(AccountInfo {
                pubkey: Pubkey::from_str(&row.pubkey).map_err(|e| IndexerError::StorageError(e.to_string()))?,
                lamports: row.lamports as u64,
                owner: Pubkey::from_str(&row.owner).map_err(|e| IndexerError::StorageError(e.to_string()))?,
                executable: row.executable,
                rent_epoch: row.rent_epoch as u64,
                data: row.data,
            })
        }).transpose()
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

        result.map(|row| {
            Ok(TransactionInfo {
                signature: Signature::from_str(&row.signature).map_err(|e| IndexerError::StorageError(e.to_string()))?,
                status: serde_json::from_value(row.status).map_err(|e| IndexerError::StorageError(e.to_string()))?,
            })
        }).transpose()
    }

    async fn store_parsed_account(&self, program_id: &str, account_type: &str, data: &Value) -> Result<(), IndexerError> {
        sqlx::query!(
            r#"
            INSERT INTO parsed_accounts (program_id, account_type, data)
            VALUES ($1, $2, $3)
            ON CONFLICT (program_id, account_type) DO UPDATE
            SET data = $3
            "#,
            program_id,
            account_type,
            data
        )
        .execute(&self.pool)
        .await
        .map_err(|e| IndexerError::StorageError(e.to_string()))?;

        Ok(())
    }
}

#[no_mangle]
pub fn create_storage_plugin() -> Box<dyn StoragePlugin> {
    Box::new(PostgresStoragePlugin::new())
}