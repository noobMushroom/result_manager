-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE teachers(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    phone_no      TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL,
    role          TEXT        NOT NULL 
);
