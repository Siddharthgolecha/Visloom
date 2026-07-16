# 0016 — Redis usage: key naming + TTL taxonomy

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Redis appears in the Compose stack (ADR 0004) for three
purposes: **streams** (ADR 0006, indexing transport), **sessions**
(ADR 0005, owner Postgres-backed session cookies — the cookie
itself is opaque, but a hot-path lookup lives in Redis),
**rate-limit counters** (ADR 0014). Without a naming convention
and TTL taxonomy, keys drift across services and expire policies
diverge. Streams are already named per ADR 0006; this ADR
covers the non-stream keys.

Postgres schema evolution is **out of scope** here. A future
ADR alongside slice 5 (`arch-compose-and-infra`) locks Postgres
migration format, up/down policy, and backwards-compat window
when there is a concrete `init.sql` to react to.

## Decision Drivers

* One key namespace, one convention, easy to grep in `MONITOR`
  or a running Redis when debugging.
* TTLs matter — a rate-limit counter that never expires is a
  leak; a session cache that expires wrong is a UX bug.
* Keys must survive the "someone else's service also uses
  Redis" case if we ever share the instance.

## Considered Options

* **No convention.** Every service names its own keys. Drift
  guaranteed.
* **One-key-one-purpose with flat names** (chosen for structure).
* **Namespaced with TTL taxonomy** (chosen for TTL policy).

## Decision Outcome

Chosen: **`vloom:<domain>:<entity>:<id>` naming + a three-class
TTL taxonomy.**

* **Naming:** every Redis key starts with the app prefix
  `vloom:`, followed by a domain segment (`session`,
  `ratelimit`, `share`, etc.), an entity segment
  (`by-token`, `by-account-ip`, ...), and an id.
  Dot-separated inside a segment where needed. Streams stay
  named per ADR 0006 (`jobs.media.index.v1`) — the `vloom:`
  prefix does **not** apply to streams.
* **TTL taxonomy:**
  * **short** (seconds–minutes): rate-limit counters,
    per-request throttles.
  * **medium** (hours): session hot-path lookups, magic-link
    tokens.
  * **long** (days): share-token lookups (bounded by the
    token's own expiry).
* Every key **must** have an explicit TTL — no bare `SET`
  without expiry. Enforced by review + a lint check
  (candidate for slice 6 / slice 7 tooling).
* **Sessions in Redis are a cache over Postgres, not the source
  of truth.** ADR [0005](0005-owner-auth-and-rbac.md) commits
  server-side sessions in Postgres and requires per-device
  revocation. Redis holds a hot-path lookup only. Two rules
  follow, both binding on slice 6's implementation:
  * **Revocation is a two-step write:** delete the Postgres row
    **and** delete (or invalidate) the Redis key. Deleting only
    the Postgres row leaves a session that continues to
    authenticate until the cache TTL runs out — that is the
    exact failure mode we're avoiding.
  * **Every cache read carries a version cross-check.** The
    cached value includes the Postgres row's monotonic
    `session_version` (or equivalent). On lookup, if the
    Postgres version differs from the cache, treat the cache as
    stale and refetch. This is the belt to the revocation
    suspenders — a missed cache-invalidation is caught on the
    next request instead of persisting for hours.
* The same "cache over Postgres, invalidate on write, version-
  check on read" rule applies to any other identity- or
  authorization-shaped key added to Redis later
  (`event_memberships` lookups, share-token validation).
  Rate-limit counters and magic-link tokens are Redis-native
  (no Postgres row) and don't need this treatment.

## Consequences

* `MONITOR` output is greppable by prefix; ops know at a
  glance which service owns a key.
* Slice 5's Compose config sizes Redis for the sum of the
  three TTL classes.
* Postgres schema evolution is deferred — slice 5's follow-up
  ADR must cite this ADR when it lands, and
  `docs/conventions/data.md` has a `## Deferred` section
  pointing at that ADR.
* Downstream: slice 5 brings up the Redis service; slice 6
  uses the `session` and `ratelimit` key classes and
  implements the two-step revocation + version-check contract
  above; slice 7 uses the `jobs.*` streams (per ADR 0006)
  alongside any worker-local scratch keys under
  `vloom:worker:...`.
