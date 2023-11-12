-- Add migration script here
CREATE TABLE daily_stats (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    chat_id BIGINT NOT NULL,
    message_count BIGINT NOT NULL,
    date TIMESTAMPTZ NOT NULL
);
