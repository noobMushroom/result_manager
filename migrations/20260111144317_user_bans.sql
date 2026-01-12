-- Add migration script here
CREATE TABLE user_bans (
    phone_number text PRIMARY KEY,
    banned_until timestamptz NOT NULL,
    reason text NOT NULL
);
