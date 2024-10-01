CREATE TABLE IF NOT EXISTS accounts (
    pubkey TEXT PRIMARY KEY,
    lamports BIGINT NOT NULL,
    owner TEXT NOT NULL,
    executable BOOLEAN NOT NULL,
    rent_epoch BIGINT NOT NULL,
    data BYTEA NOT NULL
);