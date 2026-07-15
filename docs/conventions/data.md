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

**Deferred.** Postgres schema evolution (migration naming,
transaction boundaries, backwards-compat window) is out of scope
for this doc. A follow-up ADR alongside slice 5
(`arch-compose-and-infra`) locks the migration format once there
is a concrete `init.sql` to react to.

Until that ADR lands, `services/api/migrations/` and any
`init.sql` under `infra/compose/` are unversioned — do not
depend on a naming convention that hasn't been decided.
