# 0020 — Postgres migration format + provisional embeddings dimension

* Status: Accepted
* Date: 2026-07-21
* Deciders: @Siddharthgolecha

## Context and Problem Statement

`docs/adr/0016-redis-usage.md:18-21` and `docs/conventions/data.md:36-40`
both deferred Postgres migration naming, transaction boundaries, and
backwards-compat window "alongside slice 5, once there is a concrete
`init.sql` to react to." `infra/compose/postgres/init/001-extensions.sql`
is that file. Separately, the slice-4 diagrams spec
(`docs/architecture/overview.md:223-225`) commits to adding an
`embeddings`/pgvector table in this slice, but the real embedder model —
and thus the true vector dimension — is a slice-7 decision
(`docs/adr/0010-inference-runtime-worker-cuda-api-cpu.md`). Both
decisions need to land now so slice 6's Rust migration harness has a
format and a schema to implement against.

## Decision Drivers

* Slice 6 (Rust API) needs a locked migration naming/ordering
  convention before it can write its first real migration.
* The `embeddings` table cannot wait for slice 7's embedder choice —
  the diagrams spec already promised it in slice 5 — but the schema
  also shouldn't pretend to know a dimension it doesn't.
* No new dependency: `packages/contracts/schema.sql` stays a manual
  reference, not a codegen input (`scripts/gen-contracts.sh` reads
  only `events/` + `openapi/`).

## Considered Options

* **Sequential numeric migrations** (`0001_init.sql`, `0002_...sql`),
  applied in order, no down-migrations — simplest, matches
  `schema.sql`'s existing "no migration verbs" style.
* **Timestamped migrations** (`20260721120000_init.sql`) — avoids
  numbering collisions across parallel branches, more common in
  larger teams.
* **A migration framework** (e.g. `sqlx-cli`, `refinery`) — handles
  ordering/checksumming for free, but adds a new Rust dependency
  before slice 6 has decided its own dependency set.
* **Fixed `vector(512)` embeddings column now, revisit at slice 7**
  (chosen for the dimension question) vs. **deferring the
  `embeddings` table entirely to slice 6/7** (rejected — contradicts
  the already-merged slice-4 spec's forward reference).

## Decision Outcome

Chosen: **sequential numeric migrations, forward-only, no
down-migrations; `embeddings.embedding` is `vector(512)`, explicitly
provisional.**

* **Migration format:** `services/api/migrations/NNNN_description.sql`,
  four-digit zero-padded, monotonic, applied in order by slice 6's
  harness. No gaps, no renumbering — same convention as ADR numbering
  itself (`docs/adr/README.md:33-36`).
* **Up-only.** No down-migrations. Rolling back a bad migration means
  writing and applying a new forward migration that undoes it. Simpler
  to reason about with no production data yet, and avoids maintaining
  two directions of every schema change.
* **Backwards-compat window:** none required pre-launch (single
  deployable, no rolling upgrade yet). Once slice 9's deploy process
  exists, a migration that drops or renames a column in-use by the
  currently-running binary needs a two-step (add-then-drop-later)
  pattern — out of scope until that slice.
* **`embeddings` table** (`packages/contracts/schema.sql`): one row
  per indexed unit of media, `frame_index` expressing the one-to-many
  photo/video-keyframe relationship (ADR 0007) without a separate join
  table. `embedding vector(512)` — `512` is a placeholder consistent
  with common CLIP-family embedding sizes (ADR 0009's "small
  CLIP-family model" framing), **not** a measured value. Changing this
  dimension once slice 7 picks a real embedder is itself a forward
  migration (`ALTER TABLE embeddings ALTER COLUMN embedding TYPE
  vector(M)` plus a backfill/re-embed pass) — not a schema-format
  violation, just an expected follow-up migration.

## Consequences

* Positive: slice 6 has an unambiguous migration convention to
  implement (`services/api/migrations/0001_init.sql` mirroring
  `schema.sql`, including `embeddings`).
* Positive: the slice-4 diagrams spec's forward reference to
  "embeddings arrive with the slice-5 migration" is discharged.
* Negative: `vector(512)` may churn — slice 7 may need a different
  size, forcing a real migration (and, if any data was written by
  then, a backfill). Accepted: cheap before production data exists.
* Neutral: up-only migrations mean a bad migration is fixed by a new
  migration, not a revert — slightly more forward-motion discipline,
  no rollback tooling to maintain.
* Downstream: slice 6 implements `services/api/migrations/` per this
  format and applies the `embeddings` table as part of its first
  migration; slice 7 is the deciding slice for the real embedding
  dimension and will supersede the `vector(512)` placeholder via a
  follow-up migration when it lands.
