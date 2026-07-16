# Plan — arch-contracts-package

## Tactical steps

Sequenced for implementation **after** `plan-approved` is granted;
`spec.md` + this `plan.md` are the artifacts under review. All
four OQs are resolved in-spec with proposed answers — this plan
has no `if OQ X` branches. Reviewer may override any OQ during
plan review; the corresponding steps below adjust in the same
review cycle, before `plan-approved`.

1. **Author `packages/contracts/events/_envelope.v1.json`**
   (spec §Event schemas — envelope $ref, OQ 4 proposed
   `$ref`-shared). Field set from `docs/conventions/events.md:45-63`
   + ADR `0015:58-75`. Required: `event_id` (ULID pattern),
   `traceparent`, `occurred_at` (`format: date-time`), `data`.
   Optional: `tracestate`. `additionalProperties: false`;
   `$schema` = JSON Schema 2020-12.
2. **Author the three stream schemas** (spec §Event schemas).
   Each `$ref`s the envelope and overrides `data`. Payload shapes
   per spec: `jobs.media.index.v1` (ADR `0007:38-46` —
   `MediaKind ∈ {photo, video}` discriminator),
   `events.media.indexed.v1` (embedding pointer + frame count),
   `events.media.index_failed.v1` (failure code + retry).
   `additionalProperties: false` at every object level.
3. **Author `packages/contracts/openapi/openapi.v1.yaml`** (spec
   §OpenAPI 3.1 skeleton). `openapi: 3.1.0`; two `servers`
   entries per ADR `0017:44-46` — root for `/healthz`,
   `/api/v1` for the reserved future path. Exactly one path
   (`/healthz`) with 200 body per `docs/conventions/api.md:19-20`.
   Shared components: `ErrorEnvelope`, `HealthResponse`,
   `IdempotencyKey`, and status-code responses (`NotFound`,
   `Conflict`, `Unprocessable`, `RateLimited`, `Internal`,
   `Unavailable`) wrapping `ErrorEnvelope` per
   `docs/conventions/api.md:30-33`.
4. **Author `packages/contracts/schema.sql`** (spec §schema.sql
   posture, OQ 1 proposed forward-looking reference). Header
   comment cites ADR 0016 and names slice 5 as executor. Body:
   `CREATE TABLE` statements for `accounts`, `sessions`,
   `event_owners` / `event_memberships` (per ADR 0008), `media`
   (with `media_kind` enum matching ADR 0007), `idempotency_keys`
   (per `docs/conventions/api.md:35-39`). No indexes, no
   migration verbs, no schema-version header.
5. **Author `scripts/gen-contracts.sh`** (spec §
   `scripts/gen-contracts.sh`, OQ 2 proposed pins).
   `set -euo pipefail`. Pins: `DMCG_VERSION=0.26.3`,
   `TYPIFY_VERSION=0.2.0`, `JSTT_VERSION=15.0.4`,
   `REDOCLY_VERSION=1.25.11`. Steps 0-8 as spec §
   `scripts/gen-contracts.sh` enumerates. Writes DO-NOT-EDIT
   banner into every generated tree.
6. **Author `packages/contracts/Makefile`** (spec §
   `scripts/gen-contracts.sh` — package-local Makefile). Targets:
   `contracts` (calls the script), `lint` (redocly only),
   `test` (pytest). Root Makefile deferred to slice 9
   (`.tasks/epics/arch-scaffold/parent.md:50`).
7. **Author `packages/contracts/{VERSION,README.md,CHANGELOG.md}`**
   (spec §SemVer + release-note posture). `VERSION` = `0.1.0`;
   `README.md` documents bump rules per ADR `0017:51-58`;
   `CHANGELOG.md` seeds the `0.1.0` entry naming this slice's
   PR.
8. **Run the generator once** (spec §Approach — layout).
   `cd packages/contracts && make contracts` produces `ts/`,
   `rust/`, `py/` trees for the first time. Committed as part of
   this PR per ADR `0011:44-49`. If a generator misbehaves on
   the first run, that's a plan-approved regression — halt and
   fix before continuing.
9. **Author contract tests + fixtures** (spec §Contract tests).
   `tests/test_schemas.py` with five test functions:
   `test_schemas_self_validate`, `test_examples_round_trip`,
   `test_openapi_lints`, `test_envelope_ref_shared`,
   `test_schema_sql_enums_match`. `tests/examples/*.json` with
   one minimum-valid fixture per stream schema.
   `tests/conftest.py` for pytest discovery. Fixtures are
   plain data per `docs/conventions/testing.md:36-44`.
