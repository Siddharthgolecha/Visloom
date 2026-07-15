# 0006 — Redis Streams for indexing + versioned stream naming

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

API hands media to worker for embedding; search returns worker-
produced embeddings. Transport must be durable, replayable, and
cheap to run on one VPS (ADR 0004). Names need a versioning
scheme so a breaking payload change can't corrupt an in-flight
consumer.

## Decision Drivers

* At-least-once + ack + consumer group.
* Runs inside the single-VPS Compose stack (ADR 0004).
* Payload change must be safe to roll out with old + new
  consumers coexisting.

## Considered Options

* **RabbitMQ** — AMQP broker, richer routing, extra service.
* **Postgres LISTEN/NOTIFY** — no new service, no ack, no replay.
* **Redis Streams** — durable log + consumer groups, already in
  stack (chosen — transport).
* **Stream-name vs payload-field versioning** (chose stream-name
  — naming).

## Decision Outcome

### Decision Outcome — Transport

**Redis Streams** both directions. Already in stack; `XACK` gives
at-least-once; consumer groups let slice 7 scale horizontally.
RabbitMQ = extra service; pg LISTEN/NOTIFY can't replay.

### Decision Outcome — Naming

**Version on the stream name** as `.v<int>`. Dot-separated,
lowercase. A breaking change bumps `<int>`, creating a **new
stream + new consumer group** — old and new coexist, then the old
is dropped. Payload-field versioning would let one stream carry
incompatible messages.

**Canonical streams** (contracts land in slice 3):

* `jobs.media.index.v1` — API → worker.
* `events.media.indexed.v1` — worker → API, success.
* `events.media.index_failed.v1` — worker → API, failure.

## Consequences

* Durable + replayable + no new service.
* Version bumps explicit (new stream), no rollout ambiguity.
* Dropping an old stream is a manual cutover step — documented
  in `docs/conventions/events.md`.
* Downstream: slice 3 lands `packages/contracts/events/
  *.v1.json` and adds these names + the `.v<int>` rule to
  `AGENTS.md` §6 overlap list; slice 5 brings up Redis; slice 7
  implements the consumer loop.
