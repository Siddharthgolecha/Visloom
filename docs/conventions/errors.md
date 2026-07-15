# Error conventions

Unified error taxonomy across `services/api/`,
`services/worker/`, and `apps/web/`. Anchored by ADR
[0002](../adr/0002-layered-hexagonal-architecture.md) (layering
determines where errors live) and mirroring the wire envelope
from [api.md](api.md).

## Two layers of errors

* **Domain errors** live under `domain/` and are their own
  type per language:
  * Rust: an `enum` with variants per failure mode.
  * Python: a subclass of a project-root `VisloomError`.
  * TypeScript: a tagged union with a `kind` discriminator.
* **Wire errors** are the JSON envelope
  ([api.md §Errors](api.md)) the API returns to browsers. The
  envelope's `code` is stable and machine-readable; `message`
  is human-readable and may change.

## Mapping domain → wire

The map from domain error to wire error is an **adapter
concern**, not a domain concern. Adapters translate external
failures into domain errors on the way in; the HTTP boundary
adapter translates domain errors into wire envelopes on the
way out. `application/` handlers only ever see domain errors.

Rules:

* No adapter swallows an error. If it can't map, it re-raises
  and the framework's error handler renders a 500 + logs it.
* Every domain-error variant maps to exactly one wire `code` +
  HTTP status pair. The mapping is a table in each service's
  boundary adapter — reviewable in one place.
* No PII in wire messages. Include a `trace_id` so support
  can look up the details from the logs.

## Status vocabulary

Reuses the HTTP vocabulary from [api.md §Errors](api.md): 400,
401, 403, 404, 409, 422, 429, 500, 503. Domain errors that
don't fit map to 500 (bug, not user-visible).
