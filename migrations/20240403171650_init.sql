-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    user_pubkey VARCHAR(255) PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tokens (
    mint_pubkey VARCHAR(255) PRIMARY KEY,
    symbol VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_pubkey VARCHAR(255) NOT NULL,
    mint_pubkey VARCHAR(255) NOT NULL,
    mint_symbol VARCHAR(255) NOT NULL,
    vs_token_symbol VARCHAR(255) NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    purchase_price DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_pubkey) REFERENCES users(user_pubkey),
    FOREIGN KEY (mint_pubkey) REFERENCES tokens(mint_pubkey)
);

CREATE TABLE IF NOT EXISTS tgbotusers (
    user_id BIGINT PRIMARY KEY,
    access_code TEXT,
    points INTEGER DEFAULT 3,
    referrals INTEGER DEFAULT 0,
    referred_by BIGINT
);