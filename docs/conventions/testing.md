# Testing conventions

Test tiers + adjacency + fixture policy. Extracts and expands
[coding.md §Testing](coding.md). Anchored by ADR
[0002](../adr/0002-layered-hexagonal-architecture.md) — the
layering decides where each tier's tests live.

## Three tiers

* **Unit** — pure `domain/` code. No I/O, no ports, no
  network. Fast, deterministic, run on every push.
* **Integration** — exercises `application/` handlers against
  real `adapters/` connected to real Postgres + Redis (Compose
  services from ADR [0004](../adr/0004-docker-compose-single-vps.md)).
  No mocks — matches `AGENTS.md`'s "integration tests must hit
  a real database" guidance.
* **Contract** — validates the wire between services. Generated
  from `packages/contracts/` (slice 3) so a contract change
  fails the test suite until every service updates. See ADR
  [0011](../adr/0011-generated-contracts-committed-with-drift-check.md).

## Adjacency

Tests live next to the code they cover:

* Rust: `#[cfg(test)] mod tests` in the same file for unit;
  `tests/` at crate root for integration.
* Python: `test_*.py` co-located next to the module under test.
* TypeScript: `*.test.ts` co-located next to the module under
  test.

Contract tests live under `packages/contracts/tests/` (slice 3)
because they are cross-service.

## Fixtures

Fixtures are **plain data** — a struct literal, a JSON file, a
factory function. No fixture frameworks (no `pytest.fixture`
magic, no `#[fixture]` macros). This keeps fixture setup
readable and greppable.

If a test needs a Postgres row, it inserts it inline via the
same adapter code the app uses. If it needs a Redis key, same.
