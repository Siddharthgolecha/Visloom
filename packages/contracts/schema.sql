-- Canonical Postgres reference. NOT an executed migration —
-- slice 6 (services/api) owns the migration harness and implements
-- this reference under services/api/migrations/, per the migration
-- format locked in docs/adr/0020-postgres-migration-format.md.
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
--   ADR 0020      — embeddings (pgvector; slice 5)
--
-- ULIDs are stored as `text` at this reference level; slice 6 may
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

-- One row per indexed unit of media: a photo contributes one row
-- (frame_index = 0); a video contributes one row per extracted
-- keyframe (frame_index = 0..N-1), per ADR 0007's photo/video-keyframe
-- model. `embedding` is vector(512) — PROVISIONAL. The real embedder
-- model (and thus the true dimension) is a slice-7 decision per
-- ADR 0010; changing this dimension later is itself a migration, per
-- ADR 0020.
CREATE TABLE embeddings (
    embedding_id text PRIMARY KEY,
    media_id text NOT NULL REFERENCES media(media_id),
    frame_index integer NOT NULL CHECK (frame_index >= 0),
    embedding vector(512) NOT NULL,
    created_at timestamptz NOT NULL,
    UNIQUE (media_id, frame_index)
);
