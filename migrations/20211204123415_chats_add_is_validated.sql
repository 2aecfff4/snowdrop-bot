-- Add migration script here
ALTER TABLE users ADD is_validated BOOLEAN NOT NULL DEFAULT FALSE
