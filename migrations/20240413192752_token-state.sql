-- Add migration script here
ALTER TABLE tokens
ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT false;