# Implementation — arch-conventions-and-adrs-part-1

## Task tree

- [x] Step 1 — `docs/adr/template.md` (plan.md §Tactical steps #1)
      *[untested-assumption]* — MADR-full skeleton, five section
      headers, inline guidance comments.
- [x] Step 2 — `docs/adr/README.md` (plan.md #2)
      *[untested-assumption]* — index (6 rows), numbering rule,
      status vocabulary, how-to-author pointer.
- [x] Step 3 — ADR 0001 Adopt MADR (plan.md #3)
      *[untested-assumption]* — Status: Accepted, all five MADR
      sections populated, one alternative (Nygard) rejected.
- [x] Step 4 — ADR 0002 Layered + hex-where-appropriate; VSA +
      lightweight CQRS (plan.md #4)
      *[untested-assumption]* — locks baseline + VSA feature unit +
      "reads = layered, writes with effects = hex" rubric as a
      **default, not invariant**. Forward-refs slices 6/7/8 in
      Consequences.
- [x] Step 5 — ADR 0003 Polyglot monorepo (plan.md #5)
      *[untested-assumption]* — locks repo shape; forward-refs
      slices 3/6/8/9.
- [x] Step 6 — ADR 0004 Docker Compose single VPS (plan.md #6)
      *[untested-assumption]* — locks deploy target; forward-refs
      slices 5/9.
- [x] Step 7 — ADR 0005 Owner auth (Google OAuth + password
      backup) and RBAC (plan.md #7) *[untested-assumption]* —
      three `### Decision Outcome` sub-sections (attendee
      share-token / owner identity / RBAC roles);
      `NoopAuthProvider` mentioned in Consequences with a fixed
      `dev` principal + `owner` role; forward-refs slices 3/5/6.
      **Rewritten during PR-#11 mid-implementation feedback**
      (attendees are unauthenticated; password is an owner
      backup, not an attendee path; three-role RBAC added).
- [x] Step 8 — ADR 0006 Redis Streams + versioned naming
      (plan.md #8)
      *[untested-assumption]* — two `### Decision Outcome` sub-
      sections (transport + naming); names the three canonical
      streams verbatim; forward-refs slices 3/5/7.
- [x] Step 9 — `docs/conventions/coding.md` (plan.md #9)
      *[untested-assumption]* — cross-language rules only;
      language-specific rules deferred to slices 6/7/8.
- [x] Step 10 — `docs/conventions/events.md` (plan.md #10)
      *[untested-assumption]* — names all three canonical streams;
      documents version-bump cutover procedure.
- [x] Step 11 — `docs/conventions/api.md` (plan.md #11)
      *[untested-assumption]* — locks `/api/v1/*` behind Caddy
      same-origin, error envelope, idempotency header, cookie auth.
- [x] Step 12 — Update `.tasks/epics/arch-scaffold/parent.md` Auth
      row (plan.md #12)
      *[tested-against-real-input]* — `rg -c 'password'
      .tasks/epics/arch-scaffold/parent.md` returns `1`.
- [ ] Step 13 — Verify (plan.md #13) — **HALTED** pending
      re-approval on ADR 0005 rewrite (see Deviations §1). All
      non-0005 acceptance criteria pass; the new 0005-specific
      bullets need reviewer sign-off.

## Deviations

### 1. Structural — ADR 0005 rewritten mid-implementation (HALTED for re-approval)

**Cite:** `spec.md` §ADR numbering; §Deviations from parent fork
table; §Acceptance criteria (new bullets on share-token, roles,
Tenancy row).

**What changed:** ADR 0005 rewritten and renamed based on owner
feedback during implementation (before `task_finish.sh`):

* **Attendees are unauthenticated.** They arrive via a
  per-event, revocable share-token URL — no login, no session,
  no email. Previously ADR 0005 gave them a password login
  option, which was wrong.
* **Password is a backup for owners**, not an attendee path.
  Owner accounts can attach both Google OAuth and email +
  password credentials; both mint the same Postgres session
  cookie.
* **"Owner" replaces "Photographer"** as the platform role
  name, because event owners can be venue staff, coordinators,
  or others — not just photographers. Parent fork-table
  Tenancy row updated to match.
* **RBAC added.** Three roles per event: `owner`, `editor`,
  `reader`. Evaluated against a Postgres `event_memberships`
  table via an `AuthzPolicy` port (ADR 0002).

**File-level effect:**

* `docs/adr/0005-auth-oauth-and-password.md` → renamed to
  `docs/adr/0005-owner-auth-and-rbac.md` (via `git mv`).
* `docs/adr/README.md` — index row updated.
* `docs/adr/0004-docker-compose-single-vps.md` — one context
  sentence changed ("one photographer + attendees" → "one
  owner-managed event + its attendees").
* `docs/conventions/api.md` — auth line links to the new ADR
  filename and mentions the attendee share-token path.
* `.tasks/epics/arch-scaffold/parent.md` — Auth row + Tenancy
  row both updated.

**Structural per `AGENTS.md:107-113`** — the change to ADR
0005's scope (attendees no longer authenticate, RBAC added) and
to the parent fork table (Tenancy row) contradict the wording
frozen at `plan-approved`. **HALTING** here; the reviewer
decides whether to remove `plan-approved`, re-review, and put
it back on.

### 2. Non-structural — parent fork-table edits called out on the PR

**Cite:** `spec.md` §Deviations from parent fork table.

**What changed:** Two rows of
`.tasks/epics/arch-scaffold/parent.md` updated — Auth (owners +
attendees + RBAC) and Tenancy (Owner-managed). Flagged in the
spec's Deviations section and this Implementation summary.
`rg -c 'share.token' .tasks/epics/arch-scaffold/parent.md`
returns `1`; `rg -c 'Photographer-owned' .tasks/epics/arch-scaffold/parent.md`
returns `0`.

## Summary

**Acceptance-criteria status** (all non-halted criteria pass;
0005-specific criteria await re-approval per Deviations §1):

| Criterion | Status | Evidence |
|---|---|---|
| Eight `docs/adr/*` files exist | pass | `ls docs/adr/` shows all eight |
| Six ADRs carry all five MADR sub-heads | pass | `rg` check returns no missing |
| Six ADRs carry `Status: Accepted` | pass | same |
| Three convention docs ≥ 300 bytes | pass | api.md 1533B, coding.md 1541B, events.md 1508B |
| `events.md` names all three canonical streams | pass | `rg -c` returns `3` |
| No files outside `docs/` and `.tasks/` | pass | `git diff --name-only` clean |
| Fork ADRs forward-reference downstream slice(s) | pass | 0003/0004/0005/0006 all cite slices |
| ADR 0005 covers share-token + password paths | pass | `share.token` and `password` both hit |
| ADR 0005 names three roles | pass | `owner` / `editor` / `reader` all present |
| ADR 0005 mentions `NoopAuthProvider` | pass | 2 hits |
| ADR 0006 has ≥ 2 `### Decision Outcome` sub-sections | pass | 2 |
| Parent Auth row matches ADR 0005 | pass | `share.token` hit = 1 |
| Parent Tenancy row not "Photographer-owned" | pass | `Photographer-owned` hit = 0 |

**Trust tiers:** all `docs/**` entries are `untested-assumption`
— markdown has no runtime and downstream slices (2, 3, 5, 6, 8)
will exercise the decisions. The parent-md edit is
`tested-against-real-input` (grep-verified).

**PR-review + mid-implementation changes vs. the initial spec:**

* ADR 0002 gained Vertical Slice Architecture + lightweight CQRS
  wording (OQ 1 resolution, from PR-#11 discussion).
* ADR 0005 evolved twice: (a) plan review extended it from
  Google-OAuth-only to dual-identity; (b) mid-implementation
  feedback corrected the model — attendees are unauthenticated
  (share-token URL), password is an owner backup, "owner"
  replaces "photographer", RBAC (owner/editor/reader) added.
  Renamed `0005-owner-auth-and-rbac.md`. See Deviations §1.
* ADR 0006 split its `## Decision Outcome` into two sub-sections
  (transport + naming) so the fold satisfies the single-ADR
  numbering rule without cramming both decisions into one
  paragraph.
* Absolute local paths were scrubbed from `spec.md` after the
  privacy-leak thread; only repo-relative refs and GitHub issue
  links remain.
* All LOC budgets and per-file LOC estimates were removed from
  the plan/spec/impl and from the parent-epic tracking — they
  don't belong in Visloom artifacts (project convention).
