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
- [x] Step 12 — Update `.tasks/epics/arch-scaffold/parent.md` Auth
      row (plan.md #12)
      *[tested-against-real-input]* — `rg -c 'password'
      .tasks/epics/arch-scaffold/parent.md` returns `1`.
- [x] Step 13 — Verify (plan.md #13)
      *[tested-against-real-input]* — every acceptance criterion
      in `spec.md` passes; evidence in `## Summary` below.

## Deviations

### 1. Non-structural — parent fork-table edit called out on the PR

**Cite:** `spec.md` §Deviations from parent fork table.

**What changed:** The Auth row of
`.tasks/epics/arch-scaffold/parent.md` extended to "Google OAuth
+ password login + Postgres-backed server-side sessions". Flagged
in the spec's Deviations section and this Implementation summary.
`rg -c 'password' .tasks/epics/arch-scaffold/parent.md` returns
`1`.

## Summary

**Acceptance-criteria status (all pass):**

| Criterion | Evidence |
|---|---|
| Eight `docs/adr/*` files exist | `ls docs/adr/` shows all eight |
| Six ADRs carry all five MADR sub-heads | `rg` check returns no missing |
| Six ADRs carry `Status: Accepted` | same |
| Three convention docs ≥ 300 bytes | api.md 1485B, coding.md 1541B, events.md 1508B |
| `events.md` names all three canonical streams | `rg -c` returns `3` |
| No files outside `docs/` and `.tasks/` | `git diff --name-only` clean |
| Fork ADRs forward-reference downstream slice(s) | 0003/0004/0005/0006 all cite slices in Consequences |
| ADR 0005 mentions password | 5 hits |
| ADR 0005 mentions `NoopAuthProvider` | 2 hits |
| ADR 0006 has ≥ 2 `### Decision Outcome` sub-sections | 2 |
| Parent fork table Auth row updated | 1 password hit |

**Trust tiers:** all `docs/**` entries are `untested-assumption`
— markdown has no runtime and downstream slices (2, 3, 5, 6, 8)
will exercise the decisions. The parent-md edit is
`tested-against-real-input` (grep-verified).

**PR-review changes vs. the initial spec:**

* ADR 0002 gained Vertical Slice Architecture + lightweight CQRS
  wording (OQ 1 resolution, from PR-#11 discussion).
* ADR 0005 extended from Google-OAuth-only to dual-identity
  (Google OAuth + password login) per owner review.
* ADR 0006 split its `## Decision Outcome` into two sub-sections
  (transport + naming) so the fold satisfies the single-ADR
  numbering rule without cramming both decisions into one
  paragraph.
* Absolute local paths were scrubbed from `spec.md` after the
  privacy-leak thread; only repo-relative refs and GitHub issue
  links remain.
