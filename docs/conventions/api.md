# HTTP API conventions

Rules shared by the Rust API (slice 6) and the Next.js web app
(slice 8). OpenAPI 3.1 skeleton lands in slice 3
(`packages/contracts/openapi/`).

## URL shape

* Base path: `/api/v1/`. No header-based versioning.
* Version bump = new prefix (`/api/v2/`); `/api/v1/` keeps
  serving until deprecated. Same coexist-then-cutover rule as
  event streams ([events.md](events.md)).
* Same-origin: Caddy proxies `/api/*` to the API service (ADR
  [0004](../adr/0004-docker-compose-single-vps.md)). Browsers
  never talk to the API on a separate port.

## Health

* `GET /healthz` — public, unauthenticated, 200 with
  `{"status":"ok","embedder":{"model_id":...,"version":...}}`.
* `GET /readyz` — reports upstream deps (Postgres, Redis).
  Later slice.

## Errors

```json
{ "error": { "code": "media_not_found", "message": "…", "trace_id": "…" } }
```

`code` is stable, `message` may change. Status vocabulary: 400
(validation), 401 (unauth), 403 (forbidden), 404 (missing), 409
(conflict), 422 (semantic), 429 (rate-limited), 500 (internal),
503 (upstream).

## Idempotency

Mutating endpoints accept an `Idempotency-Key` header (client
ULID). API stores key + response for a bounded window, replays on
retry. Storage lands in slice 6.

## Auth

Session cookie only — opaque token, HTTP-only, `Secure`,
`SameSite=Lax` — per ADR
[0005](../adr/0005-auth-oauth-and-password.md). No bearer tokens
on browser routes; a future ADR may add m2m bearer support.
