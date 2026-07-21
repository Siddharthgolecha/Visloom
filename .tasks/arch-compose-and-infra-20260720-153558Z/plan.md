# Plan — arch-compose-and-infra

<!-- How this task gets done, tactically. Frozen at `plan-approved`. -->

## Tactical steps

Ordered. Each step cites the `spec.md` section it implements.

1. **Postgres init.** Add `infra/compose/postgres/init/001-extensions.sql`
   (`CREATE EXTENSION IF NOT EXISTS vector;` + db/role bootstrap). Implements
   `spec.md ## Approach` "Postgres init"; acceptance criterion on no
   `CREATE TABLE`.
2. **Schema extension + ADR 0020 + docs repoint.** In one commit: add the
   `embeddings` table to `packages/contracts/schema.sql` (with the
   provisional-dimension comment and updated header), write
   `docs/adr/0020-postgres-migration-format.md`, add its index row to
   `docs/adr/README.md`, and repoint `docs/conventions/data.md ## Postgres`
   at ADR 0020. Implements `spec.md ## Approach` "Schema extension" +
   "ADR 0020"; resolves the two forward-references cited in `## Context`.
3. **Run the contracts generator** (`scripts/gen-contracts.sh` /
   `packages/contracts` `make contracts`) and confirm `git status` is clean
   after step 2. Implements the drift-check verification named in
   `spec.md ## Failure modes`.
4. **Diagram sync.** Update `docs/architecture/overview.md`'s
   `## Data model (ER)` `erDiagram` fence + prose to include `embeddings`
   and drop the "seven tables ... not shown" framing. Implements
   `spec.md ## Approach` "Diagram sync"; closes the drift named in OQ1.
5. **Redis config.** Add `infra/compose/redis/redis.conf` per the resolved
   persistence OQ (dev: no persistence directives; prod overlay adds
   `appendonly yes` + a memory limit). Implements `spec.md` OQ4.
6. **Caddy config.** Add `infra/compose/caddy/Caddyfile` — web at `/`, API
   at `/api/*`, upstream `reverse_proxy` directives commented out, dev
   overlay plain HTTP / prod overlay TLS block. Implements `spec.md` OQ2.
7. **OTel collector config.** Add `infra/compose/otel/collector.yaml` —
   OTLP gRPC+HTTP receivers, `debug` exporter. Implements `spec.md` OQ1.
8. **Compose files.** Add `infra/compose/compose.yml` (four services:
   `postgres`, `redis`, `caddy`, `otel-collector`; named volumes; bind
   mounts for the four config files from steps 1/5/6/7) and
   `infra/compose/compose.prod.yml` (image pins, `pgvector/pgvector:pg16`
   per OQ3, resource limits, `restart: unless-stopped`). Implements
   `spec.md ## Approach` "Compose files."
9. **`.env.example`.** Placeholder-only env vars for Postgres/Redis
   creds + `OTEL_EXPORTER_OTLP_ENDPOINT`. Implements the privacy
   acceptance criterion.
10. **`infra/models/README.md`.** Placeholder for the model-weights
    directory. Implements the issue's stated deliverable list.
11. **Compose CI.** Add `.github/workflows/compose.yml` running
    `docker compose ... config -q` on both overlays, triggered on
    `infra/compose/**` changes *and* changes to the workflow file itself.
    Implements OQ3 (resolved) + the compose CI acceptance criteria.
12. **Full verification pass.** `docker compose -f compose.yml [-f compose.prod.yml] config -q`;
    `docker compose ... up -d`; check pgvector extension, Redis ping,
    Caddy container health, OTel collector readiness log; `down -v` cleanup.
    Implements `spec.md ## Acceptance criteria` end-to-end.

## Pre-approval evidence

<!-- Checks that must happen before requesting plan-approved, not implementation steps. -->

- **Overlap check** (`AGENTS.md` §6, mitigates the ADR-0020-collision
  Failure mode): `gh pr list --search 'is:draft'` run 2026-07-21 — only
  draft PR is this task's own `#16`. No parallel ADR-numbering or
  `packages/contracts/` PR. Re-run immediately before `plan-approved` is
  requested if significant time has passed.

## Files touched

- `infra/compose/compose.yml` — new; base service defs (pg/redis/caddy/otel).
- `infra/compose/compose.prod.yml` — new; prod overlay.
- `infra/compose/.env.example` — new; placeholder env vars.
- `infra/compose/postgres/init/001-extensions.sql` — new; pgvector extension + bootstrap.
- `infra/compose/redis/redis.conf` — new; persistence + sizing per ADR 0016 TTL classes.
- `infra/compose/caddy/Caddyfile` — new; same-origin routing, upstreams commented.
- `infra/compose/otel/collector.yaml` — new; OTLP receivers + debug exporter.
- `infra/models/README.md` — new; model-weights placeholder.
- `packages/contracts/schema.sql` — add `embeddings` table; update header comment.
- `docs/architecture/overview.md` — sync ER diagram + prose to new table set.
- `docs/adr/0020-postgres-migration-format.md` — new ADR.
- `docs/adr/README.md` — index row for 0020.
- `docs/conventions/data.md` — repoint `## Postgres` at ADR 0020.
- `.github/workflows/compose.yml` — new; compose config CI job.

## Depends on

Slices 1–4 (`arch-conventions-and-adrs-part-1`, `-part-2`,
`arch-contracts-package`, `arch-diagrams-and-overview`) — all merged
(PRs #11, #12, #14, #15). No open dependency.
