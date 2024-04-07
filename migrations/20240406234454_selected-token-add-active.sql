-- Add migration script here
ALTER TABLE selected_tokens
ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;