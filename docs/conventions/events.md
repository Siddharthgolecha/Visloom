# Event conventions

Transport + naming for the API ↔ worker handshake, per ADR
[0006](../adr/0006-redis-streams-versioned-naming.md).

## Transport

Redis Streams both directions. Every consumer reads via a
**consumer group** so worker replicas share load and un-acked
messages redeliver on crash.

## Naming

* Dot-separated, lowercase segments.
* Kind: `jobs.*` (work requested), `events.*` (state change).
* Domain: `media`, `session`, …
* Action: `index`, `indexed`, `index_failed`.
* Version: `.v<int>` suffix, starts at `.v1`. **Version on the
  stream name, never in the payload.**

## Canonical streams

Contracts land in slice 3 (`packages/contracts/events/`) matching
these exact names:

* `jobs.media.index.v1` — API → worker.
* `events.media.indexed.v1` — worker → API, success (embedding
  pointer inside).
* `events.media.index_failed.v1` — worker → API, failure (reason
  inside).

## Versioning + cutover

Version bump = new stream + new consumer group; old + new run
side by side. Explicit cutover order:

1. Producer starts writing to `.v<N+1>`.
2. Consumers deployed against the new stream.
3. Producer stops writing to `.v<N>`.
4. Old consumer group drained (all messages acked).
5. `XDEL` the old stream in a maintenance step.

Never carry incompatible payloads on the same stream.

## Payload shape

Every message carries:

* `event_id` — ULID.
* `traceparent` — serialized W3C trace context from the
  originating request (per ADR
  [0015](../adr/0015-observability-otel-first.md); this is the
  propagation vehicle so downstream spans can attach as
  children).
* `tracestate` — optional W3C tracestate string, present when
  the originating request had one.
* `trace_id` — derived from `traceparent`, kept as a log-only
  convenience for structured search. Never used for
  propagation.
* `occurred_at` — ISO-8601 UTC.
* `data` — the payload for the specific stream.

Schemas + generated types land in slice 3.
