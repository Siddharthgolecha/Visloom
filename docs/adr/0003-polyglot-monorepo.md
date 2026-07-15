# 0003 — Polyglot monorepo, no meta-tool

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Three runtimes (Rust API, Python worker, Next.js web) share one
event contract (ADR 0006). Need a repo shape that keeps them
visible to each other without a build meta-tool layered on top of
`cargo` / `uv` / `pnpm`.

## Decision Drivers

* Atomic PRs across runtimes when contracts change.
* Native tooling stays primary; no wrapper to learn.
* CI must detect generated-contract drift (ADR 0006, slice 3).

## Considered Options

* **Polyrepo** — one repo/runtime, versioned contract packages.
* **Nx / Turborepo** — meta-tool, task cache, affected graph.
* **Plain polyglot monorepo** — native tooling + top-level
  `Makefile` (chosen).

## Decision Outcome

Chosen: **plain polyglot monorepo**. Top-level `apps/web/` (pnpm),
`services/api/` (Cargo), `services/worker/` (uv),
`packages/contracts/`, `infra/`. Cross-cutting tasks in a
top-level `Makefile` (slice 9). No Nx, no Turborepo, no Bazel.

## Consequences

* Atomic cross-runtime PRs; reviewers see full blast radius.
* No meta-tool learning tax.
* CI rebuilds everything per push — acceptable at current scale.
* Downstream: slice 6 lands the root `Cargo.toml` workspace;
  slice 8 lands `package.json` + `pnpm-workspace.yaml`; slice 9
  lands the `Makefile`; slice 3 lands `packages/contracts/` and
  the first `AGENTS.md` §6 overlap-list entries.
