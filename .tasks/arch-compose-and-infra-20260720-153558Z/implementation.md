# Implementation — arch-compose-and-infra

<!-- Append-only log. Never delete past entries; strikethrough if reversed.
     Each entry cites the spec.md/plan.md section it implements and carries
     a trust-tier marker:
       - tested-against-real-input
       - tested-with-fixture
       - untested-assumption
       - inherited-from-existing-code -->

## Task tree

- [x] Step 1 — `infra/compose/postgres/init/001-extensions.sql`
  (`CREATE EXTENSION IF NOT EXISTS vector;`, no app tables) (plan §Tactical
  #1). `tested-against-real-input` — `\dx` inside the running container
  lists `vector`.
- [x] Step 2 — `embeddings` table added to `packages/contracts/schema.sql`
  (`vector(512)`, FK to `media`, `frame_index`), ADR 0020 written, index
  row added to `docs/adr/README.md`, `docs/conventions/data.md ## Postgres`
  repointed at ADR 0020 (plan §Tactical #2). `tested-against-real-input` for
  the schema (table exists and accepts a `vector(512)` insert shape verified
  via `\dx` + column DDL); `inherited-from-existing-code` for the ADR
  (mirrors ADR 0018's shape per spec `## Research findings`).
- [x] Step 3 — ran `make contracts` (`scripts/gen-contracts.sh`) after the
  `schema.sql` edit; `git status` on `packages/contracts/{ts,rust,py}` came
  back clean (plan §Tactical #3). `tested-against-real-input`.
- [x] Step 4 — `docs/architecture/overview.md`'s ER diagram gains the
  `embeddings` entity + relationship; prose updated from "seven tables...
  not shown" to "eight tables... including embeddings" with the
  provisional-dimension caveat (plan §Tactical #4). `tested-against-real-input`
  — entity set diffed against `schema.sql`'s `CREATE TABLE` names, both are
  the same 8: `accounts, sessions, events, event_memberships, share_tokens,
  media, idempotency_keys, embeddings`.
- [x] Step 5 — `infra/compose/redis/redis.conf` (dev: no persistence
  directives) (plan §Tactical #5). `tested-against-real-input` — `redis-cli
  ping` returns `PONG` against the running container.
- [x] Step 6 — `infra/compose/caddy/Caddyfile` (same-origin shape, upstreams
  commented, `respond "ok" 200` stub, `$SITE_ADDRESS` env-templated so one
  file serves both overlays) (plan §Tactical #6). `tested-against-real-input`
  — `curl localhost:8080/` returns `200`.
- [x] Step 7 — `infra/compose/otel/collector.yaml` (OTLP gRPC+HTTP
  receivers, `debug` exporter, traces/metrics/logs pipelines) (plan
  §Tactical #7). `tested-against-real-input` — collector log shows
  "Everything is ready. Begin running and processing data."
- [x] Step 8 — `infra/compose/compose.yml` (4 services: postgres/redis/
  caddy/otel-collector, named volumes, healthchecks) + `compose.prod.yml`
  (pinned `pgvector/pgvector:pg16`, `restart: unless-stopped`, Redis
  `--appendonly yes`, resource limits, TLS-capable `SITE_ADDRESS`) (plan
  §Tactical #8). `tested-against-real-input` — both overlays `config -q`
  clean; base overlay brought all 4 services to `running` within the 60s
  window.
- [x] Step 9 — `infra/compose/.env.example` (placeholder Postgres creds,
  `SITE_ADDRESS`, `OTEL_EXPORTER_OTLP_ENDPOINT`) (plan §Tactical #9).
  `tested-against-real-input` — manually reviewed, no live credential
  values.
- [x] Step 10 — `infra/models/README.md` placeholder (plan §Tactical #10).
  `tested-against-real-input` — file exists.
- [x] Step 11 — `.github/workflows/compose.yml`: `docker compose config -q`
  on both overlays, path-filtered to `infra/compose/**` + the workflow file
  itself (plan §Tactical #11). `tested-against-real-input` — the same two
  `config -q` invocations the workflow runs were run locally and both
  exited 0.
- [x] Step 12 — full verification pass: both overlays `config -q`; `up -d`
  → all 4 services `running` within 60s; `\dx` shows `vector`; Redis `ping`
  → `PONG`; Caddy `curl` → `200`; OTel collector readiness log present;
  `down -v` → exit 0, all 3 named volumes removed (plan §Tactical #12).
  `tested-against-real-input` — every command run directly against the
  live stack, not simulated.

## Deviations

None structural. `docker`/`docker compose` on this machine resolves through
a Podman-compose shim (`podman-compose`) rather than Docker Engine directly
— an environment detail, not a spec/plan change; the same `docker compose
-f infra/compose/compose.yml ...` invocations named in `spec.md`'s
Acceptance criteria and `plan.md` step 12 ran unmodified and produced the
exact output each falsifier requires.

**Known ceiling (spec `## Tradeoffs accepted`):** `vector(512)` is a
placeholder — ADR 0020 names slice 7 (the real embedder choice) as the
upgrade path, requiring a forward migration + backfill when it lands.

**Known ceiling (spec `## Tradeoffs accepted` / `## Failure modes`):**
`schema.sql` ↔ `overview.md` ER-diagram sync has no CI guard — this PR
hand-synced both, but a future edit to one without the other will silently
drift again. Same known gap as the diagrams slice; a standing guard is its
own slice, not proposed here.

**Post-ready review fix (2026-07-21).** Reviewer caught two real defects,
both inside files already in the frozen `## Files touched` list, fixed
non-structurally without touching `spec.md`/`plan.md`:

1. **Prod port leakage.** `compose.prod.yml` only overrode
   tags/limits/restart policy and never cleared the base overlay's
   `ports:` entries, so Compose's array-merge semantics meant the merged
   prod config still published Postgres `5432`, Redis `6379`, and the
   OTel collector's `4317`/`4318` alongside Caddy's `80`/`443` — exactly
   the internal-service exposure `## Approach`'s "same service defs,
   overlay flips tags/limits/volumes" pattern was meant to avoid stating
   explicitly. Fixed by adding `ports: !reset []` to `postgres`, `redis`,
   and `otel-collector` in `compose.prod.yml`, and `!override` on
   `caddy`'s (functionally a no-op there, but makes the intent explicit).
   Verified via `docker compose ... config --format json` on the merged
   overlay: only `caddy` reports a non-null `ports` list (`80`, `443`);
   the other three report `null`. `tested-against-real-input`.
2. **Missing `frame_index` lower bound.** `embeddings.frame_index` is
   documented in `schema.sql`'s comment as `0..N-1` but had no constraint
   enforcing it, so a negative value would silently sit under the
   `(media_id, frame_index)` unique key as a distinct row. Fixed by
   adding `CHECK (frame_index >= 0)` — same inline style already used by
   `event_memberships.role` and `media.media_kind` in this file. Verified
   against a live Postgres container: re-ran `scripts/gen-contracts.sh`
   after the edit (clean — contracts don't read `schema.sql`, per
   `## Failure modes`), applied the updated `schema.sql`, confirmed
   `frame_index = -1` raises `embeddings_frame_index_check` and
   `frame_index = 0` still inserts. `tested-against-real-input`.

A third reviewer finding — add `infra/compose/.env` to `.gitignore` — is
**not applied** here. `.gitignore` is outside the frozen `## Files
touched` list, and `.env.example`'s comment already documents this as a
known, deliberate slice-9 gap (dev-workflow tooling), not an oversight.
Adding it now would be a structural deviation under the plan-frozen rule
(AGENTS.md §5 step 7) — raised back to the human as an open question
rather than self-applied. See PR #16 discussion.

## Summary

Slice 5 of the arch-scaffold epic. Ships the four-service Compose stack
(Postgres+pgvector, Redis, Caddy, OTel collector) slices 6/7/8 plug into,
and discharges the two decisions slices 1–4 explicitly deferred here: the
Postgres migration format (ADR 0020) and the `embeddings`/pgvector table
the diagrams spec forward-referenced.

**Files (14):**

- `infra/compose/compose.yml` (new) — base overlay, 4 services only
  (postgres/redis/caddy/otel-collector), named volumes + healthchecks.
  `tested-against-real-input`.
- `infra/compose/compose.prod.yml` (new) — prod overlay: pinned pgvector
  image, `restart: unless-stopped`, Redis `appendonly`, resource limits.
  `tested-against-real-input` (config-validated; `up` only exercised
  against the base overlay per plan scope).
- `infra/compose/.env.example` (new) — placeholder env vars only.
  `tested-against-real-input`.
- `infra/compose/postgres/init/001-extensions.sql` (new) — pgvector
  extension bootstrap, no app tables. `tested-against-real-input`.
- `infra/compose/redis/redis.conf` (new) — dev persistence policy (none).
  `tested-against-real-input`.
- `infra/compose/caddy/Caddyfile` (new) — same-origin routing stub,
  upstreams commented, env-templated site address. `tested-against-real-input`.
- `infra/compose/otel/collector.yaml` (new) — OTLP receivers + debug
  exporter. `tested-against-real-input`.
- `infra/models/README.md` (new) — model-weights placeholder.
  `tested-against-real-input`.
- `packages/contracts/schema.sql` — adds `embeddings` table
  (`vector(512)`, provisional); header comment updated to cite ADR 0020.
  `tested-against-real-input`.
- `docs/architecture/overview.md` — ER diagram + prose synced to the new
  8-table set. `tested-against-real-input`.
- `docs/adr/0020-postgres-migration-format.md` (new) — MADR-full,
  `Accepted`. Locks sequential up-only migrations under
  `services/api/migrations/`; names the `vector(512)` placeholder and its
  slice-7 upgrade path. `inherited-from-existing-code` (mirrors ADR 0018's
  shape per spec `## Research findings`).
- `docs/adr/README.md` — ADR 0020 index row; next-number advanced to 0021.
  `tested-against-real-input`.
- `docs/conventions/data.md` — `## Postgres` repointed at ADR 0020,
  "Deferred" language removed. `tested-against-real-input`.
- `.github/workflows/compose.yml` (new) — `docker compose config -q` CI on
  both overlays, path-filtered to `infra/compose/**` + itself.
  `tested-against-real-input`.

**Verification (all pass):** both overlays `config -q` clean; the
`contracts` generator produces no drift after the `schema.sql` edit; ER
diagram entity set (8) matches `schema.sql`'s `CREATE TABLE` names exactly;
init SQL has zero app-table `CREATE TABLE` statements; ADR 0020 has all five
MADR sections and is `Accepted`; `docs/adr/README.md` and
`docs/conventions/data.md` both cite `0020`; Redis prod overlay sets
`--appendonly yes`; Caddyfile has no uncommented `reverse_proxy` to a
nonexistent upstream; OTel collector config declares `otlp` receivers +
`debug` exporter; `infra/models/README.md` exists; `.env.example` holds no
live credential; `git diff --name-only main...HEAD` (excluding `.tasks/`)
stays within `infra/`, `docs/`, `packages/contracts/`,
`.github/workflows/`; full `up -d` → 4/4 running within 60s → `\dx` shows
`vector` → Redis `PONG` → Caddy `curl` `200` → OTel readiness log present →
`down -v` exits 0 with all named volumes removed.
