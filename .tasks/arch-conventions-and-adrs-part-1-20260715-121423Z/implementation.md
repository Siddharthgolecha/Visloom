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
- [x] Step 7 — ADR 0005 Auth: OAuth + password + Postgres sessions
      (plan.md #7)
      *[untested-assumption]* — dual-identity; `NoopAuthProvider`
      mentioned in Consequences, tech details deferred to slice-6
      ADR; forward-refs slices 5/6.
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
- [x] Step 12 — Update `.tasks/epics/arch-scaffold/parent.md:27`
      Auth row (plan.md #12)
      *[tested-against-real-input]* — `rg -c 'password'
      .tasks/epics/arch-scaffold/parent.md` returns `1`.
- [ ] Step 13 — Verify (plan.md #13) — **HALTED**, see Deviations
      below.

## Deviations

### 1. Structural — LOC budget breach (HALTED for re-approval)

**Cite:** `spec.md` §Acceptance criteria bullet "Total diff LOC ≤
350 under `docs/`."

**What changed:** Diff LOC under `docs/` is **518**, not ≤ 350.

**Per-file breakdown** (after the mitigation the plan named —
tightening Considered Options prose across all six ADRs and
compressing the convention docs):

```
39  docs/adr/0001-adopt-madr.md
53  docs/adr/0002-layered-hexagonal-architecture.md
42  docs/adr/0003-polyglot-monorepo.md
41  docs/adr/0004-docker-compose-single-vps.md
52  docs/adr/0005-auth-oauth-and-password.md
62  docs/adr/0006-redis-streams-versioned-naming.md
39  docs/adr/README.md
47  docs/adr/template.md
46  docs/conventions/api.md
48  docs/conventions/coding.md
49  docs/conventions/events.md
--
518 total
```

For reference, the pre-tightening total was 632 — the mitigation
saved 114 LOC. Further compression would drop load-bearing
content (MADR sub-sections, cutover procedures, error envelope).

**Why:** the plan predicted ~395 LOC and named tightening as the
mitigation; in execution, six MADR-full ADRs + template + index +
three conventions each carrying non-trivial load-bearing content
(status vocabulary, numbering rules, VSA/CQRS rubric,
`.v<int>` cutover procedure, error envelope, idempotency) do not
compress below ~500 LOC without becoming unreadable. The 350
ceiling was set before the OQ-driven revisions (VSA/CQRS in 0002,
dual auth in 0005, sub-sections in 0006) added material that
wasn't in the pre-review estimate.

**Structural per `AGENTS.md:107-113`** — an acceptance criterion
is contradicted. HALTING here rather than pushing past a merged
spec bullet. The reviewer decides:

**Option A** (recommended): remove `plan-approved`, revise the
LOC bullet in `spec.md` from `≤ 350` to `≤ 550`, add back
`plan-approved`. The content is otherwise on-spec; every other
acceptance criterion passes (verified below).

**Option B**: cut scope — drop one convention doc or defer 1–2
ADRs to slice 2. This changes the slice's scope commitment to
the epic parent issue and needs its own conversation.

**Option C**: accept the 518 LOC total under the existing budget
by remeasuring — e.g., count only ADR files (302 LOC), or only
new-line adds excluding blank lines (~400). Would require
rewording the bullet, so still needs re-approval.

### 2. Non-structural — parent fork-table edit called out on the PR

**Cite:** `spec.md` §Deviations from parent fork table.

**What changed:** `.tasks/epics/arch-scaffold/parent.md:27` Auth
row extended to "Google OAuth + password login + Postgres-backed
server-side sessions". Flagged in the spec's Deviations section
and this Implementation summary. `rg -c 'password'
.tasks/epics/arch-scaffold/parent.md` returns `1`.

### 3. Non-structural — LOC checks used `wc -l`, not `git diff --shortstat`

**Cite:** `spec.md` §Verification step 4 named `git diff
--shortstat main...HEAD -- 'docs/**'`.

**What changed:** Verified LOC via `wc -l docs/**` because the
docs work isn't committed yet (the branch was scaffolded before
any docs work landed). Same measurement basis (whole files, all
of which are new). Will re-run `git diff --shortstat` post-commit
before running `task_finish.sh`.

## Summary

**Acceptance-criteria status (10 of 11 pass):**

| Criterion | Status | Evidence |
|---|---|---|
| Eight `docs/adr/*` files exist | pass | `ls docs/adr/` shows all eight |
| Six ADRs carry all five MADR sub-heads | pass | `rg` check returns no missing |
| Six ADRs carry `Status: Accepted` | pass | same |
| Three convention docs ≥ 300 bytes | pass | api.md 1485B, coding.md 1541B, events.md 1508B |
| `events.md` names all three canonical streams | pass | `rg -c` returns `3` |
| **Total diff LOC ≤ 350 under `docs/`** | **FAIL** | 518 LOC; halted for re-approval |
| No files outside `docs/` and `.tasks/` | pass | `git diff --name-only` clean |
| Fork ADRs forward-reference downstream slice(s) | pass | 0003/0004/0005/0006 all cite slices in Consequences |
| ADR 0005 mentions password | pass | 5 hits |
| ADR 0005 mentions `NoopAuthProvider` | pass | 2 hits |
| ADR 0006 has ≥ 2 `### Decision Outcome` sub-sections | pass | 2 |
| Parent fork table Auth row updated | pass | 1 password hit |

**Trust tiers:** all `docs/**` entries are `untested-assumption`
— markdown has no runtime and downstream slices (2, 3, 5, 6, 8)
will exercise the decisions. The parent-md edit is
`tested-against-real-input` (grep-verified).

**PR-review changes vs. the pre-review estimate that set the LOC
budget:**

* ADR 0002 gained VSA + lightweight CQRS wording (~15 LOC over
  a "pure hex-where-appropriate" ADR).
* ADR 0005 gained dual-identity + widened-surface consequences
  (~15 LOC over Google-OAuth-only).
* ADR 0006 gained two `### Decision Outcome` sub-sections
  instead of one (~10 LOC).
* Conventions gained the `.v<int>` cutover procedure and the
  error envelope structure at the reviewer's implicit request.

Total review-driven growth: ~40 LOC. Even without it, the six-
MADR-full-ADR baseline lands at ~470 LOC.
