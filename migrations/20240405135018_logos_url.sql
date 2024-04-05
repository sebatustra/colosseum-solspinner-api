-- Add migration script here

-- Adding a new column to the tokens table
ALTER TABLE tokens
ADD COLUMN logo_url VARCHAR(255) NOT NULL;

-- Adding new columns to the positions table
ALTER TABLE positions
ADD COLUMN token_logo_url VARCHAR(255) NOT NULL,
ADD COLUMN vs_token_logo_url VARCHAR(255) NOT NULL,
ADD COLUMN vs_token_pubkey VARCHAR(255) NOT NULL;

-- Renaming columns in the positions table
ALTER TABLE positions
RENAME COLUMN mint_pubkey TO token_pubkey;

ALTER TABLE positions
RENAME COLUMN mint_symbol TO token_symbol;