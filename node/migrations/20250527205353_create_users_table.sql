-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE rewards (
    id SERIAL PRIMARY KEY,
    recipient_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    block_unlocked_at BIGINT NOT NULL
);