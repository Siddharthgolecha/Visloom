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

The approved epic plan (Epic; `.tasks/epics/arch-scaffold/parent.md`)
scopes this slice to: MADR template, ADRs 0001–0006, and
`docs/conventions/{coding,events,api}.md`. Most decisions are
**locked** on the parent issue's fork table
(`.tasks/epics/arch-scaffold/parent.md:21-35`) — the ADRs
memorialize choices the human has already approved. Two revisions
land through this PR's review: ADR 0002 adopts Vertical Slice
Architecture + lightweight CQRS as the "where hex is appropriate"
rubric, and ADR 0005 extends Auth to include password-based login
alongside Google OAuth (see Deviations from parent fork table below).

[epic]: https://github.com/Siddharthgolecha/Visloom/issues/1
[issue]: https://github.com/Siddharthgolecha/Visloom/issues/2

## Open Questions for the Human

Written before re-reading the ticket, per `AGENTS.md:47-51`. All
four resolved during PR #11 plan review — kept in-spec as append-only
audit trail per `AGENTS.md:34-36`.

1. **ADR 0002 framing.** *Resolved:* Layered + hexagonal-where-
   appropriate, with **Vertical Slice Architecture** as the
   feature-encapsulation unit and a lightweight **CQRS** rubric for
   deciding "where hex is appropriate": read/query paths default to
   simple layered routes; write/command paths that touch external
   effects (worker pipeline, auth handshake, media processing)
   go through hex ports + adapters. (Suggestion from PR #11
   [discussion_r3587993126](https://github.com/Siddharthgolecha/Visloom/pull/11#discussion_r3587993126).)
2. **ADR 0006 scope.** *Resolved:* Fold. ADR 0006 contains two
   `### Decision Outcome` sub-sections — one for transport (Redis
   Streams), one for naming (`.v<int>` on stream name) — inside a
   single file. Keeps the four-digit monotonic numbering rule
   without cramming both decisions into a single Decision Outcome
   paragraph. (Suggestion from PR #11
   [discussion_r3588070543](https://github.com/Siddharthgolecha/Visloom/pull/11#discussion_r3588070543).)
3. **`docs/conventions/api.md` versioning posture.** *Resolved:*
   URL-path versioning at `/api/v1/…` behind Caddy same-origin.
   Rust walking-skeleton slice (6) will cite this. (Suggestion from
   PR #11 [discussion_r3588149966](https://github.com/Siddharthgolecha/Visloom/pull/11#discussion_r3588149966).)
4. **ADR 0005 auth scope + dev fallback.** *Resolved:* Extend to
   **Google OAuth + password-based login** (owner review, PR #11,
   commit `64aa754`) — a change from the parent fork table
   ("Google OAuth + Postgres-backed server-side sessions").
   `NoopAuthProvider` is mentioned in `## Consequences` as a
   required local-dev fallback; the technical implementation of
   how Noop wires in is deferred to a future ADR alongside the API
   scaffold (slice 6). (Suggestion from PR #11
   [discussion_r3588207337](https://github.com/Siddharthgolecha/Visloom/pull/11#discussion_r3588207337).)

## Research findings

Every claim anchored to a `file:line` per `AGENTS.md:32-38`.

- `AGENTS.md:118-133` names the required `spec.md` sections (Context,
  Open Questions, Research findings, Approach, Tradeoffs, Failure
  modes, Acceptance criteria). This file follows them.
- `AGENTS.md:32-38` requires anchored claims — every ADR in this
  slice cross-references either the parent issue's fork table or
  the reviewed PR-#11 threads that adjusted specific forks.
- `CONTRIBUTING.md:126-133` names the reviewer's Spec-lane
  checklist: anchored research, at least one alternative, automated
  acceptance criteria, concrete failure modes. Each is present below.
- `.tasks/epics/arch-scaffold/parent.md:21-35` fixes the eleven
  locked forks; this slice records four of them as ADRs
  (monorepo / compose / auth / streams+naming). Auth extended to
  include password-based login per PR-#11 owner review.
- The Epic issue [#1](https://github.com/Siddharthgolecha/Visloom/issues/1)
  and this task's issue [#2](https://github.com/Siddharthgolecha/Visloom/issues/2)
  are the canonical remote references for the approved plan.
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
| 0002 | Layered + hex-where-appropriate; VSA + lightweight CQRS | framework |
| 0003 | Polyglot monorepo, no meta-tool | fork |
| 0004 | Docker Compose on a single VPS (dev + prod overlays) | fork |
| 0005 | Auth: Google OAuth + password login, Postgres sessions | fork |
| 0006 | Redis Streams for indexing + versioned stream naming | fork |

ADR 0006 folds transport + naming into one file with two
`### Decision Outcome` sub-sections (per PR-#11 review, OQ 2).
Splitting to 0006/0007 would either cascade slice-2 numbering or
leave `docs/conventions/events.md` citing an ADR that doesn't
exist yet.

The remaining twelve ADRs (0007–0018) land in slice 2 per the
Epic checklist (`.tasks/epics/arch-scaffold/parent.md:37-53`).

No files touched outside `docs/`. No overlap-list edits (slice 3
owns `AGENTS.md` §6 per the Epic plan). No CI, no code.

### Deviations from parent fork table

Two forks are refined here from their wording on
`.tasks/epics/arch-scaffold/parent.md:21-35`, both approved on
PR #11:

- **Auth (row 3 of parent fork table).** Extends from "Google OAuth
  + Postgres-backed server-side sessions" to include password-based
  login as a first-class second method. Same session backend
  (Postgres-backed server-side); the change adds a second identity
  provider (local credential store) beside Google OAuth.
- **Architecture shape.** Adds Vertical Slice Architecture as the
  feature-encapsulation unit and a lightweight read-vs-write CQRS
  rubric for choosing where hex applies. This is a refinement of
  "hexagonal-where-appropriate" — not a new fork.

Slice-2 ADRs and the parent-issue fork table body will be
reconciled in this slice's PR (parent.md sits under `.tasks/`, not
`docs/`, so touching it doesn't inflate the LOC budget).

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
- **ADR 0002 (hex-where-appropriate + VSA + CQRS) is prescriptive
  without a running example.** The Rust and Python skeletons will
  validate the shape in slices 6/7; if a runtime doesn't fit, ADR
  0002 gets a `Superseded-by` header per the status vocabulary in
  the index.
- **ADR 0005 dual-auth widens surface area.** Password login adds a
  credential-storage decision (hashing algorithm, rotation, rate-
  limiting, account recovery) that Google OAuth avoided. This ADR
  records the *choice* to support both; the tech details land in a
  future ADR alongside the API scaffold (slice 6).

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
- **ADR 0002's VSA + CQRS rule proves too prescriptive.** The
  "reads = layered, writes = hex" rubric is a rule of thumb, not a
  law. If a real feature crosses the line (e.g., a query that
  triggers a side-effectful cache warm), the ADR needs a
  `Superseded-by` before slices 6/7 write conflicting code.
  *Mitigation:* the rubric is stated as a **default**, not an
  invariant, in ADR 0002's Decision Outcome.
- **Parent fork-table update is silent.** Editing
  `.tasks/epics/arch-scaffold/parent.md` in this PR without
  flagging it in the PR body could hide the auth-scope change.
  *Mitigation:* the Deviations section above records the drift
  explicitly and the PR body will link to it at ready-time via
  `implementation.md` `## Summary`.
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
- [ ] ADR 0005 mentions **both** identity paths (Google OAuth
      *and* password login). *Falsified if:*
      `rg -c 'password' docs/adr/0005-*.md` returns `0`.
- [ ] ADR 0005 `## Consequences` names `NoopAuthProvider` as the
      local-dev fallback. *Falsified if:*
      `rg -c 'NoopAuthProvider' docs/adr/0005-*.md` returns `0`.
- [ ] ADR 0006 has two `### Decision Outcome` sub-sections
      (transport + naming). *Falsified if:*
      `rg -c '^### ' docs/adr/0006-*.md` returns fewer than 2.
- [ ] Parent fork table (`.tasks/epics/arch-scaffold/parent.md`)
      is updated so the Auth row matches the ADR 0005 wording.
      *Falsified if:* `rg -c 'password' .tasks/epics/arch-scaffold/parent.md`
      returns `0`.
