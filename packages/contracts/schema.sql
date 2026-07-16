-- Canonical Postgres reference for slice 3 contracts.
-- NOT an executed migration. slice 5 (arch-compose-and-infra) owns
-- the migration format per ADR 0016 (Postgres deferred) and will
-- implement this reference under services/api/migrations/.
--
-- No indexes, no migration verbs, no schema-version header — this
-- file is a review-time cross-check: if the SQL and the JSON schemas
-- under packages/contracts/events/ name different fields for the
-- same concept, that's the exact drift this file catches.
--
-- Tables covered:
--   ADR 0008:68-71 — events, event_memberships, share_tokens
--   ADR 0005      — accounts, sessions
--   ADR 0007      — media (with media_kind enum)
--   api.md:35-39  — idempotency_keys
--
-- ULIDs are stored as `text` at this reference level; slice 5 may
-- pick a binary encoding (e.g. bytea/uuid) — the wire schemas keep
-- the ULID string form regardless.

CREATE TABLE accounts (
    account_id text PRIMARY KEY,
    email text NOT NULL UNIQUE,
    display_name text NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE TABLE sessions (
    session_token text PRIMARY KEY,
    account_id text NOT NULL REFERENCES accounts(account_id),
    created_at timestamptz NOT NULL,
    expires_at timestamptz NOT NULL
);

CREATE TABLE events (
    event_id text PRIMARY KEY,
    owner_account_id text NOT NULL REFERENCES accounts(account_id),
    title text NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE TABLE event_memberships (
    event_id text NOT NULL REFERENCES events(event_id),
    account_id text NOT NULL REFERENCES accounts(account_id),
    role text NOT NULL CHECK (role IN ('owner', 'editor', 'reader')),
    created_at timestamptz NOT NULL,
    PRIMARY KEY (event_id, account_id)
);

CREATE TABLE share_tokens (
    share_token text PRIMARY KEY,
    event_id text NOT NULL REFERENCES events(event_id),
    revoked boolean NOT NULL DEFAULT false,
    expires_at timestamptz,
    created_at timestamptz NOT NULL
);

CREATE TABLE media (
    media_id text PRIMARY KEY,
    event_id text NOT NULL REFERENCES events(event_id),
    media_kind text NOT NULL CHECK (media_kind IN ('photo', 'video')),
    source_uri text NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE TABLE idempotency_keys (
    idempotency_key text NOT NULL,
    account_id text NOT NULL REFERENCES accounts(account_id),
    request_fingerprint text NOT NULL,
    response_body jsonb NOT NULL,
    response_status integer NOT NULL,
    created_at timestamptz NOT NULL,
    PRIMARY KEY (idempotency_key, account_id)
);
