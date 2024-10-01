CREATE TABLE IF NOT EXISTS transactions (
    signature TEXT PRIMARY KEY,
    status JSONB NOT NULL
);