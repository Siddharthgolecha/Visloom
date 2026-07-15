# Plan — arch-conventions-and-adrs-part-2

## Tactical steps

Sequenced for implementation **after** `plan-approved` is granted;
`spec.md` + this `plan.md` are the artifacts under review. All
five OQs are resolved in `spec.md` §Open Questions — this plan
has no `if OQ X` branches.

1. **Write ADR 0007 — Media scope: photo + video-keyframe day-1**
   (spec §ADR numbering, fork row `parent.md:28`). Considered
   Options: photo-only (rejected: excludes wedding-video use case),
   arbitrary-media (rejected: no ML pipeline for audio/PDF), chosen:
   photo + video-keyframe. Consequences names slice 7 (worker
   pipeline stages).
2. **Write ADR 0008 — Tenancy: owner-owned events + share tokens**
   (spec §ADR numbering, fork row `parent.md:29`). Uses "owner"
   vocabulary from ADR 0005 (not "photographer"). Considered
   Options: multi-tenant SaaS, per-owner isolation, chosen:
   owner-owned events with attendee share tokens. Consequences
   names slices 6 (API scoping) and 8 (web share-token UX).
3. **Write ADR 0009 — Search transport: API embeds selfie inline
   via CPU ONNX** (spec §ADR numbering, fork row `parent.md:31`).
   Considered Options: worker-embedded search (async, latency hit),
   inline API embedding (chosen, CPU ONNX bounds), external
   inference service (rejected, extra hop). Consequences names
   slice 6.
4. **Write ADR 0010 — Inference runtime: Worker CUDA+CPU / API
   CPU-only** (spec §ADR numbering, fork row `parent.md:32`).
   Considered Options: single-runtime CPU-everywhere, single
   CUDA-everywhere, chosen: split (Worker CUDA-with-CPU-fallback,
   API CPU-only). Consequences names slices 6 and 7.
5. **Write ADR 0011 — Generated contracts: committed + CI drift-
   check** (spec §ADR numbering, fork row `parent.md:34`).
   Considered Options: runtime-generated (rejected: startup cost +
   no diff review), gitignored-generated (rejected: no reviewer
   visibility), chosen: commit generated code + CI drift-check.
   Consequences names slice 3 (contracts package + drift-check
   workflow).
6. **Write ADR 0012 — Python deps: `uv` + `uv.lock`** (spec §ADR
   numbering, fork row `parent.md:35`). Considered Options: poetry,
   pip + pip-tools, pdm, chosen: `uv`. Consequences names slice 7.
7. **Write ADR 0013 — NoopAuthProvider wire-up + guard checks**
   (spec §ADR numbering, `0005:119-120`). Considered Options: env
   flag + panic-in-prod, config-driven with allowlist, chosen:
   `VISLOOM_ENV=dev` gate with startup assertion + fail-loud
   logging on any non-dev boot. Consequences names slice 6 (API
   startup wiring).
8. **Write ADR 0014 — Password crypto + rate limit + recovery**
   (spec §ADR numbering, `0005:106-108`). Records algorithm choice
   (argon2id, per current OWASP guidance), rate-limit policy shape
   (per-IP + per-account exponential backoff), recovery posture
   (magic link over email; no security questions). Parameter tuning
   (memory cost, parallelism) is called out as slice-6 code-time
   work. Considered Options: bcrypt (rejected as legacy), scrypt
   (rejected: less deployed), argon2id (chosen). Consequences names
   slice 6.
9. **Write ADR 0015 — Observability: OTel-first, folded**
   (spec §ADR numbering, `coding.md:15,34-36`, OQ 4 resolved to
   fold). Three `### Decision Outcome` sub-sections (logs, traces,
   metrics) inside one ADR, per the ADR 0006 precedent. Considered
   Options: OTel-first (chosen), split log/trace/metric stacks,
   vendor-locked (Datadog / Honeycomb). Consequences names slices
   5, 6, 7, 8.
10. **Write ADR 0016 — Redis usage** (spec §ADR numbering, OQ 2
    resolved to defer Postgres). One `## Decision Outcome`
    covering Redis key naming and expiration policy. Postgres
    schema evolution is explicitly out-of-scope and named as a
    future ADR alongside slice 5. Considered Options: no
    convention (rejected: naming drift), one-key-one-purpose
    (chosen), namespaced with TTL taxonomy (chosen sub-decision).
    Consequences names slices 5 (compose init) and 6 (API
    sessions).
11. **Write ADR 0017 — Versioning policy: URL-path + event streams
    + contracts** (spec §ADR numbering, `api.md:44-46`, OQ 5
    resolved to docs-only). Records the three-axis versioning
    story: HTTP APIs at `/api/v1/` (per `api.md`), event streams
    via `.v<int>` suffix (per ADR 0006), generated contracts via
    SemVer on the contracts package (deferred to slice 3 for
    concrete wire-format rules). Considered Options: header-based
    versioning (rejected: hard to test), content-negotiation
    (rejected: same), URL-path + stream-suffix + SemVer (chosen).
    Consequences names slices 3 and 6.
