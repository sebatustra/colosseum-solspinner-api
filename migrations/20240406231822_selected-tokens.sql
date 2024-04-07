-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS selected_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mint_pubkey VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    logo_url VARCHAR(255) NOT NULL,
    price_change_24h_percent DOUBLE PRECISION NOT NULL,
    volume_24h_usd DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);