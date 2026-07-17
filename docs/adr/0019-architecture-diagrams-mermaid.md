# 0019 — Architecture diagrams: inline Mermaid, flowchart-styled C4

* Status: Accepted
* Date: 2026-07-17
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

The architecture lives only as 18 ADRs plus the convention docs — there
is no single page a contributor can read to grasp the whole system, and
the repo has no diagrams of any kind. Slice 4 (`arch-diagrams-and-overview`)
ships `docs/architecture/overview.md` with seven diagrams, which forces a
format decision: which diagram tool, which notation, and where the diagrams
live. ADR [0018](0018-documentation-tooling.md) settled the code-*doc*
tooling but is silent on architecture diagrams, so the choice needs its own
record.

## Decision Drivers

* Diagrams must render where contributors actually read docs — GitHub's
  Markdown viewer — with no build step.
* Consistency with the docs-as-Markdown tree (ADR 0018) — a text format
  that diffs and reviews in a PR beats a binary.
* Minimal tooling: no new runtime dep, no image-export pipeline.
* Forward-compatible with the deferred MyST-Sphinx HTML publish
  (`0018-documentation-tooling.md:55-57`).

## Considered Options

* **Inline Mermaid, C4 drawn with `flowchart`/`sequenceDiagram`/`erDiagram`** —
  stable Mermaid diagram kinds, C4 level stated in prose + subgraph labels.
* **Inline Mermaid using the native `C4Context`/`C4Container`/`C4Component`
  types** — true-to-spec C4 keywords, but experimental in Mermaid.
* **External diagrams (`.drawio` / exported PNG/SVG)** — richest layout, but
  binary, doesn't diff, needs an editor + export step.
* **Separate `.mmd` files under `docs/architecture/diagrams/`** — reusable,
  but splits prose from picture for a single-page overview.

## Decision Outcome

Chosen: **inline Mermaid in `docs/architecture/overview.md`, with C4 levels
drawn using the stable `flowchart` kind** (and `sequenceDiagram` / `erDiagram`
for the flows and data model). Mermaid's native `C4*` types are still marked
experimental and lay out unreliably on GitHub and MyST, so we get the C4
*structure* (context → container → component) from flowchart subgraphs and
name the level in prose. Text-based, reviews in the PR, renders natively on
GitHub, and needs no tooling today. Diagrams stay inline so each sits next to
the prose that explains it.

## Consequences

* Positive: the overview renders on GitHub with zero build; every diagram
  diffs as text and is reviewable in the PR.
* Positive: no new dependency lands in this slice — GitHub renders Mermaid
  natively.
* Neutral / forward dependency: the MyST-Sphinx HTML publish is **deferred to
  slice 9** (`0018-documentation-tooling.md:55-57`,
  `../conventions/documentation.md:72-76`). When the Python Sphinx config is
  set up (`docs/conf.py`, slice-7 scope per `0018-documentation-tooling.md:66-67`)
  it will need a Mermaid extension (`sphinxcontrib-mermaid` or the MyST
  mermaid fence) for that publish. This slice adds no config.
* Negative: flowchart-styled C4 loses the native `Person`/`System`/`Container`
  keyword semantics; the C4 level is conveyed by prose and subgraph labels.
* Negative: the overview restates ADR/`schema.sql` content and can drift —
  there is no CI drift-check for diagrams (unlike `packages/contracts/`). A
  "source of truth" note at the top of `overview.md` points readers to the
  canonical ADRs and `schema.sql`; a standing CI guard would be its own slice.
* Supersession path: if Mermaid's native C4 support stabilizes or the diagrams
  outgrow inline Mermaid, this ADR can be superseded and the diagrams migrated
  — the same reversible-commitment posture as
  `0018-documentation-tooling.md:68-73`.
