# Plan — arch-diagrams-and-overview

<!-- How this task gets done, tactically. Frozen at `plan-approved`. -->

## Tactical steps

<!-- Ordered. Each step cites the spec.md section it implements. -->

1. Create `docs/architecture/overview.md` with the top "source of truth"
   disclaimer (ADRs + `schema.sql` canonical; diagrams a synthesized view) and
   a one-paragraph system summary. Implements spec `## Approach`, spec
   `## Failure modes` (drift mitigation).
2. Author the 4 C4 flowchart diagrams inline — Context (1), Container (2),
   Component API (3), Component Worker (4) — each with prose + ADR cross-refs
   per the locked table in spec `## Approach`. Uses stable `flowchart`/`graph`
   syntax (spec OQ 2), API+Worker for the two Component diagrams (spec OQ 3).
3. Author the 2 sequence diagrams — indexing (5) with the three canonical
   stream names + `traceparent` propagation, and search (6) inline CPU-ONNX →
   pgvector. Implements spec `## Research findings` (events.md:26-30,47-61).
4. Author the ER diagram (7) mirroring exactly the seven `schema.sql` tables +
   FKs + `role`/`media_kind` enums, with prose noting the slice-5 deferral.
   Implements spec `## Approach` + `## Failure modes` (ER overreach).
5. Write ADR 0019 from `docs/adr/template.md` (five MADR sections,
   `Status: Accepted`) recording the inline-Mermaid / flowchart-C4 choice,
   with the MyST-mermaid forward-dependency + supersession clause in
   `## Consequences`. Implements spec `## Approach` (ADR 0019) + OQ 4.
6. Add the ADR 0019 index row to `docs/adr/README.md` (title verbatim) and
   advance the "next number" pointer to `0020`. Implements spec `## Acceptance
   criteria` (index row).
7. Add a one-line architecture link to `README.md` pointing at
   `docs/architecture/overview.md`. Implements spec OQ 5.
8. Verify: `grep` checks from spec `## Acceptance criteria` (fence count,
   diagram kinds, stream/table fidelity, ADR-link resolution, docs-only diff);
   render all 7 fences on the PR page. Log results + trust tiers in
   `implementation.md`.

## Files touched

<!-- Paths (glob if widespread). One line each. -->

- `docs/architecture/overview.md` — **new.** Intro + 7 inline mermaid diagrams.
- `docs/adr/0019-architecture-diagrams-mermaid.md` — **new.** MADR-full ADR
  recording the diagram-format choice.
- `docs/adr/README.md` — add the ADR 0019 index row; advance next-number pointer.
- `README.md` — add a one-line link to the architecture overview.

## Depends on

<!-- Other open issues/PRs that must land first. Informational — reviewer
     enforces by holding plan-approval. `None` if nothing. -->

None. Slices 1–3 (PRs #11, #12, #14) already merged provide all source
material. Docs-only; does not block on infra (slice 5) or the walking
skeletons.
