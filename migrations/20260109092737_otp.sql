-- Add migration script here
CREATE TABLE otp_requests (
    id UUID PRIMARY KEY ,
    phone_number TEXT NOT NULL,
    otp_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    attempts INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL
);


CREATE INDEX idx_otp_phone_created
ON otp_requests (phone_number, created_at DESC);
