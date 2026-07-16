# `packages/contracts/`

Canonical wire description for Visloom. Every service reads from
here; nothing here reads from a service.

## What lives where

| Path | What it is | Authored or generated |
|---|---|---|
| [`events/`](events/) | JSON Schema 2020-12 files for Redis Stream payloads (per [`docs/conventions/events.md`](../../docs/conventions/events.md)). | Authored. |
| [`openapi/openapi.v1.yaml`](openapi/openapi.v1.yaml) | OpenAPI 3.1 skeleton for the HTTP API (per [`docs/conventions/api.md`](../../docs/conventions/api.md)). | Authored. |
| [`schema.sql`](schema.sql) | Forward-looking Postgres reference (per [ADR 0016](../../docs/adr/0016-redis-usage.md)). NOT an executed migration — slice 5 owns migration format. | Authored. |
| [`ts/`](ts/), [`rust/`](rust/), [`py/`](py/) | Language bindings derived from the JSON schemas. | Generated — do not edit. |
| [`tests/`](tests/) | Contract-tier tests (schema self-validation, example round-trip, envelope $ref, OpenAPI lint, SQL/JSON enum parity). | Authored. |

The generators are pinned in [`scripts/gen-contracts.sh`](../../scripts/gen-contracts.sh) per [ADR 0011](../../docs/adr/0011-generated-contracts-committed-with-drift-check.md).

## Regenerating

From this directory:

```
make contracts    # rerun the pinned generators, rewrite ts/ rust/ py/
make test         # contract tests
make lint         # redocly lint on the OpenAPI file
```

CI enforces `make contracts && git diff --exit-code` — if you touched a schema, commit the regenerated files in the same PR.

## Versioning

The package version lives in [`VERSION`](VERSION). Rules per [ADR 0017](../../docs/adr/0017-versioning-policy.md):

- Pre-1.0 scheme: `0.MAJOR.MINOR-PATCH`.
- **MAJOR** — any breaking change to a schema: removed field, renamed field, changed type, tightened validation, removed operation, changed HTTP status semantics.
- **MINOR** — additive change: new optional field, new operation, new enum variant (only if consumers with exhaustive matching are not forced by the change; otherwise MAJOR).
- **PATCH** — doc-only edits inside schemas (`title`/`description`), README updates, VERSION rewrites for a release cut.
- **Deprecation window**: two MINOR versions between "field marked deprecated in docs" and "field allowed to disappear in MAJOR."
- **Release notes**: append entries to [`CHANGELOG.md`](CHANGELOG.md) at each bump.
- **In-monorepo consumers pin by path** — Cargo path dep, uv path dep, pnpm workspace — not by version. SemVer applies at external-release time only.

## Event streams (soft overlap list, [AGENTS.md](../../AGENTS.md) §6)

Canonical stream names anchored in [ADR 0006](../../docs/adr/0006-redis-streams-versioned-naming.md):

- `jobs.media.index.v1` — API → worker.
- `events.media.indexed.v1` — worker → API, success.
- `events.media.index_failed.v1` — worker → API, failure.

A `.v<N+1>` bump lands a **new** schema file next to the old one — never edits the old file in place. Cutover order lives in [`docs/conventions/events.md`](../../docs/conventions/events.md).
