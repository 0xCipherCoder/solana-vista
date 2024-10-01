CREATE TABLE IF NOT EXISTS parsed_accounts (
    program_id TEXT NOT NULL,
    account_type TEXT NOT NULL,
    data JSONB NOT NULL,
    PRIMARY KEY (program_id, account_type)
);