# 0002 — Layered + hex-where-appropriate; VSA + lightweight CQRS

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

Three runtimes (Rust API, Python worker, Next.js web) need a
shared structural vocabulary before slices 6–8 land. Pure layered
tangles the worker's external effects with domain logic; strict
hex-everywhere taxes trivial read routes. We want the tax where it
pays off.

## Decision Drivers

* External-effect flows (indexing, auth) must be swappable.
* Query flows must not pay a ports+adapters tax.
* Documentable in one page so slices 6–8 don't re-litigate.

## Considered Options

* **Pure layered** — no ports.
* **Hex-everywhere** — every effect through a port.
* **Layered + hex-where-appropriate, VSA + lightweight CQRS**
  (chosen).

## Decision Outcome

Chosen: **layered baseline + hex where side-effects justify it,
organized as Vertical Slices per feature, with a lightweight CQRS
rubric.**

* **Baseline** per runtime: `domain/`, `application/`,
  `adapters/`, `telemetry/`.
* **VSA** as the feature-encapsulation unit. Simple slice = one
  file; complex slice = folder with handler, DTOs, port, adapter.
* **CQRS rubric**: **reads** default to layered; **writes with
  external effects** (bus, ML, third-party) go through hex ports.
  Stated as a **default**, not an invariant — non-fitting
  features call it out in their `implementation.md`.

## Consequences

* Reviewers know where a feature lives (a slice folder) and when
  to expect a port (write + external effect).
* Query paths stay short; tax lands only where it pays off.
* Judgement-call boundary — slices 6/7 will surface concrete
  cases. If the rubric proves wrong, this ADR gets
  `Superseded by`.
* Downstream: slice 6 creates the four Rust module dirs; slice 7
  creates `domain/ports/*.py` ABCs + adapter stubs; slice 8
  applies VSA at the `app/` route-group level.
