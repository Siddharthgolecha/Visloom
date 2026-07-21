# Data conventions

Redis key naming + TTL taxonomy. Anchored by ADR
[0016](../adr/0016-redis-usage.md). Streams stay in
[events.md](events.md).

## Redis keys

Every non-stream key starts with the app prefix:

```
vloom:<domain>:<entity>:<id>
```

* `<domain>` — semantic group (`session`, `ratelimit`, `share`,
  `worker`, …).
* `<entity>` — sub-classifier (`by-token`, `by-account-ip`, …).
* `<id>` — the specific identifier.

Streams do **not** carry the `vloom:` prefix — they follow the
naming rule in [events.md](events.md).

## TTL taxonomy

Every key **must** have an explicit TTL. No bare `SET`. Three
classes:

| Class | Duration | Examples |
|---|---|---|
| short | seconds–minutes | rate-limit counters, per-request throttles |
| medium | hours | session hot-path lookups, magic-link tokens |
| long | days | share-token lookups (bounded by token's own expiry) |

## Postgres

Migration naming, up/down policy, and backwards-compat window are
locked in [ADR 0020](../adr/0020-postgres-migration-format.md):
sequential numeric migrations under `services/api/migrations/`,
up-only, no down-migrations. The `embeddings` table's
`vector(512)` column is an explicitly provisional dimension per
that ADR — slice 7 supersedes it once the real embedder model is
chosen.
