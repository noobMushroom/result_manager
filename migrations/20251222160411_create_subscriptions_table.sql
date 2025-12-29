-- Add migration script here
CREATE TABLE subscriptions
(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    phone_no      TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL
);
