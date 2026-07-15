# Spec — arch-conventions-and-adrs-part-2

## Context

Slice 2 of the arch-scaffold epic ([#1][epic]), tracked as
[#3][issue] with lane `lane:considered`. Docs only.

Part-1 ([PR #11][pr11], merged in `fb4b3f3`) landed the MADR
framework + ADRs 0001–0006 + `docs/conventions/{coding,events,api}.md`.
Part-1's docs make forward promises this slice must fulfill:

- `docs/adr/README.md:20` — "`0007` is next."
- `docs/adr/0001-adopt-madr.md:39` — "Slice 2 (ADRs 0007–0018)
  exercises this template at volume."
- `docs/adr/0005-owner-auth-and-rbac.md:119-120` — Noop wire-up
  deferred to "a future ADR with slice 6."
- `docs/adr/0005-owner-auth-and-rbac.md:106-108` — password crypto,
  rate-limiting, account recovery deferred to "slice 6's follow-up
  ADRs."
- `docs/adr/README.md:36-39` — every fork ADR must name ≥1
  downstream slice in `Consequences`.

Six of the eleven locked forks on
`.tasks/epics/arch-scaffold/parent.md:23-35` are still uncovered
(Media scope, Tenancy, Search transport, Inference runtime, Generated
contracts, Python deps). Downstream slices cite them by number:
slice 3 (contracts) cites the generated-contracts ADR; slice 5
(compose) cites deploy target; slice 6 (Rust API) cites search
transport + inference runtime + Noop + password crypto; slice 7
(Python worker) cites Python-deps + inference runtime + media scope;
slice 8 (Next.js web) cites tenancy.

The remaining 0013–0018 slots land the two deferred auth ADRs plus
four thematic ADRs (observability, data storage, versioning policy,
documentation tooling) that unblock slices 6/7/8 authoring. Five new
convention docs and `docs/privacy.md` complete the docs surface.

Intended outcome: part-2 merges docs-only, downstream slices cite
ADRs by number without gaps, and the epic's docs surface is done.

[epic]: https://github.com/Siddharthgolecha/Visloom/issues/1
[issue]: https://github.com/Siddharthgolecha/Visloom/issues/3
[pr11]: https://github.com/Siddharthgolecha/Visloom/pull/11

## Open Questions for the Human

All five OQs are resolved in-spec per the review on PR #12 so
`plan.md` has no conditional branches. Kept in-spec as append-only
audit trail per `AGENTS.md:34-36`. Reviewer may still override any
answer during plan review.

1. **ADR 0018 Python doc tooling.** *Resolved:* **MyST-Parser +
   Sphinx + Google-style docstrings.** Docstrings live in Markdown,
   matching the rest of the docs tree; reversible with a
   `myst_parser` extension removal if the ecosystem doesn't hold up
   (called out in the ADR's Consequences).
2. **ADR 0016 scope.** *Resolved:* **Redis-only. Postgres schema
   evolution defers to slice 5.** ADR 0016 has one `## Decision
   Outcome` covering Redis key naming; a future ADR alongside slice
   5's compose PR locks Postgres migration format, up/down policy,
   and backwards-compat window when there is a concrete `init.sql`
   to react to. `docs/conventions/data.md` reflects this split.
3. **`docs/privacy.md` scope.** *Resolved:* **Posture-only
   consolidation.** The doc restates the rules already committed in
   `AGENTS.md:52-55` and `CONTRIBUTING.md:30-31` (no sensitive user
   content in commits, PR bodies, telemetry). It introduces **no**
   new commitments — no encryption-at-rest promise, no retention
   window, no deletion-SLA number, no IP-log policy. Items that
   need product input land under `## Deferred (needs product
   input)` inside `docs/privacy.md` itself and are also enumerated
   in this slice's `implementation.md` as follow-up work.
4. **ADR 0015 shape.** *Resolved:* **Fold.** One ADR, three `###
   Decision Outcome` sub-sections (logs, traces, metrics), matching
   the ADR 0006 precedent. Conserves ADR numbers.
5. **ADR 0017 scope.** *Resolved:* **Docs-only three-axis
   statement.** HTTP APIs at `/api/v1/` (per `docs/conventions/
   api.md`), event streams `.v<int>` (per ADR 0006), generated
   contracts SemVer'd on the contracts package. Slice 3 (contracts)
   must cite ADR 0017 and stay compatible when it lands the
   concrete wire format. No cross-slice block on part-2.

## Research findings

Every claim anchored to a `file:line` per `AGENTS.md:32-38`.

- `.tasks/epics/arch-scaffold/parent.md:23-35` — fork table. Six
  rows (Media scope, Tenancy, Search transport, Inference runtime,
  Generated contracts, Python deps) still lack an ADR after part-1.
- `docs/adr/README.md:11-16` — index shows 0001–0006 in place;
  `0007` is the next slot per `README.md:20`.
- `docs/adr/0005-owner-auth-and-rbac.md:119-120` — commits Noop
  wire-up + guard checks to "a future ADR with slice 6."
- `docs/adr/0005-owner-auth-and-rbac.md:106-108` — commits password
  crypto, rate-limiting, account recovery to "slice 6's follow-up
  ADRs" (plural).
- `docs/adr/0006-redis-streams-versioned-naming.md` — the pattern
  precedent for folding two decisions into one ADR file via two
  `### Decision Outcome` sub-sections. ADR 0015 in this slice uses
  the same pattern for three sub-sections (logs, traces, metrics).
- `docs/conventions/coding.md:15` — mentions `telemetry/` layer but
  the observability doc doesn't exist yet.
- `docs/conventions/coding.md:34-36` — inline logging/tracing
  paragraph awaiting extraction into a dedicated doc.
- `docs/conventions/coding.md:28-31` and `docs/conventions/api.md:25-33`
  — domain-error and wire-error guidance split across two files, no
  unified `errors.md`.
- `docs/conventions/coding.md:44-48` — testing paragraph awaiting
  extraction into `testing.md`.
- `docs/conventions/api.md:44-46` — "a future ADR may add m2m
  bearer support" — informational, not a hard promise, but relevant
  to ADR 0017 versioning.
- `AGENTS.md:52-55` and `CONTRIBUTING.md:30-31` — the "sensitive
  user content never leaks" rules that `docs/privacy.md` restates.
  These are the **only** commitments `docs/privacy.md` promises;
  anything beyond them is under `## Deferred (needs product input)`.
- `docs/adr/template.md` and `docs/adr/README.md:32-39` — the
  authoring skeleton + rule that every fork ADR names ≥1 downstream
  slice in `Consequences`.
- `.tasks/arch-conventions-and-adrs-part-1-20260715-121423Z/spec.md`
  and `plan.md` — the precedent for spec/plan shape; this slice
  mirrors them.

## Approach

Docs-only PR of eighteen new markdown files under `docs/`. One
existing file changes — `docs/adr/README.md` gets twelve new
index rows. No other existing-file edits. Twelve new ADRs
(0007–0018), five new convention docs, one new `docs/privacy.md`.
MADR-full for all ADRs, using `docs/adr/template.md` as the
skeleton.

### ADR numbering (locked)

Six forks first (in fork-table order), then two deferred auth ADRs,
then four thematic:

| # | Title | Kind | Cites | Cited by slice |
|---|---|---|---|---|
| 0007 | Media scope: photo + video-keyframe day-1 | fork | parent.md:28 | 7 (worker) |
| 0008 | Tenancy: owner-owned events + share tokens | fork | parent.md:29 | 6, 8 |
| 0009 | Search transport: API embeds selfie inline via CPU ONNX | fork | parent.md:31 | 6 (API) |
| 0010 | Inference runtime: Worker CUDA+CPU / API CPU-only | fork | parent.md:32 | 6, 7 |
| 0011 | Generated contracts: committed + CI drift-check | fork | parent.md:34 | 3 (contracts) |
| 0012 | Python deps: `uv` + `uv.lock` | fork | parent.md:35 | 7 (worker) |
| 0013 | NoopAuthProvider wire-up + guard checks | thematic | 0005:119-120 | 6 (API) |
| 0014 | Password crypto + rate limit + recovery | thematic | 0005:106-108 | 6 (API) |
| 0015 | Observability: OTel-first (logs / traces / metrics folded) | thematic | coding.md:15,34-36 | 5, 6, 7, 8 |
| 0016 | Redis usage (Postgres deferred to slice 5) | thematic | 0006, 0005 | 5, 6, 7 |
| 0017 | Versioning: URL-path + event streams + contracts | thematic | api.md:44-46, 0006 | 3, 6 |
| 0018 | Documentation tooling: MyST-Sphinx / rustdoc / TypeDoc | thematic | (new) | 6, 7, 8 |

ADR 0008 uses "owner" (not "photographer") to match the RBAC vocabulary
ADR 0005 landed in part-1. Each fork ADR names ≥1 downstream slice in
`## Consequences`, per `docs/adr/README.md:36-39`. ADR 0006 is the
precedent for multi-decision folds; ADR 0015 uses three `### Decision
Outcome` sub-sections for logs / traces / metrics.

### Convention docs (new)

Each new file either extracts a paragraph from part-1 conventions or
introduces a topic that has no home yet. Each cites its anchoring
ADR by number.

- `docs/conventions/observability.md` — cross-runtime logging /
  tracing / metrics shape. Cites ADR 0015. Extracts and expands the
  paragraph at `docs/conventions/coding.md:34-36`.
- `docs/conventions/errors.md` — unified error taxonomy. Unifies
  domain errors (`docs/conventions/coding.md:28-31`) and wire error
  envelope (`docs/conventions/api.md:25-33`) into one doc.
- `docs/conventions/data.md` — Redis key naming (streams stay in
  `events.md`). Postgres migration/transaction conventions are
  deferred to slice 5's follow-up (per OQ 2). Cites ADR 0016.
- `docs/conventions/testing.md` — test tiers (unit / integration /
  contract) + adjacency rules. Extracts and expands
  `docs/conventions/coding.md:44-48`.
- `docs/conventions/documentation.md` — code-doc style per language:
  Python (Sphinx + MyST-Parser + Google-style docstrings — per OQ 1);
  Rust (rustdoc); TypeScript (TSDoc + TypeDoc). Cites ADR 0018.

### `docs/privacy.md`

Posture-only consolidation (per OQ 3). Restates the rules already
committed in `AGENTS.md:52-55` and `CONTRIBUTING.md:30-31`. Sections:

- **What is sensitive** — faces, selfie uploads, share-token URLs.
  Enumerated from part-1 ADR 0005's model.
- **Where sensitive content must not appear** — commits, PR bodies,
  issue comments, telemetry (logs, traces, metrics), the anchoring
  quote is `AGENTS.md:52-55`.
- **Known leaky spots** — pre-existing hazards downstream slices
  must design around: request paths carrying share tokens, span
  attributes named `user_id`, log lines that include full URLs.
- **Deferred (needs product input)** — items intentionally not
  committed here: encryption-at-rest policy, log-retention window,
  deletion-on-request SLA, IP-log posture, cross-border data
  transfer. Each item names the future ADR or product decision
  that will resolve it. Enumerated verbatim in this slice's
  `implementation.md` as follow-up work so nothing gets lost.

No new technical or product decisions land here. If a later slice
adds encryption-at-rest, it lands as its own ADR + `docs/privacy.md`
edit.

### Alternative considered

**Ship only the six fork ADRs (0007–0012) and defer 0013–0018 to
later slices.** Rejected: the epic checklist commits to 0007–0018 in
part-2, and downstream slices 6/7/8 need the auth follow-ups plus
observability + versioning ADRs to cite in their `plan.md`. Slipping
them here means each downstream slice ships its own tiny ADR PR
before its code — more branch overhead than one docs-only slice.

**Also considered: MkDocs as the doc-tool for all three runtimes**
(recorded in ADR 0018's Considered Options). Rejected because MkDocs
doesn't ingest type-annotated docstrings the way Sphinx (`autodoc` +
`sphinx-autodoc-typehints`) does for Python, and both rustdoc and
TypeDoc are native to their toolchains already. Per-language tooling
wins.

## Tradeoffs accepted

- **Twelve ADRs in one PR is a lot to review.** Each ADR is short and
  self-contained (per MADR-full), and part-1 already normalized the
  reviewer eye on this exact shape. The alternative — twelve
  standalone PRs — burns more reviewer overhead than one focused PR.
- **Some ADRs describe decisions without an implementation trace
  yet** (e.g. ADR 0013 Noop wire-up records a plan before slice 6
  writes the code). Trust tier for these is
  `untested-assumption` — flagged in `implementation.md`, and slice 6
  will revalidate at code time.
- **Doc-tool tri-fecta (Sphinx + rustdoc + TypeDoc) is more surface
  than a single doc-tool.** Accepted: each is native to its language,
  and forcing MkDocs across all three would compromise Python
  autodoc.
- **`docs/privacy.md` is deliberately incomplete.** It records the
  posture we already have and enumerates what we don't yet have
  (encryption-at-rest, retention, deletion-SLA). Product will fill
  the deferred list in a later slice with its own ADR + doc edit.
  Preferred over blocking part-2 on product input.

## Failure modes

Adversarial re-read.

- **Numbering collision with a downstream slice that opens a
  parallel ADR PR.** Mitigation: `docs/adr/README.md:20-22` locks
  allocation at PR-open time; part-2's numbers are already taken by
  virtue of this branch existing on GitHub. But another agent
  opening `docs/adr/0007-*.md` on a parallel branch would race.
  Reviewer must check `gh pr list --search 'is:draft docs/adr'`
  before requesting `plan-approved`.
- **ADR 0008 vocabulary drift with ADR 0005.** ADR 0005 uses
  `owner`/`editor`/`reader`; the parent fork table (line 29) still
  says "photographer-owned events." If ADR 0008 uses "photographer"
  the vocabulary splits. Mitigation: 0008 uses "owner" and cites
  ADR 0005 for role definitions. Part-1 already updated the parent
  fork table to "owner-managed" wording — 0008 must match.
- **ADR 0014 (password crypto) crossing into implementation.**
  Deciding hash algorithm (argon2id vs. bcrypt) is decision-shaped;
  deciding parameters (memory cost, parallelism) is
  implementation-shaped and belongs in code. Mitigation: 0014
  records algorithm choice + rate-limit policy shape; parameter
  tuning is called out as slice-6 code-time work.
- **`docs/privacy.md` accidentally introduces a new commitment.**
  Easy to slip a sentence like "faces are encrypted at rest" into
  the "What is sensitive" section because it sounds obvious.
  Mitigation: `## Deferred (needs product input)` explicitly lists
  the items we are *not* committing to; the reviewer greps for the
  header before approving. Acceptance criterion checks the header
  exists and has ≥3 items.
- **ADR 0018 tooling choice colliding with a future decision.**
  MyST-Markdown ecosystem is younger than RST; if it doesn't work,
  we supersede. Mitigation: ADR 0018's Consequences explicitly
  names the supersession path; MyST is reversible with a
  `myst_parser` extension removal.
- **Overshooting the "docs only" boundary.** A tempting sub-move is
  to check in a `.readthedocs.yaml` or a Sphinx `conf.py` skeleton
  alongside ADR 0018. Rejected: those are code-shaped and belong in
  slices 6/7. `docs/` stays docs-only.

## Acceptance criteria

Every criterion automated-or-observable with a falsifier.

- [ ] All twelve ADRs (0007–0018) exist under `docs/adr/` and follow
  the MADR-full template's five sections (Context and Problem
  Statement, Decision Drivers, Considered Options, Decision Outcome,
  Consequences). *Falsified if:* any of the twelve files is missing
  a section header from `docs/adr/template.md`.
- [ ] `docs/adr/README.md` index has been updated to list all
  twelve new ADRs with their titles and `Accepted` status.
  *Falsified if:* `grep -c '| \[00' docs/adr/README.md` returns less
  than 18 after this PR.
- [ ] Every fork ADR (0007–0012) names at least one downstream slice
  in `## Consequences`. *Falsified if:* `grep -L 'slice [0-9]'
  docs/adr/000[7-9]-*.md docs/adr/001[0-2]-*.md` returns any file.
- [ ] Every convention doc (`observability.md`, `errors.md`,
  `data.md`, `testing.md`, `documentation.md`) cites its anchoring
  ADR by number. *Falsified if:* `grep -L 'ADR 00' docs/conventions/
  observability.md docs/conventions/errors.md docs/conventions/
  data.md docs/conventions/testing.md docs/conventions/
  documentation.md` returns any file.
- [ ] Every internal ADR cross-reference resolves. *Falsified if:*
  `grep -R 'ADR 00' docs/` surfaces a number without a corresponding
  file in `docs/adr/`.
- [ ] Every locked fork listed in the fork-table on
  `.tasks/epics/arch-scaffold/parent.md:23-35` is covered by an ADR
  in `docs/adr/`. *Falsified if:* for any of the six fork keywords
  {media scope, tenancy, search transport, inference runtime,
  generated contracts, python deps}, `grep -l -i "<keyword>"
  docs/adr/00*.md` returns no file (case-insensitive).
- [ ] `docs/privacy.md` exists and contains a `## Deferred (needs
  product input)` section listing at least three items. *Falsified
  if:* `grep -c '^## Deferred (needs product input)$'
  docs/privacy.md` returns `0`, or the section contains fewer than
  three bulleted items.
- [ ] `docs/privacy.md` does not introduce new commitments beyond
  what `AGENTS.md:52-55` and `CONTRIBUTING.md:30-31` already commit.
  *Falsified if:* the doc contains any of the strings "encryption
  at rest", "retention window", "deletion within", or a specific
  time SLA (e.g. `24h`, `30d`) outside the `## Deferred` section.
  Manual review at plan-approval time.
- [ ] `docs/privacy.md` is cited by at least one ADR landing in
  this PR. *Falsified if:* `grep -R 'docs/privacy.md\|privacy
  posture' docs/adr/*.md` returns zero hits.
- [ ] Only `docs/adr/README.md` is edited among existing files.
  *Falsified if:* `git diff --name-only main...HEAD | rg -v
  '^docs/|^\.tasks/'` returns any path, **or** any existing file
  other than `docs/adr/README.md` shows up under `git diff
  --name-only --diff-filter=M main...HEAD -- 'docs/**'`.
