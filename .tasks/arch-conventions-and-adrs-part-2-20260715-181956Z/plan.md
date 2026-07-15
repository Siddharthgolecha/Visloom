# Plan — arch-conventions-and-adrs-part-2

## Tactical steps

1. **Answer Open Questions on the draft PR** (spec §Open Questions) —
   post a comment on PR #12 with the five OQs so the human answers
   them before ADR authoring. OQs 1, 3, 4, 5 gate specific ADR
   content; OQ 2 gates ADR 0016 scope.
2. **Write ADR 0007 — Media scope: photo + video-keyframe day-1**
   (spec §ADR numbering, fork row `parent.md:28`). Considered Options:
   photo-only (rejected: excludes wedding-video use case),
   arbitrary-media (rejected: no ML pipeline for audio/PDF), chosen:
   photo + video-keyframe. Consequences names slice 7 (worker
   pipeline stages).
3. **Write ADR 0008 — Tenancy: owner-owned events + share tokens**
   (spec §ADR numbering, fork row `parent.md:29`). Uses "owner"
   vocabulary from ADR 0005 (not "photographer"). Considered
   Options: multi-tenant SaaS, per-owner isolation, chosen:
   owner-owned events with attendee share tokens. Consequences names
   slices 6 (API scoping) and 8 (web share-token UX).
4. **Write ADR 0009 — Search transport: API embeds selfie inline via
   CPU ONNX** (spec §ADR numbering, fork row `parent.md:31`).
   Considered Options: worker-embedded search (async, latency hit),
   inline API embedding (chosen, CPU ONNX bounds), external
   inference service (rejected, extra hop). Consequences names
   slice 6.
5. **Write ADR 0010 — Inference runtime: Worker CUDA+CPU / API
   CPU-only** (spec §ADR numbering, fork row `parent.md:32`).
   Considered Options: single runtime CPU-everywhere, single
   CUDA-everywhere, chosen: split (Worker CUDA-with-CPU-fallback,
   API CPU-only). Consequences names slices 6 and 7.
6. **Write ADR 0011 — Generated contracts: committed + CI drift-check**
   (spec §ADR numbering, fork row `parent.md:34`). Considered Options:
   runtime-generated (rejected: startup cost + no diff review),
   gitignored-generated (rejected: no reviewer visibility), chosen:
   commit generated code + CI drift-check. Consequences names
   slice 3 (contracts package + drift-check workflow).
7. **Write ADR 0012 — Python deps: `uv` + `uv.lock`** (spec §ADR
   numbering, fork row `parent.md:35`). Considered Options: poetry,
   pip + pip-tools, pdm, chosen: `uv`. Consequences names slice 7.
8. **Write ADR 0013 — NoopAuthProvider wire-up + guard checks**
   (spec §ADR numbering, `0005:119-120`). Considered Options: env
   flag + panic-in-prod, config-driven with allowlist, chosen:
   `VISLOOM_ENV=dev` gate with startup assertion + fail-loud logging
   on any non-dev boot. Consequences names slice 6 (API startup
   wiring).
9. **Write ADR 0014 — Password crypto + rate limit + recovery**
   (spec §ADR numbering, `0005:106-108`). Records algorithm choice
   (argon2id, per current OWASP guidance), rate-limit policy shape
   (per-IP + per-account exponential backoff), recovery posture
   (magic link over email; no security questions). Parameter tuning
   (memory cost, parallelism) is called out as slice-6 code-time
   work. Considered Options: bcrypt (rejected as legacy), scrypt
   (rejected: less deployed), argon2id (chosen). Consequences names
   slice 6.
10. **Write ADR 0015 — Observability: OTel-first, logs/traces/
    metrics** (spec §ADR numbering, `coding.md:15,34-36`, OQ 4).
    If OQ 4 says fold: three `### Decision Outcome` sub-sections
    (logs, traces, metrics) inside one ADR, per ADR 0006 precedent.
    Considered Options: OTel-first (chosen), split
    log/trace/metric stacks, vendor-locked (Datadog/Honeycomb).
    Consequences names slices 5, 6, 7, 8.
11. **Write ADR 0016 — Data storage: Postgres schema + Redis usage**
    (spec §ADR numbering, OQ 2). Scope depends on OQ 2 answer:
    (a) if lock Postgres schema evolution here → covers Postgres +
    Redis key naming with two Decision Outcome sub-sections;
    (b) if defer → covers Redis only with one Decision Outcome,
    Postgres deferred to slice 5. Considered Options and
    Consequences vary accordingly. Consequences names slices 5, 6,
    7.
