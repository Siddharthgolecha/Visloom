-- infra/compose/postgres/init/001-extensions.sql
--
-- Runs once, at first container init, before any migration harness
-- exists (slice 6 owns migrations; see docs/adr/0020-postgres-migration-format.md).
-- Enables the extension packages/contracts/schema.sql's `embeddings`
-- table depends on. Deliberately defines no app tables — schema is a
-- migration concern, not an init-script concern.

CREATE EXTENSION IF NOT EXISTS vector;
