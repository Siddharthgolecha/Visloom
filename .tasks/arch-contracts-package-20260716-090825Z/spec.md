# Spec — arch-contracts-package

## Context

Slice 3 of the arch-scaffold epic ([#1][epic]), tracked as
[#4][issue] with lane `lane:considered`. First slice that ships
non-docs artifacts.

Part-1 ([PR #11][pr11], merged `fb4b3f3`) and part-2 ([PR #12][pr12],
merged `ce46558`) landed the ADR + conventions surface. Three of
those docs make concrete forward promises this slice must satisfy:

- `docs/conventions/events.md:63` — "Schemas + generated types
  land in slice 3." Canonical stream names at
  `docs/conventions/events.md:26-30`.
- `docs/conventions/api.md:4-6` — "OpenAPI 3.1 skeleton lands in
  slice 3 (`packages/contracts/openapi/`)."
- ADR `0011:44-53` — commits generated bindings under
  `packages/contracts/{ts,rust,py}/`, `scripts/gen-contracts.sh`
  as the codegen driver, CI drift-check
  `make contracts && git diff --exit-code`, generator versions
  **pinned in the script**, and initial `AGENTS.md` §6 overlap
  entries. ADR `0006:59-62` names the same slice-3 obligations
  from the event-transport side.

None of `packages/`, `scripts/`, `services/`, `apps/`, or `infra/`
exist yet — this slice creates the first two. `.github/workflows/
ci.yml:33-42` is a placeholder setup job with no real toolchain;
slice 3 introduces the first CI job that fetches non-trivial
dependencies. `AGENTS.md:239-247` (§6) is currently empty and
waiting for the first stabilized public interface — slice 3
lands it.

Intended outcome: `packages/contracts/` is the canonical wire
description, `make contracts` reproduces `{ts,rust,py}/`
deterministically, CI fails any PR whose regenerated diff isn't
committed, and downstream slices 6/7/8 have a single directory
to import from.

[epic]: https://github.com/Siddharthgolecha/Visloom/issues/1
[issue]: https://github.com/Siddharthgolecha/Visloom/issues/4
[pr11]: https://github.com/Siddharthgolecha/Visloom/pull/11
[pr12]: https://github.com/Siddharthgolecha/Visloom/pull/12

## Open Questions for the Human

Four OQs. Each carries a proposed answer so `plan.md` has no
conditional branches; reviewer may override at plan-review time.
Kept in-spec as append-only audit trail per `AGENTS.md:34-36`.

1. **`schema.sql` posture — canonical reference vs deferred.** The
   epic checklist (`.tasks/epics/arch-scaffold/parent.md:44`) says
   slice 3 ships a "canonical schema.sql reference," but
   `docs/conventions/data.md:34-45` and ADR `0016` explicitly defer
   Postgres schema evolution to slice 5.
   *Proposed:* land `packages/contracts/schema.sql` as a
   **forward-looking reference** — a versioned target describing
   the Postgres relations the contracts imply (accounts, sessions,
   events, memberships, media, idempotency_keys), **not** an
   executed migration and **not** the source-of-truth migration
   format. Header comment cites ADR 0016's deferral and names
   slice 5 as the executor. Preserves the epic contract while
   respecting the ADR. Alternative: drop `schema.sql` entirely,
   defer to slice 5.

2. **Generator toolchain — no ADR names them.** ADR `0011:50-51`
   requires pinned generators but doesn't name them.
   *Proposed pins:*
   - Python: `datamodel-code-generator==0.26.3` (JSON Schema →
     Pydantic v2). Native 2020-12 support.
   - Rust: `typify-cli@0.2.0` (JSON Schema → serde structs) via
     `cargo install --locked`.
   - TypeScript: `json-schema-to-typescript@15.0.4` invoked via
     `npx --yes --package=…`.
   - OpenAPI lint/bundle: `@redocly/cli@1.25.11`. Client-stub
     generation from OpenAPI deferred until slice 6/8.
   All four pins live literally in `scripts/gen-contracts.sh`
   (per `0011:50-51`), not as loose "^" or ">=" ranges.
   Alternative: one meta-generator (`quicktype`) for all three
   languages — see §Alternative considered.

3. **AGENTS.md §6 enforcement scope — soft-list vs hard-fail.**
   `.github/workflows/autodev-guard.yml:114-138` today has a
   commented-out overlap-hits block; `AGENTS.md:242-247` calls the
   list "soft (not CI-enforced)."
   *Proposed:* land the paths + streams in `AGENTS.md` §6 as
   documented entries **and** replace the commented example in
   `autodev-guard.yml` with a hard-fail check that fails any
   Quick-lane PR touching `packages/contracts/**`,
   `scripts/gen-contracts.sh`, or `.github/workflows/contracts.yml`.
   Matches ADR 0011's "adds initial entries" wording — enforcement
   is the point of a stabilized list. Alternative: land AGENTS.md
   text only; leave the workflow commented.

4. **Envelope subschema strategy — shared `$ref` vs inlined copy.**
   All three event schemas share the same top-level envelope
   (`event_id`, `traceparent`, `tracestate?`, `occurred_at`,
   `data`) per `docs/conventions/events.md:45-63` and ADR
   `0015:58-79`.
   *Proposed:* one shared subschema at
   `packages/contracts/events/_envelope.v1.json`; the three stream
   schemas `$ref` its properties and override only `data`.
   Alternative: inline the envelope into each stream schema (three
   copies). Chosen over inline because an ADR-0015 payload
   evolution (e.g. adding `baggage`) is a one-file edit — not a
   three-file edit prone to drift.

## Research findings

Every claim `file:line`-anchored per `AGENTS.md:32-38`.

- `.tasks/epics/arch-scaffold/parent.md:44` — slice 3 checklist
  item: `packages/contracts/` + schemas + OpenAPI + generator +
  CI drift-check + overlap-list.
- `docs/conventions/events.md:26-30` — the three canonical stream
  names: `jobs.media.index.v1`, `events.media.indexed.v1`,
  `events.media.index_failed.v1`.
- `docs/conventions/events.md:45-63` — payload envelope shape
  (`event_id` ULID, `traceparent`, optional `tracestate`,
  derived-log-only `trace_id`, `occurred_at` ISO-8601 UTC, `data`).
  Version lives on the stream name, never in the payload
  (`events.md:18-19`).
- `docs/conventions/api.md:4-6` — OpenAPI 3.1 lives at
  `packages/contracts/openapi/`.
- `docs/conventions/api.md:8-15` — URL shape `/api/v1/` +
  same-origin `/api/*` proxy through Caddy.
- `docs/conventions/api.md:18-22` — `GET /healthz` unauthenticated,
  200 body `{"status":"ok","embedder":{"model_id":…,"version":…}}`.
  `/readyz` deferred to a later slice.
- `docs/conventions/api.md:25-33` — wire-error envelope
  (`error.code`, `error.message`, `error.trace_id`) + HTTP status
  vocabulary (400/401/403/404/409/422/429/500/503).
- `docs/conventions/api.md:35-39` — `Idempotency-Key` header
  (client ULID) on mutating endpoints; storage lands in slice 6.
- `docs/conventions/data.md:34-45` — Postgres schema evolution
  deferred to slice 5; any slice-3 `schema.sql` is unversioned by
  convention.
- `docs/conventions/testing.md:16-20` — contract tier validates
  the wire between services, generated from `packages/contracts/`.
- `docs/conventions/testing.md:32-34` — contract tests live under
  `packages/contracts/tests/`.
- `docs/conventions/testing.md:36-44` — fixtures are plain data,
  no fixture frameworks.
- `docs/conventions/errors.md:9-22` — two-layer error model
  (domain per-language + wire envelope).
- `docs/conventions/errors.md:29-38` — error mapping is an adapter
  concern; no PII in wire messages; `trace_id` present.
- ADR `0006:47-51,59-62` — canonical stream names; slice-3 homing
  under `packages/contracts/events/*.v1.json`; `.v<int>` rule
  added to AGENTS §6.
- ADR `0007:38-46` — `MediaKind ∈ {photo, video}` discriminator
  landing in slice-3 contract schemas.
- ADR `0009:63-65` — search transport is intra-API; no search
  contract schemas in slice 3.
- ADR `0011:31-53` — commit-generated + drift-check; three target
  langs; script drives codegen; pinned generators; CI runs
  `make contracts && git diff --exit-code`.
- ADR `0015:58-79` — `traceparent` + optional `tracestate` on
  every stream payload; `trace_id` log-only, derived.
- ADR `0016:19-21,91-95` — Postgres schema evolution out of scope,
  slice-5 follow-up.
- ADR `0017:36-58,71-77` — three-axis versioning; contracts
  package on SemVer; slice-3 owns concrete SemVer bump rules +
  release-note format + deprecation window.
- ADR `0003:39-42` — `packages/contracts/` is the shared-contract
  home; slice 3 owner.
- `.github/workflows/ci.yml:11-42` — path-filtered skeleton with
  `dorny/paths-filter@v3`; `changes` filter treats `.tasks/**`,
  `.github/**`, `*.md`, `LICENSE` as no-code. No runtime setup
  yet.
- `.github/workflows/autodev-guard.yml:114-138` — commented-out
  overlap-hits block; currently a no-op with the placeholder
  `overlap_hits=""`.
- `AGENTS.md:239-247` — §6 overlap list is empty today, waiting
  for stabilized interfaces.
- Repo state: `packages/`, `scripts/`, `services/`, `apps/`,
  `infra/` **do not exist yet**.
- `.tasks/arch-conventions-and-adrs-part-2-20260715-181956Z/spec.md`,
  `plan.md` — style precedent for this slice's spec+plan shape.

## Approach

One PR of ~20 new files under `packages/contracts/` and
`scripts/`, plus targeted edits to `AGENTS.md`,
`.github/workflows/autodev-guard.yml`, and a new
`.github/workflows/contracts.yml`. Every new file either derives
mechanically from a schema (generated code) or is a schema /
spec / test / script authored once. All generator versions
pinned inside `scripts/gen-contracts.sh` per ADR 0011.

### Directory layout under `packages/contracts/`

```
packages/contracts/
├── VERSION                       # SemVer, starts at 0.1.0
├── README.md                     # what the package is + bump rules
├── CHANGELOG.md                  # release notes, seeds 0.1.0
├── Makefile                      # contracts / lint / test targets
├── events/
│   ├── _envelope.v1.json         # shared envelope subschema (OQ 4)
│   ├── jobs.media.index.v1.json
│   ├── events.media.indexed.v1.json
│   └── events.media.index_failed.v1.json
├── openapi/
│   └── openapi.v1.yaml           # single-file OpenAPI 3.1 skeleton
├── schema.sql                    # forward-looking Postgres reference (OQ 1)
├── ts/                           # generated — DO NOT EDIT
├── rust/                         # generated — DO NOT EDIT
├── py/                           # generated — DO NOT EDIT
└── tests/
    ├── examples/                 # fixture JSON per stream schema
    │   ├── jobs.media.index.v1.json
    │   ├── events.media.indexed.v1.json
    │   └── events.media.index_failed.v1.json
    ├── test_schemas.py           # self-validation + example round-trip
    └── conftest.py               # pytest discovery
```

Each generated tree carries a top-level `DO NOT EDIT — regenerate
via make contracts` banner (per `0011:44-49`).

### Event schemas — envelope $ref (OQ 4)

`_envelope.v1.json` declares the common properties. Per
`events.md:45-63` and ADR `0015:58-75`, it requires
`event_id` (ULID pattern), `traceparent`, `occurred_at` (RFC 3339
date-time), and `data`, plus optional `tracestate`; sets
`additionalProperties: false`; declares `$schema` as JSON Schema
2020-12.

Each stream schema `$ref`s the envelope and overrides `data`.
`data` shapes (informed by ADR `0007:38-46`):

- `jobs.media.index.v1.data` — `media_id` (ULID), `media_kind`
  (`"photo"|"video"`), `source_uri` (string, `format: uri`),
  `owner_account_id` (ULID), `event_id_ref` (ULID for the parent
  event).
- `events.media.indexed.v1.data` — `media_id`, `embedding_ref`
  (opaque pointer string), `frames` (integer ≥1, always 1 for
  `photo`), `embedder_model_id`, `embedder_version`.
- `events.media.index_failed.v1.data` — `media_id`,
  `failure.code` (enum), `failure.message` (string, non-PII per
  `errors.md:32-37`), `retry.attempt` (integer),
  `retry.next_at` (nullable RFC 3339).

All schemas set `additionalProperties: false` at every object
level for review-time drift-catching.

### OpenAPI 3.1 skeleton (single file)

`packages/contracts/openapi/openapi.v1.yaml`:

- `openapi: 3.1.0`; `info.version: 0.1.0` matching `VERSION`.
- **Two `servers` entries** so `/healthz` sits above `/api/v1/`
  per ADR `0017:44-46`:
  - `/` — root, used by the `/healthz` operation via a per-
    operation `servers:` override.
  - `/api/v1` — default for future operations.
- Paths — **only `/healthz`** in this slice. 200 response shape
  literally from `api.md:19-20` (`status`,
  `embedder.model_id`, `embedder.version`). `/readyz` is not
  documented (deferred per `api.md:21-22`). No `/api/v1/*`
  operations — that's slice 6.
- `components.schemas`:
  - `ErrorEnvelope` — exact wire shape from `api.md:26-33`
    (`error.code`, `error.message`, `error.trace_id`).
  - `HealthResponse` — `status`, `embedder.model_id`,
    `embedder.version`.
- `components.parameters`:
  - `IdempotencyKey` — header parameter, ULID pattern, per
    `api.md:35-39`. Declared so slice 6 can `$ref` it.
- `components.responses`:
  - `NotFound` (404), `Conflict` (409), `Unprocessable` (422),
    `RateLimited` (429), `Internal` (500), `Unavailable` (503)
    — each wraps `ErrorEnvelope`. Status vocabulary literally
    from `api.md:30-33`.

Single-file YAML preferred over split refs because there's no
consumer yet; splitting introduces bundling complexity without
benefit. Slice 6 may split when operations arrive.

### `schema.sql` posture (OQ 1)

`packages/contracts/schema.sql` — **forward-looking reference,
not executed migration.** Header:

```sql
-- Canonical Postgres reference for slice 3 contracts.
-- NOT an executed migration. Slice 5 (arch-compose-and-infra)
-- owns the migration format per ADR 0016 (Postgres deferred)
-- and will implement this reference under
-- services/api/migrations/.
```

Contents: `CREATE TABLE` statements for tables the contracts
imply — `accounts`, `sessions`, `event_owners` /
`event_memberships` (per ADR 0008), `media` (with `media_kind`
matching the JSON Schema enum per ADR 0007),
`idempotency_keys` (per `api.md:35-39`). No indexes, no
migrations up/down, no schema-versioning header. Purpose is
review-time cross-check: if the SQL and the JSON schemas name
different fields for the same concept, that's the exact drift
`schema.sql` catches.

### `scripts/gen-contracts.sh` — inputs, outputs, pinned generators

Bash script with `set -euo pipefail`. Inputs are the authored
sources under `packages/contracts/{events,openapi}/`; outputs
are the generated trees under `packages/contracts/{ts,rust,py}/`.

Pinned versions (OQ 2):

```
DMCG_VERSION=0.26.3               # datamodel-code-generator (Python)
TYPIFY_VERSION=0.2.0              # typify-cli (Rust)
JSTT_VERSION=15.0.4               # json-schema-to-typescript
REDOCLY_VERSION=1.25.11           # @redocly/cli
```

Steps: (0) sanity-check `uv`, `cargo`, `npx` are on `PATH`; (1)
clean and recreate output trees; (2) bundle each event schema
with its envelope `$ref` for generators that don't follow `$ref`;
(3) Python codegen via `uvx --with datamodel-code-generator==$DMCG_VERSION
datamodel-codegen …`; (4) Rust codegen via `cargo install --locked
--version $TYPIFY_VERSION typify-cli` then `typify …`; (5) TS
codegen via `npx --yes --package=json-schema-to-typescript@$JSTT_VERSION
json2ts …`; (6) OpenAPI lint via `npx --yes --package=@redocly/cli@$REDOCLY_VERSION
redocly lint openapi/openapi.v1.yaml`; (7) regenerate per-language
manifests (`pyproject.toml`, `Cargo.toml`, `package.json`) with
`version` pulled from `../../VERSION`; (8) write DO-NOT-EDIT
banner into every generated tree's top-level file.

Complementary `packages/contracts/Makefile` provides `make
contracts` (runs the script), `make lint` (redocly only), `make
test` (runs `pytest tests/`). Root `Makefile` deferred to slice 9;
the package-local Makefile is enough to satisfy `0011:47-49`.

### CI drift-check

New workflow `.github/workflows/contracts.yml` (kept separate
from `ci.yml:33-42` because it needs its own toolchain matrix —
Python 3.12 + Rust stable + Node 20 — and runs against a
different path-filter). Structure:

```yaml
name: contracts
on:
  pull_request:
  push: { branches: [main] }
permissions: { contents: read }
jobs:
  changes:          # dorny/paths-filter — trigger only on contracts diff
  drift:
    needs: changes
    if: needs.changes.outputs.contracts == 'true'
    steps:
      - checkout
      - setup-python (3.12) + install uv
      - setup-rust  (stable)
      - setup-node  (20)
      - cd packages/contracts && make contracts
      - git diff --exit-code -- packages/contracts/{ts,rust,py}
      - make test
```

Cached generator installs via `actions/cache` keyed on
`hashFiles('scripts/gen-contracts.sh')` so steady-state cost is
seconds. Same `dorny/paths-filter@v3` major version as
`ci.yml:19`.

### Contract tests

`packages/contracts/tests/` (per `testing.md:32-34`), Python
because Python is the lightest way to run JSON Schema validation
in CI without pulling a full Node/Rust build. Fixtures are plain
data files (per `testing.md:36-44`).

Tests in `test_schemas.py`:

- `test_schemas_self_validate` — every `events/*.json` parses as
  a valid JSON Schema 2020-12 document (via `jsonschema`).
- `test_examples_round_trip` — every `tests/examples/*.json`
  validates against the schema of the matching filename.
- `test_openapi_lints` — invokes `redocly lint` via subprocess
  (pinned version) — non-zero exit fails.
- `test_envelope_ref_shared` — every stream schema references
  `_envelope.v1.json` via `$ref`, catches the "envelope
  forgotten" regression per OQ 4.
- `test_schema_sql_enums_match` — the `media_kind` enum in
  `schema.sql` matches the JSON-schema `MediaKind` enum,
  catches SQL/JSON drift per §Failure modes.

Invoked from CI as `make test` after the regenerate step. Full
cross-service contract tests grow in slices 6/7/8 when services
exist.

### AGENTS.md §6 entries (OQ 3)

Replace `AGENTS.md:245-247` ("*No overlap-list files yet.*
Add entries…") with:

```
Current entries — CI-enforced for the file-path items via
`.github/workflows/autodev-guard.yml`:

- `packages/contracts/**` — any change to a schema, OpenAPI
  spec, `schema.sql`, or the generator script requires Spec
  lane per ADR 0011.
- `scripts/gen-contracts.sh` — the codegen driver. Toolchain
  version pins live here; a bump reruns codegen against every
  target and lands as a single Spec-lane PR.
- `.github/workflows/contracts.yml` — the drift-check workflow.
  Touching its trigger, matrix, or toolchain versions is a
  Spec-lane change.

Soft entries (human review only — greps for these substrings
have too many false positives to gate CI on):

- Event stream names: `jobs.media.index.v1`,
  `events.media.indexed.v1`, `events.media.index_failed.v1`
  (per ADR 0006). New streams must land with schemas under
  `packages/contracts/events/` in the same PR.
- Stream-name grammar `.v<int>` suffix (per ADR 0006 + ADR
  0017). A `.v<N+1>` bump is a Spec-lane PR that adds a new
  schema file next to the old one; never edits the old in
  place.
```

The introduction sentence at `AGENTS.md:242-244` stays; the
"soft (not CI-enforced)" claim is updated to "soft for review;
CI enforces file-path entries via `autodev-guard.yml`."

### `autodev-guard.yml` overlap-hits block (OQ 3)

Verbatim replacement for `.github/workflows/autodev-guard.yml:114-138`:

```yaml
      - name: Quick lane bounds
        if: steps.escape.outputs.skip == 'false'
        run: |
          set -euo pipefail
          labels="${{ steps.pr.outputs.labels }}"
          if [[ ",$labels," != *",lane:quick,"* ]]; then
            exit 0
          fi
          # Overlap list (AGENTS.md §6). Hard-fail when a Quick PR
          # touches a stabilized public interface — those need Spec.
          overlap_hits="$(grep -E \
            '^(packages/contracts/|scripts/gen-contracts\.sh$|\.github/workflows/contracts\.yml$)' \
            "$RUNNER_TEMP/pr_files.txt" || true)"
          if [[ -n "$overlap_hits" ]]; then
            echo "::error::Quick-lane PR touches overlap-list files — restart in Spec lane (AGENTS.md §6)."
            echo "$overlap_hits"
            exit 1
          fi
          additions=${{ github.event.pull_request.additions }}
          deletions=${{ github.event.pull_request.deletions }}
          total=$(( additions + deletions ))
          if (( additions > 200 || total > 400 )); then
            echo "::warning::Quick-lane PR is large (+$additions -$deletions = $total). Consider escalating to Spec lane (AGENTS.md §4)."
          fi
```

Stream-name grep is deliberately **not** in the workflow —
matching commit diffs for stream-name strings has too many
false positives; those stay soft, human-reviewed per §6 skim
rule.

### SemVer + release-note posture

`packages/contracts/VERSION` — plain text, `0.1.0`.
`packages/contracts/README.md` documents the rules per ADR
`0017:51-58`:

- Pre-1.0 scheme: `0.MAJOR.MINOR-PATCH`.
- MAJOR: any breaking change to a schema — removed field,
  renamed field, changed type, tightened validation, removed
  operation, changed status-code semantics.
- MINOR: additive change — new optional field, new operation,
  new enum variant when consumers with exhaustive matching are
  not forced by the change.
- PATCH: doc-only edits inside schemas
  (`title`/`description`), README, VERSION rewrites for a
  release cut.
- Deprecation window: two MINOR versions between "field marked
  deprecated in docs" and "field allowed to disappear in MAJOR."
- Release notes: append entries to
  `packages/contracts/CHANGELOG.md` at each bump. Initial
  `0.1.0` entry seeded in this slice.
- In-monorepo consumers pin by path (Cargo path dep, uv path
  dep, pnpm workspace) rather than by version; SemVer applies at
  external-release time.

### Alternative considered

**One meta-generator (`quicktype`) for all three target
languages.** Reduces `gen-contracts.sh` to one binary invocation
per schema. Rejected because `quicktype`'s JSON Schema 2020-12
coverage has been thinner than each per-language tool's — some
`if/then/else`, `unevaluatedProperties`, and `$ref` cases lag —
and ADR 0011's promise is deterministic, pinnable regeneration.
A per-language pin against a widely-used tool is safer than a
single tool that lags on schema features. Trade:
`gen-contracts.sh` is longer (three invocations vs one), but
each generator's blast radius is one language.

**Also considered: land the root Makefile in this slice.** Slice
9 (`arch-development-workflow`, per
`.tasks/epics/arch-scaffold/parent.md:50`) owns the top-level
`Makefile`. Slice 3 lands the package-local
`packages/contracts/Makefile` only, so `make contracts` works
from that directory and CI `cd`'s into it. Slice 9 will add a
root-level forward target.

## Tradeoffs accepted

- **Three generators, three release cadences.** Each tool has
  its own upstream. Accepted because a monoculture (`quicktype`)
  trails per-language 2020-12 support.
- **`schema.sql` is a forward-looking spec, not an executed
  migration.** ADR 0016 defers Postgres; the epic asked for a
  "canonical reference." We land the reference so slice 5 has a
  target, at the cost of a file that isn't executable until
  slice 5 wires it in.
- **Envelope drift risk concentrated in one file.** With `$ref`,
  any breakage in `_envelope.v1.json` breaks all three stream
  schemas. Accepted — mitigations in §Failure modes catch it.
- **CI drift-check job is heavier than `ci.yml`'s current
  placeholder** (Python + Rust + Node toolchains). Accepted:
  path-filtered so it only runs when `packages/contracts/**`
  changes; cached installs keep steady-state cost low.
- **Contract tests are Python-only** — no Rust or TS test suite
  for schemas yet. Accepted: JSON Schema validation is
  cross-language; one host language suffices. Rust/TS *use* the
  generated types in slices 6/8, and the compile step there is
  the second line of defence.

## Failure modes

Adversarial re-read.

- **Generator pin drift.** A contributor bumps a pin in
  `gen-contracts.sh` without regenerating.
  *Mitigation:* the drift-check job runs `make contracts` then
  diffs — a bumped pin without a regenerate is a mandatory
  failure.
- **`schema.sql` diverges from JSON Schemas.** A field renamed
  in `events/*.v1.json` but not in `schema.sql` is silent drift
  a reviewer misses.
  *Mitigation:* acceptance criterion
  `test_schema_sql_enums_match` grep-verifies `media_kind` enum
  parity today; full cross-check deferred to slice 5's
  migration wiring.
- **Envelope re-vendored per schema going out of sync with ADR
  0015.**
  *Mitigation:* `_envelope.v1.json` is `$ref`'d by all three
  stream schemas, not copied. Test `test_envelope_ref_shared`
  fails if any stream schema drops the `$ref`.
- **CI drift-check exceeds job timeout.** Three toolchain
  installs on a cold runner can approach 5 minutes.
  *Mitigation:* `actions/cache` keyed on
  `hashFiles('scripts/gen-contracts.sh')`; path-filter skips
  the job entirely for docs-only PRs. Hard timeout 10 min,
  steady-state ~1 min.
- **Contracts package version bump missed by consumers.**
  *Mitigation:* README documents pin-by-path for in-monorepo
  use; SemVer applies at external-release time only.
  Downstream slices cite ADR 0017 when they land.
- **OpenAPI spec grows before services exist.** Reviewer
  temptation to spec `/api/v1/media` etc. here.
  *Mitigation:* acceptance criterion "OpenAPI file has exactly
  one path (`/healthz`)" falsifies scope creep.
- **Slice-6/7/8 later disagree with a chosen generator.**
  E.g., slice 6 wants `schemars`-style reverse generation
  instead of `typify`.
  *Mitigation:* ADR 0011's Consequences already name
  "generator upgrade = one PR that reruns codegen against
  pinned new version" — the pin is reviewable. Not a slice-3
  problem to pre-empt.
- **AGENTS.md §6 hard-fail backfires on a legitimate Quick PR**
  — e.g., fixing a typo in `packages/contracts/README.md`.
  *Mitigation:* the workflow grep matches
  `packages/contracts/` broadly; a README change *is*
  documentation of a stabilized interface, so it correctly
  routes through Spec lane. If proven onerous, tighten the
  regex in a follow-up (which itself would be Spec-lane —
  self-consistent meta-rule).

## Acceptance criteria

Every criterion automated-or-observable with a falsifier.

- [ ] `packages/contracts/events/` contains exactly four schema
  files: `_envelope.v1.json` plus the three canonical stream
  schemas. *Falsified if:*
  `ls packages/contracts/events/*.json | wc -l` returns anything
  other than 4, **or** any filename doesn't match
  `_envelope.v1.json` or one of the names at
  `docs/conventions/events.md:26-30`.
- [ ] Every stream schema `$ref`s `_envelope.v1.json`.
  *Falsified if:*
  `grep -L '_envelope.v1.json' packages/contracts/events/{jobs,events}.*.v1.json`
  returns any file.
- [ ] All JSON schemas declare JSON Schema 2020-12 and the
  OpenAPI file declares `openapi: 3.1.0`. *Falsified if:*
  `grep -L '"\$schema".*2020-12' packages/contracts/events/*.json`
  returns any file, **or** `grep -c '^openapi: 3.1.0$' packages/contracts/openapi/openapi.v1.yaml`
  returns 0.
- [ ] OpenAPI file has exactly one path (`/healthz`) and no
  `/api/v1/*` operations. *Falsified if:*
  `grep -cE '^  /[a-z]' packages/contracts/openapi/openapi.v1.yaml`
  returns anything other than 1, **or** `grep '/api/v1/' packages/contracts/openapi/openapi.v1.yaml`
  returns any operation path.
- [ ] `scripts/gen-contracts.sh` names every pinned generator
  version literally (not `latest` / `^` / `>=`). *Falsified if:*
  `grep -E 'latest|\^|>=' scripts/gen-contracts.sh` matches,
  **or** any of `DMCG_VERSION`, `TYPIFY_VERSION`,
  `JSTT_VERSION`, `REDOCLY_VERSION` is missing.
- [ ] `make contracts` from `packages/contracts/` produces an
  empty `git diff` on a fresh checkout. *Falsified if:*
  `cd packages/contracts && make contracts && git diff --exit-code`
  returns non-zero.
- [ ] CI has a `contracts` workflow that runs `make contracts`
  and diffs. *Falsified if:* `.github/workflows/contracts.yml`
  is missing, **or** `grep -E 'git diff --exit-code' .github/workflows/contracts.yml`
  returns zero hits.
- [ ] `AGENTS.md` §6 lists the CI-enforced entries and the soft
  entries. *Falsified if:* for any of these substrings —
  `packages/contracts/**`, `scripts/gen-contracts.sh`,
  `.github/workflows/contracts.yml`, `jobs.media.index.v1`,
  `events.media.indexed.v1`, `events.media.index_failed.v1`,
  `.v<int>` — `grep -F "<substring>" AGENTS.md` returns zero.
- [ ] `autodev-guard.yml` hard-fails a Quick PR touching
  `packages/contracts/**`. *Falsified if:* `grep -E 'packages/contracts/' .github/workflows/autodev-guard.yml`
  returns zero, **or** the `overlap_hits=""` initializer from
  `autodev-guard.yml:124` is still present.
- [ ] Contract tests pass. *Falsified if:*
  `cd packages/contracts && make test` returns non-zero, **or**
  the CI `contracts` workflow reports the test step as failing.
- [ ] `schema.sql` header explicitly names slice 5 as the
  migration owner and the file contains no indexes or
  migration verbs. *Falsified if:*
  `head -5 packages/contracts/schema.sql | grep -q 'slice 5'`
  returns false, **or**
  `grep -E 'CREATE INDEX|ALTER TABLE|-- migration' packages/contracts/schema.sql`
  matches.
- [ ] Every generated tree carries a "DO NOT EDIT" banner.
  *Falsified if:* `grep -L 'DO NOT EDIT' packages/contracts/ts/*.md packages/contracts/rust/src/lib.rs packages/contracts/py/visloom_contracts/__init__.py`
  returns any file.
