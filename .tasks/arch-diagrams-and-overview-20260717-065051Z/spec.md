# Spec — arch-diagrams-and-overview

<!-- The contract for this task: what it should do, why, and how we'll know.
     Frozen at `plan-approved`. After that, deviations go in implementation.md. -->

## Context

Slice 4 of the arch-scaffold epic ([#1][epic]), tracked as [#5][issue]
with lane `lane:considered`. Docs only.

Slices 1–3 landed the source material this slice synthesizes:

- Part-1 ([PR #11][pr11], `fb4b3f3`) — MADR framework + ADRs 0001–0006 +
  `docs/conventions/{coding,events,api}.md`.
- Part-2 ([PR #12][pr12], `ce46558`) — ADRs 0007–0018, five more
  convention docs, `docs/privacy.md`.
- Contracts ([PR #14][pr14], `c9a8c41`) — `packages/contracts/` event
  JSON Schemas, OpenAPI skeleton, `schema.sql`, generator, CI drift-check.

The architecture today exists **only** as 18 ADRs (`docs/adr/0001`–`0018`)
plus 8 convention docs. There is no single page a contributor can read to
grasp the whole system, and the repo contains **zero diagrams** of any kind
(no mermaid, drawio, or image files). Root `README.md` is a two-line
tagline with no link to any docs.

This slice ships `docs/architecture/overview.md` — one page synthesizing
the ADRs — with 7 inline Mermaid diagrams (C4 context / container /
component ×2, sequence ×2, ER data model), each cross-referencing the ADRs
it visualizes. The diagram-format choice is recorded as ADR 0019 (parallel
to ADR 0018's doc-tooling precedent), and `README.md` gains an architecture
link so the page is discoverable.

Intended outcome: a new contributor reads one page + 7 diagrams and
understands the three-runtime system, its two data-flows, and its data
model, with every claim traceable to an ADR.

[epic]: https://github.com/Siddharthgolecha/Visloom/issues/1
[issue]: https://github.com/Siddharthgolecha/Visloom/issues/5
[pr11]: https://github.com/Siddharthgolecha/Visloom/pull/11
[pr12]: https://github.com/Siddharthgolecha/Visloom/pull/12
[pr14]: https://github.com/Siddharthgolecha/Visloom/pull/14

## Open Questions for the Human

All resolved in-spec (via plan-mode review before scaffold). Kept as an
append-only audit trail per `AGENTS.md:34-36`. Reviewer may override any
answer during plan review.

1. **Diagram layout — inline vs separate files?** *Resolved:* **all 7 inline**
   as mermaid fenced blocks in a single `docs/architecture/overview.md`, each
   under its own `##` section with prose + ADR cross-refs. One file, renders
   natively on GitHub, prose context stays with each diagram.
2. **"C4" rendering — Mermaid's native `C4*` types vs stable flowchart?**
   *Resolved:* **flowchart-styled C4.** Mermaid's `C4Context`/`C4Container`/
   `C4Component` types are experimental and render unreliably on GitHub/MyST;
   `flowchart`/`graph` with subgraphs is stable everywhere. Prose labels each
   diagram with its C4 level. Loses strict C4 keyword semantics (accepted).
3. **The "×2" in the count — which two get C4 Component diagrams?**
   *Resolved:* **API (Rust) + Worker (Python)** — the count resolves to
   Context(1)+Container(1)+Component×2(2)+Sequence×2(2)+ER(1)=7, and the two
   backend runtimes are the ones with real internal structure per ADR 0002.
   Web (`apps/web/`) is a thin skeleton (parent.md:49) with little to
   decompose.
4. **Record the format choice as an ADR?** *Resolved:* **yes — ADR 0019**
   ("Architecture diagrams: inline Mermaid, flowchart-styled C4"), for
   consistency with ADR 0018 recording the doc-*tooling* choice. Adds an
   index row to `docs/adr/README.md`.
5. **Discoverability of the new page?** *Resolved:* **link from `README.md`.**
   A one-line architecture pointer; a doc nothing links to is dead on arrival.
   `README.md` is not in the `AGENTS.md:§6` overlap list.

## Research findings

Every claim anchored to a `file:line` per `AGENTS.md:32-38`.

**System shape (for the C4 + sequence diagrams):**

- `.tasks/epics/arch-scaffold/parent.md:23-35` — the locked fork table:
  polyglot monorepo, single-VPS Docker Compose, owner/attendee auth split,
  photo+video-keyframe media, Redis-Streams indexing, inline-CPU-ONNX search,
  Worker CUDA+CPU / API CPU-only.
- `docs/adr/0002-layered-hexagonal-architecture.md` — per-runtime baseline
  `domain / application / adapters / telemetry`; VSA + lightweight CQRS; hex
  ports (`AuthProvider`, `AuthzPolicy`, `SelfieEmbedder`). Source for the two
  Component diagrams.
- `docs/adr/0003-polyglot-monorepo.md` — three runtimes: `services/api/`
  (Rust), `services/worker/` (Python), `apps/web/` (Next.js), plus
  `packages/contracts/` and `infra/`.
- `docs/adr/0004-docker-compose-single-vps.md` — deploy topology: Postgres +
  Redis + Caddy + OTel collector on one VPS. Source for the Container diagram.
- `docs/adr/0005-owner-auth-and-rbac.md` — Owner: Google OAuth + password
  backup, Postgres sessions. Source for the Context diagram's actors.
- `docs/adr/0008-tenancy-owner-events-and-share-tokens.md` — Attendee:
  unauthenticated, revocable per-event share token. Event is the tenant
  boundary; RBAC `owner`/`editor`/`reader`.
- `docs/adr/0009-search-transport-cpu-onnx-inline.md` — synchronous intra-API
  search, target < 2 s, `SelfieEmbedder` CPU-ONNX inline → pgvector NN.
  Source for the search sequence diagram.
- `docs/adr/0010-inference-runtime-worker-cuda-api-cpu.md` — Worker CUDA +
  CPU-fallback embedder; API CPU-only. Source for the Worker Component diagram.
- `docs/conventions/events.md:26-30` — the three canonical streams verbatim:
  `jobs.media.index.v1` (API→worker), `events.media.indexed.v1` (success),
  `events.media.index_failed.v1` (failure).
- `docs/conventions/events.md:47-61` — the envelope payload shape
  (`event_id`, `traceparent`, `tracestate`, `trace_id`, `occurred_at`,
  `data`). Source for the indexing sequence diagram + trace propagation.
- `docs/adr/0015-observability-otel-first.md` — `traceparent` propagated
  through the event envelope so traces span API→worker.

**Data model (for the ER diagram):**

- `packages/contracts/schema.sql:21-74` — the seven tables: `accounts`,
  `sessions`, `events`, `event_memberships`, `share_tokens`, `media`,
  `idempotency_keys`, with FKs and the `role`/`media_kind` CHECK enums
  (`schema.sql:45,61`). The ER diagram mirrors these exactly.
- `packages/contracts/schema.sql:1-19` — header: this is a *reference*, not
  an executed migration; embeddings/pgvector/frames tables are deferred to
  slice 5. The ER diagram must not invent tables beyond the seven present.

**Authoring precedent + placement:**

- `docs/adr/README.md` — the ADR index; `0019` is the next free number.
- `docs/adr/template.md` — the MADR-full skeleton ADR 0019 follows.
- `docs/adr/0018-documentation-tooling.md` — precedent: a thematic
  doc-tooling ADR whose `## Consequences` names downstream slices. ADR 0019
  mirrors its shape and its supersession-path clause.
- `docs/conventions/events.md:3` and `docs/conventions/documentation.md:4` —
  the relative ADR cross-ref link style (`../adr/000N-*.md`) `overview.md`
  matches.
- `.tasks/arch-conventions-and-adrs-part-2-20260715-181956Z/spec.md` — the
  spec/plan shape this slice mirrors.

## Approach

Docs-only PR. **One new file** synthesizes the architecture:
`docs/architecture/overview.md`, a top intro paragraph + 7 `##` sections,
each opening with 1–3 sentences of prose, the mermaid fence, and the ADR
cross-refs. Plus **one new ADR** (0019) recording the diagram-format choice,
its **index row** in `docs/adr/README.md`, and a **one-line link** added to
`README.md`.

All diagrams use stable `flowchart`/`graph`/`sequenceDiagram`/`erDiagram`
mermaid — not the experimental `C4*` types (OQ 2). A "source of truth"
disclaimer at the top of `overview.md` states the ADRs + `schema.sql` are
canonical and the diagrams are a synthesized view (drift mitigation — see
Failure modes).

### The 7 diagrams (locked)

| # | Diagram | Mermaid kind | Cites |
|---|---|---|---|
| 1 | C4 Context — Owner / Attendee / Google OAuth ↔ Visloom | `flowchart` | 0005, 0008 |
| 2 | C4 Container — 3 runtimes + Postgres/Redis/Caddy/OTel on one VPS | `flowchart` | 0002, 0003, 0004 |
| 3 | C4 Component — API (Rust): domain/application/adapters/telemetry + ports | `flowchart` | 0002, 0009, 0013 |
| 4 | C4 Component — Worker (Python): consumer loop, keyframe, CUDA/CPU embedder | `flowchart` | 0002, 0007, 0010 |
| 5 | Sequence — indexing (async) via Redis Streams + trace propagation | `sequenceDiagram` | 0006, 0007, 0015 |
| 6 | Sequence — search (sync, < 2 s) inline CPU-ONNX → pgvector | `sequenceDiagram` | 0009 |
| 7 | ER — the 7 `schema.sql` tables + FKs + enums | `erDiagram` | 0005, 0007, 0008 |

### ADR 0019

MADR-full (five sections per `docs/adr/template.md`), `Status: Accepted`.
Records: architecture diagrams live inline in `docs/architecture/overview.md`
as Mermaid; C4 levels are drawn with stable flowchart syntax rather than the
experimental `C4*` types. `## Consequences` names the forward-dependency:
the MyST-Sphinx publish target (slice 7 `conf.py`) will need
`sphinxcontrib-mermaid`, and carries a supersession-path clause mirroring
`docs/adr/0018-documentation-tooling.md:68-73`. Index row added to
`docs/adr/README.md` (title verbatim; `0019` → next number becomes `0020`).

### Alternative considered

**Mermaid's native `C4Context`/`C4Container`/`C4Component` types.** True-to-spec
C4 semantics, but experimental in Mermaid — layout is rough and rendering lags
on GitHub/MyST. Rejected: an unreliable render defeats the point of a
contributor-facing overview. Flowchart-styled C4 renders everywhere and the
C4 level is stated in prose.

**Separate diagram files under `docs/architecture/diagrams/`.** Reusable, but
splits prose from picture and adds files a one-page overview doesn't need.
Rejected for the single-file layout (OQ 1).

**Keep the format choice implicit (spec.md only, no ADR 0019).** Lighter, but
inconsistent with ADR 0018 recording the doc-*tooling* decision. Rejected —
one small ADR keeps the diagram convention discoverable and durable (OQ 4).

## Tradeoffs accepted

- **Flowchart-styled C4 is not strict C4.** We lose the `Person`/`System`/
  `Container` keyword semantics; the C4 level lives in prose + subgraph
  labels. Accepted for reliable rendering across GitHub and the future MyST
  publish target.
- **The overview duplicates ADR/schema content and can drift.** There is no
  CI drift-check for diagrams (unlike `packages/contracts/`). Accepted with a
  "source of truth" disclaimer + a Failure-modes entry naming the ceiling; a
  CI guard would be its own slice.
- **The ER diagram is intentionally incomplete.** It shows only the seven
  tables in `schema.sql` today; embeddings/pgvector/frames arrive with slice
  5. Accepted and stated in the diagram's prose.
- **Mermaid rendering depends on the reader's surface.** GitHub renders it
  natively; the MyST-Sphinx HTML publish (deferred to slice 9) needs an
  extension. Accepted and recorded in ADR 0019 `## Consequences`.

## Failure modes

Adversarial re-read.

- **Diagram ↔ source drift.** The ER table set or a stream name in the
  indexing sequence silently diverges from `schema.sql` / `events.md` after a
  future edit, and nothing catches it. Mitigation: the "source of truth"
  disclaimer points readers to the canonical files; the fidelity acceptance
  criterion greps stream/table names against source at review time. A
  standing CI guard is out of scope (its own slice).
- **Mermaid fence fails to parse and renders as a code block.** A single
  syntax error kills a diagram. Mitigation: render check — every fence viewed
  on the PR page (or run through `mmdc`) before requesting ready; acceptance
  criterion requires all 7 to render.
- **ER diagram overreach.** Tempting to add an `embeddings` or `frames` table
  because the indexing flow mentions embeddings. Rejected: `schema.sql` has
  no such tables (`schema.sql:1-19`). The diagram shows exactly seven; prose
  notes the deferral.
- **Overshooting "docs only."** Tempting to add `sphinxcontrib-mermaid` to a
  `conf.py` or a `.readthedocs.yaml`. Rejected — that is slice-7/9 code-shaped
  work. ADR 0019 records the dependency in prose; no config lands here.
- **ADR 0019 number collision** with a parallel branch allocating `0019`.
  Mitigation: run `gh pr list --search 'is:draft'` before requesting
  `plan-approved` and skim for an ADR PR (per `AGENTS.md:§6`); this branch
  claims `0019` by existing.
- **Vocabulary drift.** The diagrams must use the ADR vocabulary
  (`owner`/`editor`/`reader`, `owner`/`attendee`, `photo`/`video`) — not
  invented synonyms like "photographer" or "guest". Mitigation: labels quote
  ADR/schema terms; fidelity criterion greps for the canonical role/kind enums.

## Acceptance criteria

Every criterion automated-or-observable with a falsifier.

- [ ] `docs/architecture/overview.md` exists and contains exactly 7 mermaid
  fences. *Falsified if:* `grep -c '^```mermaid' docs/architecture/overview.md`
  ≠ 7.
- [ ] The 7 fences cover the required kinds: ≥1 `sequenceDiagram`, ≥1
  `erDiagram`, and the remaining flowchart/graph diagrams for the C4 levels.
  *Falsified if:* `grep -c 'sequenceDiagram' overview.md` < 2, or
  `grep -c 'erDiagram' overview.md` < 1.
- [ ] Every diagram section cites at least one ADR by relative link.
  *Falsified if:* any `##` diagram section has no `../adr/00` link between it
  and the next `##` heading.
- [ ] Every ADR link in `overview.md` resolves to a real file. *Falsified if:*
  any `../adr/00NN-*.md` target extracted from `overview.md` is absent from
  `docs/adr/`.
- [ ] The ER diagram names exactly the seven `schema.sql` tables and no
  others. *Falsified if:* the set of entities in the `erDiagram` differs from
  {`accounts`, `sessions`, `events`, `event_memberships`, `share_tokens`,
  `media`, `idempotency_keys`} (`packages/contracts/schema.sql:21-74`).
- [ ] The indexing sequence uses the canonical stream names verbatim.
  *Falsified if:* `overview.md` does not contain all of `jobs.media.index.v1`,
  `events.media.indexed.v1`, `events.media.index_failed.v1`
  (`docs/conventions/events.md:26-30`).
- [ ] All 7 mermaid fences render without a parse error (viewed on the PR
  page, or via `mmdc`). *Falsified if:* any fence displays as raw text / a
  mermaid error box.
- [ ] ADR 0019 exists, follows the five MADR sections of `docs/adr/template.md`,
  and is `Status: Accepted`. *Falsified if:* `docs/adr/0019-*.md` is missing
  any template section header, or its status is not `Accepted`.
- [ ] `docs/adr/README.md` lists ADR 0019 with its title verbatim. *Falsified
  if:* `grep -c '0019' docs/adr/README.md` = 0, or the row title differs from
  the ADR's H1.
- [ ] `README.md` links to `docs/architecture/overview.md`. *Falsified if:*
  `grep -c 'docs/architecture/overview.md' README.md` = 0.
- [ ] Only docs are touched. *Falsified if:* `git diff --name-only main...HEAD`
  (excluding `.tasks/`) returns any path outside `docs/` and `README.md`.