12. **Write ADR 0017 — Versioning policy: URL-path + event streams
    + contracts** (spec §ADR numbering, `api.md:44-46`, OQ 5).
    Records the three-axis versioning story: HTTP APIs at `/api/v1/`
    (per `api.md`), event streams via `.v<int>` suffix (per ADR
    0006), generated contracts via SemVer on the contracts package
    (deferred to slice 3 for concrete rules). Considered Options:
    header-based versioning (rejected: hard to test),
    content-negotiation (rejected: same), URL-path + stream-suffix
    + SemVer (chosen). Consequences names slices 3 and 6.
13. **Write ADR 0018 — Documentation tooling: Sphinx / rustdoc /
    TypeDoc** (spec §ADR numbering, OQ 1). Per-language tool choice.
    If OQ 1 picks MyST → ADR records myst-parser + Google-style
    docstrings; if RST → ADR records raw RST + Google-style
    docstrings via napoleon. Considered Options: MkDocs single-tool
    (rejected — see spec §Alternative considered), pdoc for Python
    (rejected: no cross-refs), chosen: per-language tooling.
    Consequences names slices 6 (rustdoc setup), 7 (Sphinx setup),
    8 (TypeDoc setup).
14. **Update `docs/adr/README.md` index** (spec §Acceptance
    criteria) — append twelve new rows to the index table (lines
    11–16 today), one per new ADR. Each row:
    `[NNNN](NNNN-...md) | <title> | Accepted`.
15. **Write `docs/conventions/observability.md`** (spec §Convention
    docs) — cross-runtime logging + tracing + metrics shape. Names
    OTel SDKs per language (opentelemetry-rust, opentelemetry-python,
    `@opentelemetry/api` for TS). Cites ADR 0015.
16. **Write `docs/conventions/errors.md`** (spec §Convention docs) —
    unified error taxonomy. Domain errors as sealed enums / tagged
    unions per language (ADR 0002's ports live here); wire envelope
    per `api.md:25-33`. Names the "map domain error → wire error"
    responsibility as an adapter concern.
17. **Write `docs/conventions/data.md`** (spec §Convention docs,
    OQ 2). Postgres migration naming (`<ts>_<slug>.sql`), transaction
    boundaries (one transaction per application-layer handler), Redis
    key naming (`vloom:<domain>:<entity>:<id>`). Streams stay in
    `events.md`. Cites ADR 0016.
18. **Write `docs/conventions/testing.md`** (spec §Convention docs)
    — three tiers (unit / integration / contract) with adjacency
    rule: tests colocated with the module they exercise (per
    `coding.md:44-48`). Contract tests generated from
    `packages/contracts/` (slice 3). Fixture policy: fixtures are
    plain data, no fixture frameworks.
19. **Write `docs/conventions/documentation.md`** (spec §Convention
    docs, OQ 1). Python (Sphinx + Google-style + MyST-or-RST per
    OQ 1); Rust (rustdoc, `#[doc = ...]` for module-level, `//!`
    for crate-level); TypeScript (TSDoc + TypeDoc). Cites ADR 0018.
20. **Write `docs/privacy.md`** (spec §`docs/privacy.md`, OQ 3).
    Sections: What is PII in Visloom / Storage constraints / Log +
    span scrubbing rules / Retention posture / Deletion on request /
    Known leaky spots. Quantitative commitments per OQ 3 answer.
21. **Verify** (spec §Acceptance criteria) — run the eight
    falsifier checks in `spec.md § Acceptance criteria`. Tighten
    ADRs whose `Considered Options` overshoots or whose
    `Consequences` misses its downstream slice.
22. **Push, request review, poll for `plan-approved`.** Use
    `tools/autodev/poll_plan_approved.sh .tasks/<id>` after the
    human reviews `spec.md` + `plan.md`.

## Files touched

Eighteen new files under `docs/` + one small edit to
`docs/adr/README.md`. No file outside `docs/` (other than this
task dir's `spec.md`, `plan.md`, `implementation.md`).

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
- `docs/adr/0016-data-storage-postgres-and-redis.md`
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
  (step 14). No other edits to existing files.

## Depends on

- Part-1 (PR #11, merged `fb4b3f3`). Part-2's ADRs cite the MADR
  template + README + ADRs 0001–0006 that part-1 landed. Resolved.
- No parallel PR touches `docs/adr/` today (checked
  `gh pr list --search 'is:draft docs/adr'` at branch-open time).
  Overlap re-checked before requesting `plan-approved`.