12. **Write ADR 0018 — Documentation tooling: MyST-Sphinx /
    rustdoc / TypeDoc** (spec §ADR numbering, OQ 1 resolved to
    MyST). Per-language tool choice. Records `myst-parser` +
    Sphinx + Google-style docstrings for Python; `rustdoc` for
    Rust; TSDoc + TypeDoc for TypeScript. Considered Options:
    MkDocs single-tool (rejected — see spec §Alternative
    considered), pdoc for Python (rejected: no cross-refs), raw
    RST (rejected: docstrings should match the docs tree's
    Markdown), chosen: per-language tooling with MyST for Python.
    Consequences names slices 6 (rustdoc setup), 7 (Sphinx +
    MyST setup), 8 (TypeDoc setup); also names the supersession
    path if MyST ecosystem doesn't hold up.
13. **Edit `docs/adr/README.md` index** (spec §Acceptance
    criteria) — append twelve new rows to the index table (lines
    11–16 today), one per new ADR. Each row:
    `[NNNN](NNNN-...md) | <title> | Accepted`. This is the sole
    existing-file edit in the PR.
14. **Write `docs/conventions/observability.md`** (spec §Convention
    docs) — cross-runtime logging + tracing + metrics shape. Names
    OTel SDKs per language (opentelemetry-rust, opentelemetry-
    python, `@opentelemetry/api` for TS). Cites ADR 0015.
15. **Write `docs/conventions/errors.md`** (spec §Convention docs)
    — unified error taxonomy. Domain errors as sealed enums /
    tagged unions per language (ADR 0002's ports live here); wire
    envelope per `api.md:25-33`. Names the "map domain error → wire
    error" responsibility as an adapter concern.
16. **Write `docs/conventions/data.md`** (spec §Convention docs,
    OQ 2 resolved to Redis-only). Redis key naming
    (`vloom:<domain>:<entity>:<id>`), TTL taxonomy for sessions +
    share tokens, streams stay in `events.md`. Postgres
    conventions are explicitly out-of-scope with a `## Deferred`
    section pointing to the slice-5 follow-up. Cites ADR 0016.
17. **Write `docs/conventions/testing.md`** (spec §Convention docs)
    — three tiers (unit / integration / contract) with adjacency
    rule: tests colocated with the module they exercise (per
    `coding.md:44-48`). Contract tests generated from
    `packages/contracts/` (slice 3). Fixture policy: fixtures are
    plain data, no fixture frameworks.
18. **Write `docs/conventions/documentation.md`** (spec §Convention
    docs, OQ 1 resolved to MyST-Sphinx). Python (Sphinx +
    `myst-parser` + Google-style docstrings); Rust (rustdoc, `//!`
    for crate-level, `//` for item-level); TypeScript (TSDoc +
    TypeDoc). Cites ADR 0018.
19. **Write `docs/privacy.md`** (spec §`docs/privacy.md`, OQ 3
    resolved to posture-only). Sections: **What is sensitive**,
    **Where sensitive content must not appear**, **Known leaky
    spots**, **Deferred (needs product input)**. Restates only
    `AGENTS.md:52-55` and `CONTRIBUTING.md:30-31`; introduces no
    new commitments. Enumerate the deferred items (encryption at
    rest, log retention, deletion SLA, IP-log posture,
    cross-border transfer) and mirror them into
    `implementation.md` as follow-up work.
20. **Verify** (spec §Acceptance criteria) — run the ten falsifier
    checks. Cross-check the fork-coverage criterion by grepping
    `docs/adr/00*.md` for each of the six fork keywords, not by
    inspecting the spec table.

## Files touched

Eighteen new files under `docs/` plus one edit to
`docs/adr/README.md`. No file outside `docs/` other than this task
dir's `spec.md`, `plan.md`, `implementation.md`.

New:

- `docs/adr/0007-media-scope-photo-and-video-keyframe.md`
- `docs/adr/0008-tenancy-owner-events-and-share-tokens.md`
- `docs/adr/0009-search-transport-cpu-onnx-inline.md`
- `docs/adr/0010-inference-runtime-worker-cuda-api-cpu.md`
- `docs/adr/0011-generated-contracts-committed-with-drift-check.md`
- `docs/adr/0012-python-deps-uv-and-uv-lock.md`
- `docs/adr/0013-noop-auth-provider.md`
- `docs/adr/0014-password-crypto-and-rate-limit.md`
- `docs/adr/0015-observability-otel-first.md`
- `docs/adr/0016-redis-usage.md`
- `docs/adr/0017-versioning-policy.md`
- `docs/adr/0018-documentation-tooling.md`
- `docs/conventions/observability.md`
- `docs/conventions/errors.md`
- `docs/conventions/data.md`
- `docs/conventions/testing.md`
- `docs/conventions/documentation.md`
- `docs/privacy.md`

Edited:

- `docs/adr/README.md` — append twelve rows to the index table
  (step 13). **Sole existing-file edit in this PR.**

## Depends on

- Part-1 (PR #11, merged `fb4b3f3`). Part-2's ADRs cite the MADR
  template + README + ADRs 0001–0006 that part-1 landed. Resolved.
- No parallel PR touches `docs/adr/` today (checked
  `gh pr list --search 'is:draft docs/adr'` at branch-open time).
  Overlap re-checked before requesting `plan-approved`.
