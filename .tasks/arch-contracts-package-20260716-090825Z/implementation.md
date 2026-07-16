# Implementation — arch-contracts-package

## Task tree

- [x] Step 1 — `_envelope.v1.json` (plan.md #1) *[tested-with-fixture]* — Required all five `events.md`-canonical fields (`event_id`, `traceparent`, `trace_id`, `occurred_at`, `data`) + optional `tracestate`. `trace_id` pattern `^[0-9a-f]{32}$`; `traceparent` full W3C regex.
- [x] Step 2 — three stream schemas (plan.md #2) *[tested-with-fixture]* — Each uses `allOf: [$ref envelope]` plus `unevaluatedProperties: false` to compose envelope + overridden `data`. `data` shapes exactly as spec §Event schemas.
- [x] Step 3 — `openapi/openapi.v1.yaml` (plan.md #3) *[tested-against-real-input]* — Single `/healthz` path with per-operation `servers: [/]` override so root health sits above `/api/v1/`. Shared `ErrorEnvelope`, `HealthResponse`, `IdempotencyKey`, and six status-code responses reserved for slice 6 to `$ref`. Redocly 2.39.0 lints clean under `openapi/redocly.yaml`.
- [x] Step 4 — `schema.sql` (plan.md #4) *[untested-assumption]* — Six tables (`accounts`, `sessions`, `events`, `event_memberships`, `share_tokens`, `media`, `idempotency_keys`). ADR-0008 tables named literally. `media.media_kind` enum matches JSON schema (checked by `test_schema_sql_enums_match`).
- [x] Step 5 — `scripts/gen-contracts.sh` (plan.md #5) *[tested-against-real-input]* — Runs `make contracts` end-to-end on this workstation: bundle → Python (datamodel-code-generator 0.68.1 → Pydantic v2 via `--use-annotated` `--field-constraints`) → Rust (cargo-typify 0.7.0) → TypeScript (json-schema-to-typescript 15.0.4) → OpenAPI lint (@redocly/cli 2.39.0). See Deviations §1-3.
- [x] Step 6 — `packages/contracts/Makefile` (plan.md #6) *[tested-against-real-input]* — `contracts` / `lint` / `test` / `clean` / `help`. `REPO_ROOT := $(shell git rev-parse --show-toplevel)`.
- [x] Step 7 — `VERSION` / `README.md` / `CHANGELOG.md` (plan.md #7) *[inherited-from-existing-code]* — Version `0.1.0`. README documents SemVer rules per ADR 0017:51-58. CHANGELOG seeds the `0.1.0` entry.
- [x] Step 8 — `make contracts` clean run (plan.md #8) *[tested-against-real-input]* — Produced `ts/events/*.ts`, `rust/src/events/*.rs` + `mod.rs` + `lib.rs`, `py/visloom_contracts/events/*.py`, and per-language manifests. rustfmt component installed as prerequisite; documented in Deviations §4.
- [x] Step 9 — tests + deps + fixtures (plan.md #9) *[tested-against-real-input]* — `tests/pyproject.toml` pins `pytest==9.1.1`, `jsonschema==4.26.0`, `referencing==0.36.2`, `pyyaml==6.0.3`. `tests/uv.lock` committed. Six test functions (added `test_openapi_is_31` beyond the five in the plan — pure additive check). Three fixture ULIDs use the Crockford alphabet correctly.
- [x] Step 10 — `.github/workflows/contracts.yml` (plan.md #10) *[untested-assumption]* — Path-filtered on `packages/contracts/**` + `scripts/gen-contracts.sh` + the workflow itself. `defaults.run.working-directory: packages/contracts`. Toolchain: Python 3.12 + `astral-sh/setup-uv@v3` (pinned uv 0.11.7) + Rust stable with rustfmt + Node 20. Cache on cargo-typify binary keyed by `hashFiles('scripts/gen-contracts.sh')`. Steps: `make contracts` → `git diff --exit-code -- ts rust py` → `make test`.
- [x] Step 11 — AGENTS.md §6 (plan.md #11) *[tested-against-real-input]* — Replaced the "*No overlap-list files yet.*" placeholder with the CI-enforced entries + soft entries block from spec. Intro updated to note CI enforcement.
- [x] Step 12 — `.github/workflows/autodev-guard.yml` (plan.md #12) *[tested-against-real-input]* — Replaced the `overlap_hits=""` placeholder with the live grep from spec §autodev-guard overlap-hits block. `packages/contracts/`, `scripts/gen-contracts.sh`, `.github/workflows/contracts.yml` now hard-fail Quick PRs.
- [x] Step 13 — verify (plan.md #13) *[tested-against-real-input]* — All 16 spec-`## Acceptance criteria` falsifiers pass locally on this workstation. `make contracts` idempotent (second run produces empty diff against committed generated trees). `make test`: 13/13 tests pass. Live PR draft — CI drift-check will run on push.

## Deviations

All non-structural per AGENTS.md §5 step 7 (no acceptance-criterion
change, no scope creep, no contradicted approach).

- **§1 — `TYPIFY_VERSION` swap.** spec.md §OQ 2 pinned `typify-cli@0.2.0`. That crate name does not exist on crates.io; the CLI is packaged as `cargo-typify` (current stable `0.7.0`). Same purpose, corrected upstream identity. Pin literal is preserved in `scripts/gen-contracts.sh`; `TYPIFY_VERSION=0.7.0`.
- **§2 — `DMCG_VERSION` bump.** spec.md §OQ 2 pinned `datamodel-code-generator==0.26.3`. User instruction "Use the last versions of libraries" during implementation prompted a bump to `0.68.1`. New version required `--use-annotated --field-constraints` to emit `Annotated[str, Field(pattern=...)]` instead of the `constr(...)`-in-type-position pattern that trips modern type checkers. Same purpose, same pinned-literal shape.
- **§3 — `REDOCLY_VERSION` bump.** Same user instruction. Bumped `@redocly/cli` from `1.25.11` to `2.39.0`. Required adding `openapi/redocly.yaml` with `no-unused-components`, `security-defined`, `operation-4xx-response`, `info-license` rules suppressed — the shared components (error envelope, status-code responses, `IdempotencyKey`) are reserved for slice 6 to `$ref`, `/healthz` is deliberately unauthenticated per api.md:19-20, `/healthz` has no error responses at slice 3, and license posture is deferred to product.
- **§4 — rustfmt prerequisite.** `cargo-typify` shells to `rustfmt` at codegen time; installed via `rustup component add rustfmt` locally. `.github/workflows/contracts.yml` already declares `components: rustfmt` on `dtolnay/rust-toolchain@stable`. No source-of-truth change.
- **§5 — Envelope bundling for `$ref`-shy generators.** Spec §`scripts/gen-contracts.sh` step 2 anticipated bundling for generators that don't follow bare relative `$ref`s. In practice all three per-language generators (cargo-typify, json-schema-to-typescript, datamodel-code-generator's multi-file mode) needed the bundled inputs — the source-of-truth stream schemas keep their bare `$ref`, and a small inline Python bundler (embedded in `gen-contracts.sh`) inlines the envelope into a run-scoped tmp dir before invoking the generators. Contract test `test_envelope_ref_shared` still validates the source-of-truth files hold the `$ref` shape.
- **§6 — Extra test `test_openapi_is_31`.** Added a sixth test function on top of the five listed in plan §Contract tests. It parses `openapi/openapi.v1.yaml` and asserts `openapi == "3.1.0"`. Additive check; falsifies acceptance criterion "OpenAPI file declares `openapi: 3.1.0`" from spec §Acceptance criteria via runtime rather than grep.

## Summary

Slice 3 ships `packages/contracts/` end-to-end: shared event
envelope + three canonical stream schemas + OpenAPI 3.1 skeleton +
forward-looking `schema.sql` reference + `scripts/gen-contracts.sh`
(pinned Python / Rust / TypeScript / OpenAPI generators) + package-
local `Makefile` + committed generated bindings in `{ts,rust,py}/`
+ six Python contract tests behind `pytest`/`jsonschema` pinned via
`uv.lock` + `.github/workflows/contracts.yml` drift-check + hard-
fail overlap-list enforcement in `autodev-guard.yml`.

All 16 spec-`## Acceptance criteria` falsifiers pass locally.
`make contracts` reproduces `{ts,rust,py}/` deterministically;
CI `git diff --exit-code` gates any drift. `make test` runs
13/13 green. Contracts package version `0.1.0` seeded.

Deviations from spec are documented above and are all non-
structural (upstream tool identity corrections, dep version
bumps under user instruction to use latest, plus one extra
additive test).
