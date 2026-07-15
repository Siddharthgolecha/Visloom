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

Written before re-reading the ticket, per `AGENTS.md:47-51`.
≤5 high-leverage forks the plan cannot resolve from context alone.

1. **ADR 0018 documentation tooling: commit to `myst-parser` vs.
   raw RST for Python docs today, or defer to
   `docs/conventions/documentation.md`?** MyST lets Python docstrings
   live in Markdown alongside the rest of the docs tree; raw RST is
   the Sphinx default and integrates more cleanly with
   `sphinx-autodoc-typehints`. This is a real fork, not a style
   preference — worth an ADR line if we commit now.
2. **ADR 0016 data storage: does Postgres schema evolution (naming
   convention, up/down migrations, backwards-compat window) need to
   be locked here, or deferred until slice 5's compose PR has a
   concrete `init.sql` to react to?** Locking here means slice 5 can
   cite the ADR; deferring means the ADR describes only Redis
   key-naming and leaves Postgres to a later slice.
3. **`docs/privacy.md` deletion-SLA number: commit to a specific
   number (e.g. "attendee-uploaded selfies deleted within 24h of
   search completion", "faces purged within 30d of event
   archival"), or record the posture without numbers and defer
   quantitative commitments to a later slice with product input?**
   Numbers turn `privacy.md` from posture-doc into a promise
   downstream code must honor; posture-only keeps it aspirational.
4. **ADR 0015 observability: fold logs + traces + metrics into three
   `### Decision Outcome` sub-sections (per ADR 0006's pattern) or
   split into three separate ADRs (0015 traces, 0016 logs, 0017
   metrics)?** Folding conserves ADR numbers for the twelve-slot
   budget; splitting matches part-1's precedent that each ADR
   records "one decision." A fold is my default recommendation
   because the three outcomes share the OTel decision.
5. **ADR 0017 versioning: does it need to reconcile with slice 3's
   contracts package (which will define the schema-versioning wire
   format), or is a docs-only "we use SemVer for OpenAPI spec,
   `.v<int>` for event streams, URL-path for HTTP" statement
   sufficient?** Reconciling means part-2 blocks on slice 3;
   docs-only means slice 3 must cite this ADR and stay compatible.

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
  `### Decision Outcome` sub-sections.
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
- `AGENTS.md:53-55` and `CONTRIBUTING.md:30-31` — the "sensitive
  user content never leaks" rules that `docs/privacy.md` should
  centralize.
- `docs/adr/template.md` and `docs/adr/README.md:32-39` — the
  authoring skeleton + rule that every fork ADR names ≥1 downstream
  slice in `Consequences`.
- `.tasks/arch-conventions-and-adrs-part-1-20260715-121423Z/spec.md`
  and `plan.md` — the precedent for spec/plan shape; this slice
  mirrors them.

## Approach

Docs-only PR of eighteen new markdown files under `docs/`, no
edits to existing files. Twelve new ADRs (0007–0018), five new
convention docs, one new `docs/privacy.md`. MADR-full for all ADRs,
using `docs/adr/template.md` as the skeleton.

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
| 0015 | Observability: OTel-first, logs/traces/metrics | thematic | coding.md:15,34-36 | 5, 6, 7, 8 |
| 0016 | Data storage: Postgres schema + Redis usage | thematic | 0006, 0005 | 5, 6, 7 |
| 0017 | Versioning: URL-path + event streams + contracts | thematic | api.md:44-46, 0006 | 3, 6 |
| 0018 | Documentation tooling: Sphinx / rustdoc / TypeDoc | thematic | (new) | 6, 7, 8 |

ADR 0008 uses "owner" (not "photographer") to match the RBAC vocabulary
ADR 0005 landed in part-1. Each fork ADR names ≥1 downstream slice in
`## Consequences`, per `docs/adr/README.md:36-39`. ADR 0006 is the
precedent for two-decision folds; 0015 is the likely candidate (see OQ
4).

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
- `docs/conventions/data.md` — Postgres migration naming,
  transaction boundaries, Redis key naming (streams stay in
  `events.md`). Cites ADR 0016.
- `docs/conventions/testing.md` — test tiers (unit / integration /
  contract) + adjacency rules. Extracts and expands
  `docs/conventions/coding.md:44-48`.
- `docs/conventions/documentation.md` — code-doc style per language:
  Python (Sphinx + Google-style docstrings, MyST vs. RST per OQ 1);
  Rust (rustdoc); TypeScript (TSDoc + TypeDoc). Cites ADR 0018.

### `docs/privacy.md`

Single privacy-posture doc. Centralizes the "sensitive user content
never leaks" rules from `AGENTS.md:53-55` and `CONTRIBUTING.md:30-31`
into a doc downstream slices can cite. Covers: what is PII in this
app (faces, selfie uploads, share-token URLs, IP logs), storage
constraints (encryption at rest, no PII in logs/spans), telemetry
scrubbing rules, log retention posture, deletion-on-request posture.
No new technical decisions — records existing ones. Quantitative
commitments follow OQ 3.

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
- **`docs/privacy.md` will need re-review when product provides
  concrete SLA numbers** (OQ 3). Accepted: posture-first is better
  than blocking on product for the docs-scaffold slice.

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
  ADR 0005 for role definitions.
- **ADR 0014 (password crypto) crossing into implementation.**
  Deciding hash algorithm (argon2id vs. bcrypt) is decision-shaped;
  deciding parameters (memory cost, parallelism) is
  implementation-shaped and belongs in code. Mitigation: 0014
  records algorithm choice + rate-limit policy shape; parameter
  tuning is called out as slice-6 code-time work.
- **`docs/privacy.md` promising something the code can't deliver.**
  E.g. "no PII in traces" is easy to violate accidentally in a
  handler that logs a request body. Mitigation: posture-first (OQ 3)
  plus a `## Failure modes` section on the doc itself listing
  known-leaky spots (log lines that include full request paths carry
  share tokens; span attributes named `user_id` in early prototypes).
- **ADR 0018 tooling choice colliding with a future decision.**
  MyST-Markdown ecosystem is younger than RST; if OQ 1 picks MyST
  and it doesn't work, we need to supersede. Mitigation: ADR 0018's
  Consequences explicitly names the supersession path; MyST is
  reversible with a `myst_parser` extension removal.
- **Overshooting the "docs only" boundary.** A tempting sub-move is
  to check in a `.readthedocs.yaml` or a Sphinx `conf.py` skeleton
  alongside ADR 0018. Rejected: those are code-shaped and belong in
  slices 6/7. `docs/` stays docs-only.

## Acceptance criteria

Every criterion automated-or-observable with a falsifier.

- [ ] All twelve ADRs (0007–0018) exist under `docs/adr/` and follow
  the MADR-full template's five sections (Context and Problem
  Statement, Decision Drivers, Considered Options, Decision Outcome,
  Consequences). *Falsified if:* any file is missing a section
  header from `docs/adr/template.md:1-45`.
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
- [ ] Every locked fork in `.tasks/epics/arch-scaffold/parent.md:23-35`
  is covered by an ADR (part-1 or part-2). *Falsified if:* the
  fork→ADR map in `spec.md` §Approach has any row with an empty
  "Cites" column.
- [ ] `docs/privacy.md` exists and is cited by at least one ADR
  landing in this PR. *Falsified if:* `grep -R 'docs/privacy.md\|
  privacy posture' docs/adr/*.md` returns zero hits.
- [ ] No file outside `docs/` changes in this PR (other than
  `.tasks/<id>/*` scaffolding). *Falsified if:* `git diff --stat
  main -- ':!docs' ':!.tasks'` returns any files.
