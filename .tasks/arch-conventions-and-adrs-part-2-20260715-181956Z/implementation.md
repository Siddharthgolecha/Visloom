# Implementation — arch-conventions-and-adrs-part-2

## Task tree

- [x] Step 1 — ADR 0007 Media scope: photo + video-keyframe day-1
      (plan.md #1) *[untested-assumption]* — locks the day-1 media
      abstraction; forward-refs slice 7 (worker keyframe extractor)
      and slice 3 (media contract schemas).
- [x] Step 2 — ADR 0008 Tenancy: owner-owned events + share tokens
      (plan.md #2) *[untested-assumption]* — event-as-tenant model
      with `event_memberships` + `share_tokens` tables; forward-refs
      slices 5/6/8.
- [x] Step 3 — ADR 0009 Search transport: API embeds selfie inline
      via CPU ONNX (plan.md #3) *[untested-assumption]* —
      `SelfieEmbedder` port lives in API; no cross-service search
      call; forward-refs slice 6.
- [x] Step 4 — ADR 0010 Inference runtime: Worker CUDA+CPU / API
      CPU-only (plan.md #4) *[untested-assumption]* — split runtime,
      two Dockerfiles; forward-refs slices 6/7.
- [x] Step 5 — ADR 0011 Generated contracts: committed + CI
      drift-check (plan.md #5) *[untested-assumption]* — commit
      generated bindings, CI runs `make contracts && git diff
      --exit-code`; forward-refs slice 3.
- [x] Step 6 — ADR 0012 Python deps: `uv` + `uv.lock` (plan.md #6)
      *[untested-assumption]* — Dockerfile uses `uv sync --frozen
      --no-dev`; forward-refs slice 7.
- [x] Step 7 — ADR 0013 NoopAuthProvider wire-up + guard checks
      (plan.md #7) *[untested-assumption]* — `VISLOOM_ENV=dev` gate
      with startup panic on env mismatch; forward-refs slice 6.
- [x] Step 8 — ADR 0014 Password crypto + rate limit + recovery
      (plan.md #8) *[untested-assumption]* — argon2id + per-account
      + per-IP exponential backoff + magic-link recovery; parameter
      tuning is code-time work; forward-refs slice 6.
- [x] Step 9 — ADR 0015 Observability: OTel-first (folded)
      (plan.md #9) *[untested-assumption]* — three `### Decision
      Outcome` sub-sections (logs / traces / metrics) per ADR 0006
      precedent; forward-refs slices 5/6/7/8.
- [x] Step 10 — ADR 0016 Redis usage: key naming + TTL taxonomy
      (plan.md #10) *[untested-assumption]* — `vloom:<domain>:...`
      naming, three-class TTL taxonomy; Postgres explicitly deferred
      to slice 5; forward-refs slices 5/6/7.
- [x] Step 11 — ADR 0017 Versioning policy (plan.md #11)
      *[untested-assumption]* — three-axis story (`/api/v1/` +
      `.v<int>` + SemVer); slice 3 must cite; forward-refs
      slices 3/6.
- [x] Step 12 — ADR 0018 Documentation tooling (plan.md #12)
      *[untested-assumption]* — MyST-Sphinx for Python (per OQ 1),
      rustdoc, TypeDoc; forward-refs slices 6/7/8.
- [x] Step 13 — Updated `docs/adr/README.md` index (plan.md #13)
      *[tested-against-real-input]* — twelve rows appended;
      "0019 is next" replaces "0007 is next"; grep confirms 18
      total rows.
- [x] Step 14 — `docs/conventions/observability.md` (plan.md #14)
      *[untested-assumption]* — OTel SDKs per language, PII-scrub
      rules; cites ADR 0015.
- [x] Step 15 — `docs/conventions/errors.md` (plan.md #15)
      *[untested-assumption]* — domain vs wire error split, adapter
      mapping responsibility; cites ADR 0002 + `api.md`.
- [x] Step 16 — `docs/conventions/data.md` (plan.md #16)
      *[untested-assumption]* — Redis key naming, three-class TTL
      taxonomy; Postgres explicitly out-of-scope with `## Deferred`
      section pointing at the slice-5 follow-up ADR. Cites ADR
      0016.
- [x] Step 17 — `docs/conventions/testing.md` (plan.md #17)
      *[untested-assumption]* — unit / integration / contract
      tiers; adjacency rule; fixtures-are-plain-data policy.
- [x] Step 18 — `docs/conventions/documentation.md` (plan.md #18)
      *[untested-assumption]* — Python MyST-Sphinx (per OQ 1),
      Rust rustdoc, TS TSDoc + TypeDoc. Cites ADR 0018.
- [x] Step 19 — `docs/privacy.md` (plan.md #19)
      *[untested-assumption]* — posture-only consolidation of
      `AGENTS.md:52-55` + `CONTRIBUTING.md:30-31`; introduces no
      new commitments; `## Deferred (needs product input)` section
      enumerates 6 items intentionally not decided (encryption at
      rest, log retention window, deletion-on-request, deletion
      SLA, IP-log posture, cross-border data transfer). Cited by
      ADR 0015.
- [ ] Step 20 — Verify (plan.md #20) — **HALTED** pending
      re-approval of the widened spec/plan (Deviations §2).
      All 10 acceptance checks pass under the widened criterion
      (evidence in `## Summary`), but the widening itself needs
      the reviewer's sign-off before Step 20 counts as done.

## Deviations

Post-`plan-approved`, non-structural. Each cite is
plan.md/spec.md; each entry uses strikethrough for reversed
claims per the append-only rule.

### 1. Plan step 18 typo — Rust item docs

**Cite:** plan.md §Tactical steps #18 (Rust item-level docstring
delimiter).

Plan.md #18 said Rust item-level docs use `//`. Delivered
`docs/conventions/documentation.md` correctly uses `///`
(`//` is a plain comment in Rust and not picked up by rustdoc).
Reviewer flagged this — recording the plan-vs-delivery gap here
for the audit trail. The delivered value matches ADR 0018's
wording and standard rustdoc idiom.

### 2. Trace propagation clarified in ADR 0015 + events.md (structural — reviewer-requested)

**Cite:** ADR 0015 §Decision Outcome — Traces; events.md
§Payload shape.

Reviewer flagged the initial wording of ADR 0015: "same trace
context propagates onto Redis Stream messages as a payload
field (`trace_id` per `docs/conventions/events.md`)" — a bare
`trace_id` is *correlation*, not *propagation*. Corrected: the
serialized W3C context (`traceparent` + optional `tracestate`)
now rides on every stream payload; `trace_id` remains only as a
derived, log-only convenience. `docs/conventions/events.md`
payload shape updated to match.

**Structural note:** `events.md` was landed in part-1 (merged
in `fb4b3f3`), so editing it here re-scopes the frozen
acceptance criterion "Only `docs/adr/README.md` is edited among
existing files." Spec.md §Acceptance criteria and plan.md
§Files touched were both updated in commit `333d564` to name
`events.md` as the second permitted existing-file edit.

~~Initial handling: kept `plan-approved` and flagged the
widening in-artifact.~~ **Corrected on reviewer instruction:**
per `AGENTS.md` §5 step 7, a post-approval change to an
acceptance criterion is a structural deviation that must HALT
for re-approval — widening the frozen spec/plan while keeping
the label is not allowed, even when the widening is downstream
of a reviewer request. `plan-approved` removed on this pass;
the revised spec.md/plan.md/events.md/ADR 0015 wording is now
under review under this new commit and waits for the label to
return.

### 3. ADR 0016 session cache invalidation on revocation

**Cite:** ADR 0016 §Decision Outcome; ADR 0005 §Owner identity
(per-device session revocation).

Reviewer flagged that ADR 0016 put session hot-path lookups in
Redis with hour-scale TTLs without stating that Redis is a
cache over Postgres. Added two binding rules to ADR 0016:
(a) revocation is a two-step write — delete the Postgres row
**and** delete/invalidate the Redis key; (b) every cache read
carries a `session_version` cross-check against Postgres so a
missed cache invalidation is caught on the next request. Rule
generalized to any other identity/authz key added later.

### 4. ADR 0013 unset-`VISLOOM_ENV` contradiction resolved

**Cite:** ADR 0013 §Decision Outcome.

Reviewer flagged an internal contradiction: bullet 1 selected
`NoopAuthProvider` when `VISLOOM_ENV` was unset **or** `dev`,
while bullet 2 panicked when unset. This is exactly the
prod-safety gate the ADR exists to lock. Resolved with three
ordered rules: real creds → real provider; explicit `dev` →
Noop; anything else, including unset → panic. Unset is now
prod-hostile by default; `VISLOOM_ENV=dev` must be an explicit
opt-in.

## Summary

**Acceptance-criteria status (all 10 pass):**

| # | Criterion | Evidence |
|---|---|---|
| 1 | Twelve ADRs (0007–0018) exist with five MADR sub-heads | `rg` check returns no missing files or headers |
| 2 | `docs/adr/README.md` index has 18 rows | `grep -c '| \[00' docs/adr/README.md` returns `18` |
| 3 | Every fork ADR (0007–0012) names ≥1 downstream slice in Consequences | 0007→3, 0008→3, 0009→2, 0010→2, 0011→2, 0012→2 slice refs |
| 4 | Every new convention doc cites its anchoring ADR | grep hit in each of the 5 files |
| 5 | Every `ADR 00NN` ref in `docs/` resolves | No broken refs |
| 6 | Every locked fork keyword is covered by an ADR | all six keywords hit ≥1 file |
| 7 | `docs/privacy.md` has `## Deferred (needs product input)` + ≥3 bullets | 6 bullets |
| 8 | `docs/privacy.md` no forbidden new-commitment strings outside `## Deferred` | grep-clean on "encryption at rest", "retention window", "deletion within", time SLAs |
| 9 | `docs/privacy.md` cited by ≥1 ADR | ADR 0015 cites it |
| 10 | Only `docs/adr/README.md` and `docs/conventions/events.md` edited among existing files (widened per Deviations §2) | `git diff --name-only --diff-filter=M main...HEAD -- 'docs/**'` returns exactly these two paths |

**Trust tiers:** every `docs/**` entry is `untested-assumption` —
markdown has no runtime and the decisions get exercised in slices
3/5/6/7/8 that cite them. The two exceptions are step 13 (README
index edit, `tested-against-real-input` via grep) and step 20
(the verify run itself, `tested-against-real-input`).

**Plan-review shape reused from part-1:** all five OQs resolved
in-spec before `plan-approved` landed; plan.md carries zero
"if OQ X" branches. Reviewer feedback (MSpider3 on PR #12) matched
the recommended answers on all five OQs — validation, not
rework.

**What downstream slices can now cite:**

* Slice 3 (contracts): ADR 0011 (commit + drift-check), ADR 0017
  (SemVer on contracts package), stream names from ADR 0006 +
  `events.md`.
* Slice 5 (compose): ADR 0004 (dev/prod overlay), ADR 0015 (OTel
  collector), ADR 0016 (Redis service + non-stream key
  conventions).
* Slice 6 (Rust API): ADR 0002 (layered + hex + VSA + CQRS),
  ADR 0005 (owner auth + RBAC), ADR 0008 (event scoping), ADR
  0009 (inline CPU ONNX), ADR 0010 (CPU-only image), ADR 0013
  (Noop wire-up), ADR 0014 (argon2id + dual rate-limit + magic-
  link), ADR 0015 (OTel), ADR 0017 (`/api/v1/`).
* Slice 7 (Python worker): ADR 0007 (photo + video-keyframe),
  ADR 0010 (CUDA+CPU-fallback), ADR 0012 (`uv`), ADR 0015 (OTel),
  ADR 0018 (Sphinx + MyST).
* Slice 8 (Next.js web): ADR 0008 (share-token UX), ADR 0015
  (OTel), ADR 0017 (hard-code `/api/v1/`), ADR 0018 (TypeDoc).

The epic's docs surface (part-1 + part-2) is now complete. The
remaining seven epic slices ship code against a stable ADR
vocabulary.
