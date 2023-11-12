-- Add migration script here
CREATE TABLE chats (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    title TEXT NOT NULL
);
