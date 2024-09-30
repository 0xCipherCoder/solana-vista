CREATE TABLE IF NOT EXISTS accounts (
    pubkey TEXT PRIMARY KEY,
    lamports BIGINT NOT NULL,
    owner TEXT NOT NULL,
    data TEXT NOT NULL
);