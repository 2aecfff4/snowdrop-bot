-- Add migration script here
ALTER TABLE users DROP COLUMN is_validated;
ALTER TABLE chats ADD is_validated BOOLEAN NOT NULL DEFAULT FALSE;
