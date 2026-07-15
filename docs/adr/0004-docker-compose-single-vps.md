# 0004 — Docker Compose on a single VPS

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Initial target: one owner-managed event + its attendees. Traffic
and durability fit inside a single VPS with pg + Redis + Caddy.
k8s now trades ops complexity for scale we don't have.

## Decision Drivers

* Dev/prod parity — broken deploy reproduces locally.
* One-command bring-up for a contributor with only Docker.
* No secondary control plane.

## Considered Options

* **Kubernetes** (k3s or managed) — orchestration, HPA, rollouts.
* **Fly.io / Railway** — PaaS, per-service deploys.
* **Docker Compose, single VPS, dev + prod overlays** (chosen).

## Decision Outcome

Chosen: **Compose with two overlay files** — `compose.yml` +
`compose.prod.yml`. Same service defs local and prod; overlay
flips image tags, limits, volumes. Slice 5 lands the initial
`infra/compose/` tree (pg + redis + caddy + otel); slices 6/7/8
add their services additively.

## Consequences

* Dev/prod parity; one-command bring-up.
* `docker compose down -v && make up` = full reset.
* No zero-downtime rollouts — restarts are seconds, acceptable.
* Single VPS is a single point of failure — acceptable at scale;
  supersede if we outgrow it.
* Downstream: slice 5 implements the overlay pattern; slice 9
  wires `make up` / `make down` / `make db-reset`.
