# Plan — arch-conventions-and-adrs-part-1

## Tactical steps

1. **Write `docs/adr/template.md`** (spec §Approach) — MADR-full
   skeleton with the five required section headers and inline
   `<!-- -->` guidance.
2. **Write `docs/adr/README.md`** (spec §Approach) — index +
   how-to-author. Numbering scheme (four-digit, monotonic), status
   vocabulary (`Proposed` / `Accepted` / `Superseded`), links to the
   six ADRs, one-paragraph pointer to `template.md`.
3. **Write ADR 0001 — Adopt MADR** (spec §ADR numbering) — framework
   ADR adopting the template. Considered Options names Nygard-style
   short ADRs as the runner-up. Status: Accepted.
4. **Write ADR 0002 — Layered + hex-where-appropriate; VSA + CQRS**
   (spec §ADR numbering, OQ 1) — framework ADR locking the
   domain/application/adapters/telemetry split, adopting **Vertical
   Slice Architecture** as feature-encapsulation, and the
   lightweight **CQRS rubric** (reads → layered, writes/side-effects
   → hex). Considered Options: pure layered vs. hex-everywhere vs.
   the chosen hybrid. Consequences names slices 6/7 as first
   materializations and the "default, not invariant" caveat.
5. **Write ADR 0003 — Polyglot monorepo** (spec §ADR numbering) —
   locks repo shape. Considered Options: polyrepo vs. Nx/Turborepo
   meta-tool vs. plain monorepo. Consequences names the root
   `pnpm-workspace.yaml` (slice 8) and root `Cargo.toml` workspace
   (slice 6).
6. **Write ADR 0004 — Docker Compose on single VPS** (spec §ADR
   numbering) — locks deploy target. Considered Options: k8s,
   Fly.io, plain Compose. Consequences names slice 5's dev + prod
   overlay pattern.
7. **Write ADR 0005 — Owner auth (Google OAuth + password
   backup) and role-based access** (spec §ADR numbering, OQ 4,
   §Deviations) — locks three coupled decisions in one file with
   three `### Decision Outcome` sub-sections: (a) **attendee
   access** is unauthenticated via a per-event revocable
   share-token URL; (b) **owner identity** is Google OAuth as
   primary + email/password as backup, minting the same
   Postgres session cookie; (c) **authorization** is role-based
   per event — `owner`, `editor`, `reader`. Considered Options
   split into owner-identity alternatives (Google-only, password-
   only, Auth0, chosen dual) and authorization alternatives
   (binary owner/not-owner, full ABAC, chosen 3-role RBAC).
   Consequences names the `NoopAuthProvider` local-dev fallback
   and defers credential-hashing / rate-limit / recovery tech to
   a slice-6 ADR.
8. **Write ADR 0006 — Redis Streams + versioned naming** (spec §ADR
   numbering, OQ 2) — locks indexing transport and stream-name
   versioning. **Two `### Decision Outcome` sub-sections** (per
   OQ 2 resolution): one for transport (Redis Streams over
   RabbitMQ / pg LISTEN-NOTIFY), one for naming (`.v<int>` on
   stream name, dot-separated, lowercase). Names the three
   canonical streams verbatim so `events.md` and slice-3 contract
   filenames can cite this ADR.
9. **Write `docs/conventions/coding.md`** (spec §Approach) —
   cross-language rules: file naming, layering (per ADR 0002),
   error-handling posture, logging shape, test-adjacency, DI at the
   service boundary. No language-specific style rules.
10. **Write `docs/conventions/events.md`** (spec §Approach) — event
    naming scheme (per ADR 0006), stream-vs-consumer-group
    semantics, versioning rule ("version on stream name; version
    bump = new stream + new consumer group; cutover is explicit").
    Lists the three canonical stream names verbatim.
11. **Write `docs/conventions/api.md`** (spec §Approach, OQ 3) —
    HTTP API conventions: URL shape (`/api/v1/*` behind Caddy
    same-origin — resolved OQ 3), health-endpoint contract
    (`/healthz`), error envelope, idempotency headers, versioning
    posture (URL-path `/api/v1/`).
12. **Update parent fork table** (spec §Deviations) — edit two
    rows of `.tasks/epics/arch-scaffold/parent.md`: **Auth**
    reads "Owners: Google OAuth + password backup, Postgres
    sessions. Attendees: unauthenticated, share-token URL. RBAC
    per event: owner/editor/reader." **Tenancy** reads
    "Owner-managed events; attendees browse via share tokens"
    (replacing "Photographer-owned"). Matches ADR 0005.
13. **Verify** (spec §Acceptance criteria) — run the checks in the
    spec's Verification section.

## Files touched

Ten new files under `docs/` plus one small edit to the parent fork
table.

- `docs/adr/README.md` — new. ADR index + numbering rules.
- `docs/adr/template.md` — new. MADR-full skeleton.
- `docs/adr/0001-adopt-madr.md` — new. Framework ADR.
- `docs/adr/0002-layered-hexagonal-architecture.md` — new.
  Framework ADR (VSA + CQRS refinement).
- `docs/adr/0003-polyglot-monorepo.md` — new. Fork ADR.
- `docs/adr/0004-docker-compose-single-vps.md` — new. Fork ADR.
- `docs/adr/0005-owner-auth-and-rbac.md` — new. Fork ADR
  covering attendee share-token access, owner Google OAuth +
  password backup, and per-event RBAC (owner/editor/reader).
- `docs/adr/0006-redis-streams-versioned-naming.md` — new. Fork
  ADR with two `### Decision Outcome` sub-sections.
- `docs/conventions/coding.md` — new. Cross-language rules.
- `docs/conventions/events.md` — new. Stream naming + versioning.
- `docs/conventions/api.md` — new. HTTP API conventions with
  `/api/v1/` URL-path versioning.
- `.tasks/epics/arch-scaffold/parent.md` — edit the Auth and
  Tenancy rows of the fork table.

## Depends on

None. This is the first slice of the epic; no upstream PR blocks it.
Downstream slices (2, 3, 5, 6, 8) will cite this slice's ADRs and
convention docs by number and path.