10. **Author `.github/workflows/contracts.yml`** (spec §CI
    drift-check). `dorny/paths-filter@v3` on
    `packages/contracts/**` and `scripts/gen-contracts.sh`;
    installs Python 3.12 + Rust stable + Node 20; step order
    install → `make contracts` → `git diff --exit-code -- packages/contracts/{ts,rust,py}` →
    `make test`. `actions/cache` on generator installs keyed on
    `hashFiles('scripts/gen-contracts.sh')`.
11. **Edit `AGENTS.md` §6** (spec §AGENTS.md §6 entries, OQ 3
    proposed hard-fail). Replace the "*No overlap-list files
    yet.*" block at `AGENTS.md:245-247` with the verbatim block
    in spec §AGENTS.md §6 entries. Update the intro sentence at
    `AGENTS.md:242-244` to note CI enforcement of file-path
    entries.
12. **Edit `.github/workflows/autodev-guard.yml`** (spec §
    `autodev-guard.yml` overlap-hits block, OQ 3 proposed
    hard-fail). Replace lines 114-138 with the verbatim block in
    spec.
13. **Verify** (spec §Acceptance criteria). Run every falsifier
    check in order — fresh checkout → `make contracts` → `git
    diff --exit-code` → `make test` → grep checks against
    `AGENTS.md`, `autodev-guard.yml`, `contracts.yml`.
    Cross-check the fork-schema acceptance criteria by grepping
    `packages/contracts/events/` for each of the three canonical
    stream names.

## Files touched

Twenty-plus new files under `packages/contracts/`,
`scripts/`, and `.github/workflows/`; two edits to shared repo
files. Generated trees included with globs.

New:

- `packages/contracts/VERSION` — SemVer, initial `0.1.0`.
- `packages/contracts/README.md` — what the package is + bump
  rules citing ADR 0017.
- `packages/contracts/CHANGELOG.md` — initial `0.1.0` entry.
- `packages/contracts/Makefile` — `contracts`, `lint`, `test`
  targets.
- `packages/contracts/events/_envelope.v1.json` — shared
  envelope subschema (OQ 4).
- `packages/contracts/events/jobs.media.index.v1.json` — API →
  worker indexing job schema.
- `packages/contracts/events/events.media.indexed.v1.json` —
  worker → API success event schema.
- `packages/contracts/events/events.media.index_failed.v1.json`
  — worker → API failure event schema.
- `packages/contracts/openapi/openapi.v1.yaml` — OpenAPI 3.1
  skeleton, one path (`/healthz`) + shared components.
- `packages/contracts/schema.sql` — forward-looking Postgres
  reference (OQ 1).
- `packages/contracts/ts/**` — generated TypeScript bindings.
- `packages/contracts/rust/**` — generated Rust bindings.
- `packages/contracts/py/**` — generated Python bindings.
- `packages/contracts/tests/test_schemas.py` — five test
  functions per spec §Contract tests.
- `packages/contracts/tests/conftest.py` — pytest discovery.
- `packages/contracts/tests/examples/*.json` — one fixture per
  stream schema (three files).
- `scripts/gen-contracts.sh` — codegen driver with pinned
  generator versions (OQ 2).
- `.github/workflows/contracts.yml` — drift-check workflow.

Edited:

- `AGENTS.md` — replace `AGENTS.md:245-247` with the §6
  entries block from spec, per OQ 3. Update intro at
  `AGENTS.md:242-244` to note CI enforcement.
- `.github/workflows/autodev-guard.yml` — replace the commented
  overlap-hits block at lines 114-138 with the live check from
  spec, per OQ 3.

## Depends on

- Part-2 conventions + ADRs (PR #12, merged `ce46558`). Slice 3
  cites `docs/conventions/{events,api,data,testing,errors}.md`
  and ADRs 0003, 0006, 0007, 0009, 0011, 0015, 0016, 0017.
  Resolved.
- No parallel PR touches `packages/contracts/` today (checked
  `gh pr list --search 'is:draft'` at branch-open time —
  only `#14` (this PR itself) is a draft in that scope; `#13`
  landed and is not in the contracts path). Overlap re-checked
  before requesting `plan-approved`.
