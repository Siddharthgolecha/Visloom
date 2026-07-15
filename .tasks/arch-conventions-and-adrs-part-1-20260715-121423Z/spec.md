# Spec — arch-conventions-and-adrs-part-1

## Context

Slice 1 of the arch-scaffold epic ([#1][epic]), tracked as [#2][issue]
with lane `lane:considered`. Docs only, ≤ 350 LOC of markdown.

The repo today holds only the autodev workflow (`AGENTS.md`,
`CONTRIBUTING.md`, `.tasks/_template/`, `tools/autodev/`) — zero
application, docs, or ADR skeleton. Before any downstream slice can
cite a naming rule or an architectural decision (slice 2 doubles the
ADR count; slice 3 lands `packages/contracts/` schemas that mirror
`docs/conventions/events.md`; slices 6–8 stand up three runtimes
against `docs/conventions/coding.md`), the ADR framework and the
first six ADRs must exist as merged, referenceable files.

The approved epic plan at
`/Users/siddharthgolecha/.claude/plans/you-are-the-lead-wobbly-prism.md:110-113`
scopes this slice to: MADR template, ADRs 0001–0006, and
`docs/conventions/{coding,events,api}.md`. All decisions this slice
records are already **locked** on the parent issue (see the fork
table at `.tasks/epics/arch-scaffold/parent.md:21-35`) — the ADRs
memorialize choices the human has already approved; they do not
re-open them.

[epic]: https://github.com/Siddharthgolecha/Visloom/issues/1
[issue]: https://github.com/Siddharthgolecha/Visloom/issues/2

## Open Questions for the Human

Written before re-reading the ticket, per `AGENTS.md:47-51`.

1. **ADR 0002 framing.** The most subjective ADR in the set: does the
   reviewer accept "layered + hexagonal-where-appropriate" as the
   locked shape, or want strict hexagonal everywhere (or pure
   layered)? Slices 6–8 will materialize whichever wording lands.
2. **ADR 0006 scope.** Fold transport (Redis Streams) + naming
   (`.v<int>` on stream name) into one ADR, or split them across
   0006/0007? Folding is proposed because `events.md` in this slice
   defines the naming rule — splitting forces the doc to cite a
   slice-2 ADR that doesn't exist yet. If the reviewer prefers a
   split, slice-2's ADR numbering cascades and slice-2's approved
   plan needs to update.
4. **`docs/conventions/api.md` versioning posture.** URL-path (`/v1/`)
   vs header-based vs no versioning yet. The epic plan doesn't pin
   this; the Rust walking-skeleton slice will need something to
   cite. Proposed: URL-path `/v1/`.
5. **ADR 0005 dev fallback wording.** The epic plan at
   `you-are-the-lead-wobbly-prism.md:226-230` documents a
   `NoopAuthProvider` for dev when Google credentials are absent.
   Is that fallback in-scope for this ADR's Consequences section,
   or does it belong in a separate ADR when the API scaffold lands?

## Research findings

Every claim anchored to a `file:line` per `AGENTS.md:32-38`.

- `AGENTS.md:118-133` names the required `spec.md` sections (Context,
  Open Questions, Research findings, Approach, Tradeoffs, Failure
  modes, Acceptance criteria). This file follows them.
- `AGENTS.md:32-38` requires anchored claims — every ADR in this
  slice cross-references either the parent issue's fork table or the
  epic plan's Open Question resolutions.
- `CONTRIBUTING.md:126-133` names the reviewer's Spec-lane
  checklist: anchored research, at least one alternative, automated
  acceptance criteria, concrete failure modes. Each is present below.
- `.tasks/epics/arch-scaffold/parent.md:21-35` fixes the eleven
  locked forks; this slice records four of them as ADRs
  (monorepo / compose / auth / streams+naming).
- `/Users/siddharthgolecha/.claude/plans/you-are-the-lead-wobbly-prism.md:110-113`
  is the approved epic plan and scopes this slice to nine files.
- `/Users/siddharthgolecha/.claude/plans/you-are-the-lead-wobbly-prism.md:44-49`
  resolves Open Question 2 of the epic — stream naming is versioned
  on the stream name (`jobs.media.index.v1`).
- `/Users/siddharthgolecha/.claude/plans/you-are-the-lead-wobbly-prism.md:241-243`
  names the epic-level acceptance criterion "every ADR has all five
  MADR sections populated" — this slice adopts MADR-full accordingly.
- `.tasks/_template/spec.md` and `plan.md` are the shapes this
  document mirrors.
- The repository has no prior `docs/` tree — `find docs 2>/dev/null`
  is empty; all files in this slice are new.

## Approach

Ship a docs-only PR of ten new markdown files under `docs/`, no
edits to existing files. Six ADRs (two framework + four fork), one
template, one index, three convention docs. MADR-full (five sections)
across every ADR; convention docs stay short and cite ADRs by number.

### ADR numbering (locked)

| # | Title | Kind |
|---|---|---|
| 0001 | Adopt MADR for architecture decisions | framework |
| 0002 | Layered + hexagonal-where-appropriate architecture | framework |
| 0003 | Polyglot monorepo, no meta-tool | fork |
| 0004 | Docker Compose on a single VPS (dev + prod overlays) | fork |
| 0005 | Google OAuth + Postgres-backed server-side sessions | fork |
| 0006 | Redis Streams for indexing + versioned stream naming | fork |

ADR 0006 folds transport + naming into one decision because
`docs/conventions/events.md` in this slice already documents the
naming scheme. Splitting them would leave `events.md` citing an ADR
that doesn't exist yet — breaking the anchored-claims rule.

The remaining twelve ADRs (0007–0018) land in slice 2 per
`you-are-the-lead-wobbly-prism.md:114-116`.

No files touched outside `docs/`. No overlap-list edits (slice 3
owns `AGENTS.md` §6 per the epic plan). No CI, no code.

### Alternative considered

**Split ADR 0006 into transport (0006) and naming (0007).**
Rejected: `events.md` in this slice defines the naming rule, so
splitting forces one of two bad outcomes — either `events.md`
cites a slice-2 ADR that doesn't exist at merge time (breaks
`AGENTS.md:32-38` anchored claims), or the naming ADR lands here
and 0007 renumbers, cascading into slice 2's approved plan.

## Tradeoffs accepted

- **Six ADRs in one PR** is denser than the "one decision per PR"
  discipline typical of ADR practice, but each ADR here memorializes
  a **pre-locked** decision from the parent issue — the reviewer is
  auditing wording, not re-opening the fork. Slice 2 (0007–0018) is
  the same shape.
- **Convention docs land before the code they govern.** `coding.md`
  will be revised in slices 6–8 when walking skeletons surface
  language-specific rules. Preferred over withholding — downstream
  slices need something to cite.
- **ADR 0002 (hex-where-appropriate) is prescriptive without a
  running example.** The Rust and Python skeletons will validate the
  shape in slices 6/7; if a runtime doesn't fit, ADR 0002 gets a
  `Superseded-by` header per the status vocabulary in the index.

## Failure modes

Adversarial re-read.

- **Slice-1 ADR wording contradicts slice-3 contracts.** If ADR
  0006 says "streams are versioned via `.v<int>` suffix" and slice
  3's `packages/contracts/events/*.json` filenames use `_v1`, slice
  3 reopens 0006. *Mitigation:* 0006 pins the exact scheme
  (`jobs.media.index.v1` — dot-separated, lowercase, `.v<int>`
  suffix) and `events.md` echoes it byte-for-byte.
- **`coding.md` locks a rule that a runtime can't honor.** E.g.,
  "no I/O in domain layer" clashing with TypeScript async
  ergonomics. *Mitigation:* keep `coding.md` minimal here;
  language-specific rules land with the walking-skeleton slices.
- **MADR-full ADRs sprawl past the LOC budget.** Six ADRs × ~40
  LOC + three convention docs × ~35 LOC + template/index ≈ 395 LOC,
  over 350. *Mitigation:* Considered Options sections in the four
  fork ADRs stay to one sentence per alternative — decisions are
  locked, alternatives are audit-trail only.
- **Reviewer disagrees with ADR 0002's framing.** Open Question 1
  surfaces it explicitly so the human resolves it before
  `plan-approved`.
- **Index claims statuses the ADRs don't yet exercise.**
  *Mitigation:* the index lists only Proposed / Accepted /
  Superseded — no exotic statuses. All six ADRs land as `Accepted`.

## Acceptance criteria

- [ ] `docs/adr/README.md`, `docs/adr/template.md`, and
      `docs/adr/000{1..6}-*.md` (six ADR files) all exist.
      *Falsified if:* any of the eight files is missing.
- [ ] Every ADR in `docs/adr/0001..0006*.md` contains all five
      MADR section headers. *Falsified if:*
      `rg -L '^## Context and Problem Statement$|^## Decision Drivers$|^## Considered Options$|^## Decision Outcome$|^## Consequences$' docs/adr/000[1-6]-*.md`
      names any file.
- [ ] Every ADR carries a `Status: Accepted` line in its header.
      *Falsified if:* `rg -L '^\* Status: Accepted$' docs/adr/000[1-6]-*.md`
      names any file.
- [ ] `docs/conventions/{coding,events,api}.md` all exist and are
      each ≥ 300 bytes. *Falsified if:* any file missing or shorter.
- [ ] `docs/conventions/events.md` names all three canonical
      streams (`jobs.media.index.v1`, `events.media.indexed.v1`,
      `events.media.index_failed.v1`) verbatim. *Falsified if:*
      any of the three strings is absent.
- [ ] Total diff LOC ≤ 350 under `docs/`. *Falsified if:*
      `git diff --shortstat main...HEAD -- 'docs/**'` reports
      > 350 insertions.
- [ ] No files touched outside `docs/` and `.tasks/`. *Falsified if:*
      `git diff --name-only main...HEAD | rg -v '^docs/|^\.tasks/'`
      returns any path.
- [ ] Every fork ADR's `## Consequences` names at least one
      downstream slice that consumes the decision (e.g., 0004 →
      slice 5's `infra/compose/`; 0006 → slice 3's
      `packages/contracts/events/`). *Falsified if:* any fork ADR's
      Consequences section makes no forward reference.
