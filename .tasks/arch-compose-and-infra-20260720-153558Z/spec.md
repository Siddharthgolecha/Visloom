# Spec — arch-compose-and-infra

<!-- The contract for this task: what it should do, why, and how we'll know.
     Frozen at `plan-approved`. After that, deviations go in implementation.md. -->

## Context

Ship infra/compose/{compose.yml,compose.prod.yml,.env.example}, Postgres init with pgvector, Redis conf, Caddyfile (same-origin), OTel collector config, infra/models/README.md. Only pg + redis + caddy + otel services — API/worker/web arrive in later slices via additive edits.

## Open Questions for the Human

<!-- ≤5 high-leverage questions. Write these BEFORE re-reading the original
     task description, so codebase-first questions dominate over ticket framing. -->

1.
2.
3.

## Research findings

<!-- What already exists here that's reusable. Every claim anchored to a
     specific file:line, existing skill, or references/*.md — no free-form opinion. -->

## Approach

<!-- Chosen approach + at least one alternative considered.
     Say why the chosen one wins over the alternative. -->

### Alternative considered

<!-- The runner-up. Why we didn't pick it. -->

## Tradeoffs accepted

<!-- What we're giving up to take this approach. -->

## Failure modes

<!-- What could break, what feels fragile. Adversarial re-read of your own approach.
     Concrete, actionable — not a generic risk list. -->

## Acceptance criteria

<!-- Each criterion is automated-or-observable, with an explicit "what would falsify it" line. -->

- [ ] Criterion. *Falsified if:* …
- [ ] Criterion. *Falsified if:* …
