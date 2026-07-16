# Changelog — `packages/contracts/`

Format follows [ADR 0017](../../docs/adr/0017-versioning-policy.md). Newest at top.

## 0.1.0 — 2026-07-16

Initial release. Slice 3 (`arch-contracts-package`, PR #14) lands the
package scaffold, the first three event schemas, the OpenAPI 3.1
skeleton, the forward-looking `schema.sql` reference, the pinned
generator script, contract tests, and the CI drift-check workflow.

### Added

- [`events/_envelope.v1.json`](events/_envelope.v1.json) — shared Redis Stream envelope subschema.
- [`events/jobs.media.index.v1.json`](events/jobs.media.index.v1.json) — API → worker indexing job.
- [`events/events.media.indexed.v1.json`](events/events.media.indexed.v1.json) — worker → API success event.
- [`events/events.media.index_failed.v1.json`](events/events.media.index_failed.v1.json) — worker → API failure event.
- [`openapi/openapi.v1.yaml`](openapi/openapi.v1.yaml) — OpenAPI 3.1 skeleton (one path `/healthz`, shared components).
- [`schema.sql`](schema.sql) — forward-looking Postgres reference (executed by slice 5).
- Generated bindings under [`ts/`](ts/), [`rust/`](rust/), [`py/`](py/).
- Contract tests under [`tests/`](tests/) with pinned `pytest` + `jsonschema` deps.
