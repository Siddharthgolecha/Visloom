# Spec — arch-compose-and-infra

<!-- The contract for this task: what it should do, why, and how we'll know.
     Frozen at `plan-approved`. After that, deviations go in implementation.md. -->

## Context

Slice 5 of the arch-scaffold epic ([#1][epic]), tracked as [#6][issue] with
lane `lane:considered`. Slices 1–4 landed the source material this slice
builds on:

- Part-1/2 ([PR #11][pr11], [PR #12][pr12]) — ADRs 0001–0018 + conventions.
- Contracts ([PR #14][pr14]) — `packages/contracts/schema.sql` (7 tables,
  reference-only, no migrations).
- Diagrams ([PR #15][pr15]) — `docs/architecture/overview.md`, 7 Mermaid
  diagrams including an ER diagram of those 7 tables, plus ADR 0019.

This slice ships `infra/compose/{compose.yml,compose.prod.yml,.env.example}`,
a Postgres init enabling pgvector, Redis conf, a same-origin Caddyfile, an
OTel collector config, and `infra/models/README.md` — **only** pg + redis +
caddy + otel services; API/worker/web arrive in slices 6/7/8 via additive
edits ([ADR 0004:29-31][adr4]).

It also discharges two decisions the prior slices explicitly deferred here:

1. **Postgres migration format.** [ADR 0016:19-21][adr16] and
   [data.md:36-44][data] both assign "a follow-up ADR alongside slice 5,
   once there is a concrete `init.sql` to react to" — this slice now has
   that `init.sql`.
2. **Embeddings/pgvector/frames tables.** The diagrams spec states twice
   that these "arrive with the slice-5 migration"
   ([overview.md:223-225][overview-er]; mirrored in the now-merged slice-4
   spec) — so `packages/contracts/schema.sql` gains those tables here, and
   `overview.md`'s ER diagram + "seven tables present today" prose must be
   hand-synced to match (no CI guard on that drift — a documented risk, see
   Failure modes).

Intended outcome: `docker compose config` validates on both overlays, the
four-service stack brings up cleanly end-to-end, and slice 6's Rust
migration harness has a locked format + a schema (with an explicitly
provisional vector dimension) to implement against.

[epic]: https://github.com/Siddharthgolecha/Visloom/issues/1
[issue]: https://github.com/Siddharthgolecha/Visloom/issues/6
[pr11]: https://github.com/Siddharthgolecha/Visloom/pull/11
[pr12]: https://github.com/Siddharthgolecha/Visloom/pull/12
[pr14]: https://github.com/Siddharthgolecha/Visloom/pull/14
[pr15]: https://github.com/Siddharthgolecha/Visloom/pull/15
[adr4]: ../../docs/adr/0004-docker-compose-single-vps.md
[adr16]: ../../docs/adr/0016-redis-usage.md
[data]: ../../docs/conventions/data.md
[overview-er]: ../../docs/architecture/overview.md

## Open Questions for the Human

Written before re-reading the issue body, per `AGENTS.md:34-36`. Three
scope-shaping decisions were already made with the human at kickoff
(recorded as resolved below); five remain open for plan review.

**Resolved at kickoff:**

1. **Schema scope — extend `schema.sql` now, or defer all embedding
   tables to slice 6/7?** *Resolved:* **extend now.** Honors the slice-4
   spec's explicit statement that these tables "arrive with slice 5."
   Uses a documented **placeholder `vector(N)` dimension** since the real
   embedder model is a slice-7 decision ([ADR 0010][adr10]); ADR 0020
   records the placeholder and that changing it later is itself a
   migration. Ripples: (a) re-touches `packages/contracts/` — spec-lane
   gated regardless (`AGENTS.md` §6), and the [contracts drift-check
   workflow][contracts-ci] runs on this PR (expected clean: `schema.sql`
   is a manual reference, not a codegen input, per its own header
   comment); (b) the `overview.md` ER diagram + "seven tables" prose
   silently disagree with the new schema unless hand-synced — no CI guard
   exists for that (this repo's known gap; tracked as a Failure mode).
2. **Where does the new ADR live — bundled here, or a sibling PR?**
   *Resolved:* **bundled in this PR.** The decision reacts to this PR's
   own `init.sql`; splitting it into a separate PR would make ADR 0020
   cite a file that doesn't exist yet in `main`.
3. **Does this slice add CI for the compose stack, or defer to slice 9?**
   *Resolved:* **add it here** — a workflow that runs
   `docker compose config -q` on both overlays. Cheap, catches the
   most common breakage (a typo'd compose key) without requiring the
   full `make` target set that's slice 9's job. The trigger's path filter
   covers both `infra/compose/**` and the workflow file itself
   (`.github/workflows/compose.yml`) so an edit to the CI job self-validates
   instead of silently going unchecked.

**Open for plan review:**

1. **OTel collector depth.** Full receiver/exporter matrix, or the minimum
   that proves the pipe works? *Proposed:* OTLP gRPC + HTTP receivers, a
   `debug`/logging exporter only — backend choice (Grafana Cloud /
   Honeycomb / self-hosted) is explicitly deferred to ops-time
   ([ADR 0015:95-97][adr15]).
2. **Caddy same-origin routing + TLS.** *Proposed:* web at `/`, API
   reverse-proxied at `/api/*` — matches the container diagram in
   `overview.md`. Dev overlay serves plain HTTP; prod overlay adds
   ACME/TLS on a real domain. Because no API/web container exists yet,
   the actual `reverse_proxy` upstream directives are **commented out**
   this slice — Caddy only proves its config parses and the container
   runs (a static/health stub responds), not live routing. Live routing
   is a slices-6/8 acceptance concern, additively wired.
3. **Postgres image + major-version pin.** Forward-binds slice 6's
   migrations. *Proposed:* `pgvector/pgvector:pg16` (official pgvector
   image, Postgres 16).
4. **Redis persistence policy.** *Proposed:* dev overlay ephemeral
   (no persistence — fast resets, matches [ADR 0004:36][adr4]'s
   "`docker compose down -v && make up` = full reset"); prod overlay
   enables `appendonly yes` + memory limits.
5. **Placeholder vector dimension value.** *Proposed:* `512` (a common
   CLIP-family embedding size, consistent with ADR 0009's "small
   CLIP-family model" framing) — explicitly provisional, ADR 0020 names
   the upgrade path.

[adr10]: ../../docs/adr/0010-inference-runtime-worker-cuda-api-cpu.md
[adr15]: ../../docs/adr/0015-observability-otel-first.md
[contracts-ci]: ../../.github/workflows/contracts.yml

## Research findings

Each claim traces to a `file:line` or whole-document ADR citation, per
`AGENTS.md:36`.

**Compose topology:**

- `docs/adr/0004-docker-compose-single-vps.md` — two-overlay pattern
  (`compose.yml` + `compose.prod.yml`), same service defs local/prod,
  overlay flips tags/limits/volumes. Explicitly assigns this slice
  pg+redis+caddy+otel only, api/worker/web additive later
  (`docs/adr/0004-docker-compose-single-vps.md:29-31`).
- `docs/architecture/overview.md:46-84` — the Container diagram this
  slice's topology must match: Caddy fronts web+API same-origin; API and
  worker never call each other directly, only via Redis Streams; both
  write Postgres and export to the OTel collector.

**Postgres / pgvector:**

- `packages/contracts/schema.sql:1-19` — header states slice 5 owns the
  migration format and "will implement this reference under
  `services/api/migrations/`"; ULIDs stay `text` at the reference level,
  slice 5 may pick a binary encoding (not exercised this slice — the
  encoding stays `text` to avoid a second, unreviewed decision beyond
  the two already scoped in).
- `docs/adr/0016-redis-usage.md:18-21` and `docs/conventions/data.md:36-44`
  — both explicitly defer "migration naming, transaction boundaries,
  backwards-compat window" to this slice's ADR.
- `docs/architecture/overview.md:218-225` — "Embeddings, pgvector columns,
  and per-frame rows are not shown [in the ER diagram]: they arrive with
  the slice-5 migration." Direct instruction that this slice adds them.
- `docs/adr/0009-search-transport-cpu-onnx-inline.md` — pgvector column is
  populated by the API search path; `embedding_ref` is "an opaque pointer
  into the pgvector column ... encoding decided by slice 6/9"
  (`packages/contracts/events/events.media.indexed.v1.json`, `data.embedding_ref`
  description).
- `docs/adr/0010-inference-runtime-worker-cuda-api-cpu.md` — the embedder
  model (and thus the real vector dimension) is chosen at slice-7 code
  time; this slice cannot know the true `N`.
- `docs/adr/0007-media-scope-photo-and-video-keyframe.md:40-55` — media is
  `{photo, video}`; video yields 1..N `MediaFrame` embeddings per source —
  source for the frames table's one-to-many shape.

**Redis:**

- `docs/adr/0016-redis-usage.md:39-58` — `vloom:<domain>:<entity>:<id>`
  naming + three-class TTL taxonomy (short/medium/long); streams excluded
  from the `vloom:` prefix.
- `docs/conventions/data.md:7-32` — the convention doc mirroring the above,
  already merged; `redis.conf` sizing in this slice must not contradict it
  (no naming enforcement lives in `redis.conf` itself — that's app-side).

**Observability:**

- `docs/adr/0015-observability-otel-first.md:93-97` — "Compose stack
  (slice 5) runs an OTel collector service that every runtime points at
  via `OTEL_EXPORTER_OTLP_ENDPOINT`." Backend choice is deferred.

**Contracts / CI surface:**

- `.github/workflows/contracts.yml` — the drift-check that regenerates
  `packages/contracts/{ts,rust,py}/` from `events/`+`openapi/` and fails on
  diff. `schema.sql` is not a generator input (`scripts/gen-contracts.sh`
  reads only `events/` + `openapi/`), so a `schema.sql`-only edit should not
  trigger drift — verified at implementation time by running the generator
  and confirming a clean `git status`.
- `.github/workflows/autodev-guard.yml:125` — hard-fails a Quick-lane PR
  touching `packages/contracts/`; irrelevant here since this is Spec lane,
  but confirms the overlap-list classification is correct.

**Authoring precedent:**

- `docs/adr/0018-documentation-tooling.md` — precedent for a slice-scoped
  ADR whose `## Consequences` names the downstream slice consuming it;
  ADR 0020 mirrors this shape.
- `docs/adr/template.md`, `docs/adr/README.md:33` — next number is `0020`;
  five MADR-full sections required.
- `.tasks/arch-diagrams-and-overview-20260717-065051Z/spec.md` — the
  spec/plan shape and Open-Questions-resolved-before-code pattern this
  slice mirrors.

## Approach

**Compose files.** `infra/compose/compose.yml` defines four services —
`postgres` (pgvector image), `redis`, `caddy`, `otel-collector` — with
named volumes for Postgres/Redis data and bind-mounts for the three config
files (`postgres/init/*.sql`, `redis/redis.conf`, `caddy/Caddyfile`,
`otel/collector.yaml`). `compose.prod.yml` overlays image pins, resource
limits, `restart: unless-stopped`, and Redis `appendonly`. `.env.example`
holds placeholder credentials/endpoints only (privacy — no real secrets,
ever, per `AGENTS.md` privacy-by-default rule).

**Postgres init.** `infra/compose/postgres/init/001-extensions.sql` runs
`CREATE EXTENSION IF NOT EXISTS vector;` plus any db/role bootstrap Compose
needs — it does **not** execute `schema.sql`'s tables (that's slice 6's
migration harness, per `schema.sql:1-4`; init scripts and migrations stay
separate concerns).

**Schema extension.** `packages/contracts/schema.sql` gains one table:
`embeddings` — one row per indexed unit of media (a photo contributes one
row, a video contributes one row per extracted keyframe), with a
`vector(512)` column (dimension explicitly provisional, see OQ5), an FK to
`media`, and a `frame_index` column expressing the one-to-many relationship
without a separate join table (keeps the schema at 8 tables, not 9 —
confirmed in plan review). The "Tables covered" header comment and the
migration-ownership note update to reflect slice 5 landing.

**Diagram sync.** `docs/architecture/overview.md`'s `## Data model (ER)`
section adds the `embeddings` entity to the `erDiagram` fence and rewrites
the "seven tables present today... not shown" prose to state the current
(now 8-table) set and that the dimension is provisional pending slice 7.

**ADR 0020.** MADR-full, `Status: Accepted`, per `docs/adr/template.md`.
Records: migration tool/format (see alternatives below), up/down policy,
backwards-compat window, and the provisional-dimension caveat on the new
`embeddings` table. Index row added to `docs/adr/README.md`.

**Redis / Caddy / OTel configs.** Each is a minimal, working config per the
resolved Open Questions above — sized/scoped for what slice 5 can prove
(config parses, container runs, extension/ping/log-line checks) without
inventing behavior slices 6/8 own.

**CI.** `.github/workflows/compose.yml` runs
`docker compose -f infra/compose/compose.yml config -q` and the same with
`-f infra/compose/compose.prod.yml` appended, on push/PR touching
`infra/compose/**` or the workflow file itself.

### Alternative considered

**Defer the embedding tables to slice 6, keep `schema.sql` untouched this
slice.** Simpler — no contracts-package re-touch, no ER-diagram sync, no
provisional-dimension caveat. Rejected: it directly contradicts the
already-merged slice-4 spec's stated plan ("arrive with the slice-5
migration"), and letting that forward reference go unfulfilled means a
future slice-6 spec has to either fix a documentation claim retroactively
or discover the mismatch mid-implementation. Better to resolve the
tension now, in the open, with an explicit provisional-value ADR.

**Land ADR 0020 as its own sibling docs-only PR** (separate child issue),
keeping this PR pure infra. Matches the epic's one-concern-per-PR ethos
more strictly. Rejected (per kickoff decision): the ADR's content — the
migration format — is meaningless without the concrete `init.sql`/schema
change it reacts to; splitting them means the sibling PR either forward-
references files not yet in `main` or the two PRs must merge in lockstep
anyway, which is more process overhead than the single-PR bundle.

**Defer compose CI to slice 9.** Keeps this PR's diff to infra files +
docs only. Rejected (per kickoff decision): a compose file that breaks
silently at merge time and isn't caught until slice 6/7/8 tries to `up`
against it is a worse failure mode than one small workflow file now.

## Tradeoffs accepted

- **Placeholder vector dimension may churn.** `vector(512)` is a guess;
  slice 7's real embedder model may need a different size, forcing a
  migration. Accepted — ADR 0020 names this as the upgrade path, and a
  dimension change before any production data exists is cheap.
- **Caddy and the OTel collector are config-only stubs this slice.** No
  live reverse-proxy routing or real trace ingestion can be smoke-tested
  until slices 6/8 exist. Accepted — verification here is "config parses,
  container runs, minimal health signal," not full integration.
- **`schema.sql` ↔ `overview.md` ER-diagram sync has no CI guard.** Same
  known gap as the diagrams slice itself; a future edit to one without the
  other silently drifts. Accepted with a Failure-modes entry; a standing
  guard is its own slice (not proposed here — would be scope creep).
- **The frames concept is folded into `embeddings.frame_index` rather than
  a separate table.** Slightly denormalized (a photo's single frame stores
  `frame_index = 0` rather than omitting it), but avoids a needless join
  table for what is structurally one embedding row per unit of media.
  Accepted; revisit only if slice 7 needs frame-level metadata beyond an
  index.

## Failure modes

Adversarial re-read.

- **`schema.sql` ↔ `overview.md` ER drift.** Nothing catches a future edit
  to one without the other (no CI guard exists for diagrams, unlike
  contracts). Mitigation: this slice's acceptance criteria grep both files
  for the same table set at review time; the risk persists afterward and
  is named here for the next slice to inherit knowingly.
- **Contracts drift-check false-fires on a `schema.sql`-only change.**
  If `scripts/gen-contracts.sh` turns out to read `schema.sql` in some
  way not visible in the current script, the drift-check could fail
  unexpectedly. Mitigation: run `make contracts` (or the script directly)
  locally before requesting `plan-approved` and confirm clean diff —
  stated as a verification step, not assumed.
- **Postgres image drift.** `pgvector/pgvector:pgN` tags move; an unpinned
  `latest` silently changes Postgres major version underneath slice 6's
  migrations. Mitigation: pin an explicit tag (OQ3), not `latest`.
- **OTel collector silently drops telemetry.** A misconfigured receiver/
  exporter pair starts the container but drops every span — looks healthy,
  isn't. Mitigation: acceptance criterion checks the collector's own
  startup log for "Everything is ready," the standard OTel-collector
  readiness line, not just "container running."
- **Caddyfile fails to parse under the prod overlay's TLS block** even
  though the dev overlay parses fine. Mitigation: CI job (this slice) runs
  `config -q` against *both* overlays, not just the base file.
- **ADR 0020 number collision.** A parallel branch could also claim `0020`.
  Mitigation: `gh pr list --search 'is:draft'` before requesting
  `plan-approved`, per `AGENTS.md` §6.
- **Provisional dimension gets forgotten and treated as final.** If ADR
  0020 doesn't say clearly enough that `512` is a placeholder, slice 7
  might build around it as fixed. Mitigation: the ADR's `## Consequences`
  explicitly names "changing the dimension is itself a migration" and
  cites the deciding slice (7).
- **Init script accidentally becomes a de facto migration.** If
  `postgres/init/*.sql` grows beyond "enable extension + bootstrap db/role"
  into something that creates app tables, it collides with slice 6's
  migration harness and creates two competing sources of schema truth.
  Mitigation: acceptance criterion checks init SQL contains no
  `CREATE TABLE` for app-domain tables.

## Acceptance criteria

Every criterion automated-or-observable with a falsifier.

- [ ] `infra/compose/compose.yml` defines exactly four services —
  `postgres`, `redis`, `caddy`, `otel-collector` — and no `api`/`worker`/
  `web` service keys. *Falsified if:* a service key outside that set of
  four is present, or one of the four is missing.
- [ ] `docker compose -f infra/compose/compose.yml config -q` exits 0.
  *Falsified if:* nonzero exit or parse error.
- [ ] `docker compose -f infra/compose/compose.yml -f infra/compose/compose.prod.yml config -q`
  exits 0. *Falsified if:* nonzero exit or parse error.
- [ ] A new CI workflow runs both `config -q` invocations above on PRs
  touching `infra/compose/**` or the workflow file itself. *Falsified if:*
  no workflow file references `docker compose ... config`, or its path
  filter omits `.github/workflows/compose.yml`.
- [ ] Postgres init contains `CREATE EXTENSION IF NOT EXISTS vector` and no
  `CREATE TABLE` statements. *Falsified if:* `grep -c 'CREATE TABLE'
  infra/compose/postgres/init/*.sql` ≠ 0, or the extension line is absent.
- [ ] `packages/contracts/schema.sql` gains exactly one new table
  (`embeddings`) with a `vector(512)` column and an explicit
  provisional-dimension comment. *Falsified if:* the file's `CREATE TABLE`
  count changes by anything other than +1, the column is not exactly
  `vector(512)`, or no comment marks the dimension provisional.
- [ ] Running the contracts generator (`scripts/gen-contracts.sh` or
  `packages/contracts`'s `make contracts`) after the `schema.sql` edit
  produces a clean `git status` (no drift). *Falsified if:* the generator
  run produces any diff under `packages/contracts/{ts,rust,py}/`.
- [ ] `docs/architecture/overview.md`'s ER diagram and prose reflect the
  updated table set. *Falsified if:* the set of entities in the `erDiagram`
  fence differs from the actual `CREATE TABLE` names in `schema.sql`.
- [ ] ADR 0020 exists, follows the five MADR sections, is `Status:
  Accepted`, and names slice 6 and/or 7 in `## Consequences` as the
  downstream consumer. *Falsified if:* any template section is missing,
  status isn't `Accepted`, or no downstream slice is named.
- [ ] `docs/adr/README.md` lists ADR 0020 with its title verbatim.
  *Falsified if:* `grep -c '0020' docs/adr/README.md` = 0 or the row title
  differs from the ADR's H1.
- [ ] `docs/conventions/data.md`'s `## Postgres` section no longer says
  "Deferred" without qualification — it points at ADR 0020.
  *Falsified if:* `grep -c '0020' docs/conventions/data.md` = 0.
- [ ] Redis config sets an explicit persistence policy matching the
  overlay (dev: none/default; prod: `appendonly yes`). *Falsified if:*
  `compose.prod.yml`'s Redis service has no `appendonly` setting.
- [ ] Caddyfile parses under both overlays (covered by the CI criterion
  above) and contains no active `reverse_proxy` to a nonexistent upstream
  (commented placeholders are fine). *Falsified if:* an uncommented
  `reverse_proxy` directive targets a service not defined in
  `compose.yml`.
- [ ] OTel collector config declares OTLP gRPC + HTTP receivers and at
  least one exporter. *Falsified if:* `collector.yaml` has no `receivers:`
  block containing `otlp`, or no `exporters:` block.
- [ ] `infra/models/README.md` exists. *Falsified if:* the file is absent.
- [ ] `.env.example` contains no real secret values (all placeholders).
  *Falsified if:* any value looks like a live credential (manual review —
  privacy-by-default).
- [ ] `git diff --name-only main...HEAD` (excluding `.tasks/`) stays within
  `infra/`, `docs/`, `packages/contracts/`, `.github/workflows/`.
  *Falsified if:* any path falls outside that set.
- [ ] `docker compose -f infra/compose/compose.yml up -d` brings all four
  services to a running state within 60s. *Falsified if:* any service exits
  or fails to reach `running`/`healthy` within that window.
- [ ] `docker compose exec postgres psql -U <u> -d <db> -c '\dx'` lists
  `vector` in its output. *Falsified if:* the extension is absent from
  `\dx`.
- [ ] `docker compose exec redis redis-cli ping` returns `PONG`.
  *Falsified if:* any other response, timeout, or connection refusal.
- [ ] The Caddy container's config-stub endpoint returns a non-5xx response.
  *Falsified if:* the request errors, times out, or returns 5xx.
- [ ] The OTel collector container's logs contain the standard readiness
  line ("Everything is ready" or equivalent startup-complete message).
  *Falsified if:* absent from `docker compose logs otel-collector` within
  30s of `up`.
- [ ] `docker compose -f infra/compose/compose.yml down -v` exits 0 and
  removes the stack's named volumes, confirming the "full reset" claim in
  [ADR 0004:36][adr4]. *Falsified if:* nonzero exit, or a volume from this
  stack survives `docker volume ls` afterward.
