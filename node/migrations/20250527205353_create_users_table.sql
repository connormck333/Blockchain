-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    public_key TEXT NOT NULL,
    address TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0
);