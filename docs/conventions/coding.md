# Coding conventions

Cross-language rules for `services/api/` (Rust),
`services/worker/` (Python), `apps/web/` (TypeScript). Language-
specific style lands with slices 6–8.

## Layout

Per ADR [0002](../adr/0002-layered-hexagonal-architecture.md),
each runtime:

* `domain/` — pure types, no I/O, no framework imports.
* `application/` — feature handlers (Vertical Slices).
* `adapters/` — one adapter per external effect.
* `telemetry/` — logging, tracing, metrics.

Reads default to layered. Writes with external effects go through
a `domain/ports/` port + `adapters/` implementation.

## Naming

* Files/dirs: `snake_case` (Rust, Python), `kebab-case` (TS route
  segments), `PascalCase.tsx` (React components).
* Types: `PascalCase`. Constants: `SCREAMING_SNAKE_CASE`.
* Feature (VSA) names are verbs: `create_event`, `process_media`.

## Errors

Domain errors are their own type (Rust `enum`, Python domain-root
subclass, TS tagged union). Adapters map external errors to
domain errors before returning. No swallowed exceptions.

## Logging + tracing

Structured only (JSON in prod, human-readable in dev). One line
per notable event. Trace ids propagate through adapters.

## Dependency injection

Wired at the service boundary (`main.rs`, `__main__.py`,
`bootstrap.ts`). Handlers receive deps as args — never a global
lookup.

## Testing

Tests co-located with code (`#[cfg(test)]`, `test_*.py`,
`*.test.ts`). Integration tests hit real Postgres + Redis (ADR
[0004](../adr/0004-docker-compose-single-vps.md)), not mocks.
