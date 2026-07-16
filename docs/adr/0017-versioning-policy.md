# 0017 — Versioning policy: URL-path + event streams + contracts

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Three surfaces need a versioning story: the **HTTP API** (browser
+ web app), **event streams** (worker handshake), and the
**generated contracts package** (typed bindings imported by every
runtime). Without one policy, breaking changes ripple
unpredictably. Part-1 (ADR 0006, `docs/conventions/api.md`)
already committed pieces of this; this ADR unifies them into
one document downstream slices can cite.

## Decision Drivers

* Any breaking change must be observable at the point of use —
  no silent renames of a JSON field or an endpoint path.
* Version bumps should be a build-time, reviewable event, not
  runtime magic.
* Contributors should not have to reason about three unrelated
  versioning schemes — the story should be uniform across the
  three surfaces.

## Considered Options

* **Header-based versioning** for HTTP (`Accept:
  application/vnd.visloom.v1+json`). Rejected — hard to test in
  a browser URL bar; adds a full negotiation layer.
* **Content-negotiation for everything.** Rejected — same as
  above, plus adds moving parts.
* **URL-path + stream-suffix + SemVer** (chosen).

## Decision Outcome

Chosen: **three-axis versioning, one philosophy.**

* **HTTP API — URL-path.** All application routes live under
  `/api/v1/` (per `docs/conventions/api.md`). A breaking change
  bumps the prefix to `/api/v2/`; `/api/v1/` keeps serving
  until deprecated. Same coexist-then-cutover rule as event
  streams. `/healthz` and `/readyz` sit above the version
  prefix (unversioned).
* **Event streams — `.v<int>` suffix on the stream name** (per
  ADR 0006). Breaking payload changes create a new stream +
  new consumer group; old and new run side by side, then old
  is dropped in an explicit cutover step (documented in
  `docs/conventions/events.md`).
* **Generated contracts — SemVer on the contracts package.**
  The `packages/contracts/` package (slice 3) carries its own
  version number (`0.MAJOR.MINOR-PATCH` while the app is
  pre-1.0). A breaking change to any schema bumps MAJOR; a
  new field is MINOR; internal doc-only changes are PATCH.
  Slice 3's plan must cite this ADR and stay compatible when
  it lands the concrete wire format.

Common philosophy: **versions coexist; cutovers are explicit.**
Never carry an incompatible change on the same version.

## Consequences

* Web (slice 8) hard-codes `/api/v1/` in its client base URL;
  a v2 rollout means a client update alongside the API
  deploy.
* Worker consumer groups (slice 7) reference stream names by
  full `.v<int>` — a new stream is a new subscription, not a
  config flip.
* Contracts consumers (any slice) pin the package version in
  `Cargo.toml` / `pyproject.toml` / `package.json`.
* Slice 3 owns the concrete SemVer bump rules for the
  contracts package (release-note format, deprecation window)
  when it lands the wire format.
* Downstream: slice 3 (contracts package) cites this ADR;
  slice 6 (API) implements the `/api/v1/` prefix; slice 8
  (web) hard-codes the same prefix.
