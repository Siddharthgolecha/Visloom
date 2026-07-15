# Plan — arch-conventions-and-adrs-part-1

## Tactical steps

1. **Write `docs/adr/template.md`** (spec §Approach) — MADR-full
   skeleton with the five required section headers and inline
   `<!-- -->` guidance. ~30 LOC.
2. **Write `docs/adr/README.md`** (spec §Approach) — index +
   how-to-author. Numbering scheme (four-digit, monotonic), status
   vocabulary (`Proposed` / `Accepted` / `Superseded`), links to the
   six ADRs, one-paragraph pointer to `template.md`. ~30 LOC.
3. **Write ADR 0001 — Adopt MADR** (spec §ADR numbering) — framework
   ADR adopting the template. Considered Options names Nygard-style
   short ADRs as the runner-up. Status: Accepted. ~30 LOC.
4. **Write ADR 0002 — Layered + hexagonal-where-appropriate** (spec
   §ADR numbering, OQ 1) — framework ADR locking the
   domain/application/adapters/telemetry split. Considered Options:
   pure layered vs. hexagonal-everywhere vs. chosen hybrid.
   Consequences names slices 6/7 as first materializations. ~40 LOC.
5. **Write ADR 0003 — Polyglot monorepo** (spec §ADR numbering) —
   locks repo shape. Considered Options: polyrepo vs. Nx/Turborepo
   meta-tool vs. plain monorepo. Consequences names the root
   `pnpm-workspace.yaml` (slice 8) and root `Cargo.toml` workspace
   (slice 6). ~35 LOC.
6. **Write ADR 0004 — Docker Compose on single VPS** (spec §ADR
   numbering) — locks deploy target. Considered Options: k8s,
   Fly.io, plain Compose. Consequences names slice 5's dev + prod
   overlay pattern. ~35 LOC.
7. **Write ADR 0005 — Google OAuth + server sessions** (spec §ADR
   numbering, OQ 5) — locks auth model. Considered Options: JWT
   stateless, Auth0, chosen Google-OAuth + Postgres-session.
   Consequences names the `NoopAuthProvider` dev fallback (subject
   to OQ 5). ~35 LOC.
8. **Write ADR 0006 — Redis Streams + versioned naming** (spec §ADR
   numbering, OQ 2) — locks indexing transport and stream-name
   versioning. Considered Options: RabbitMQ, Postgres
   LISTEN/NOTIFY, chosen Redis Streams. Names the three canonical
   streams verbatim so `events.md` and slice-3 contract filenames
   can cite this ADR. ~40 LOC.
9. **Write `docs/conventions/coding.md`** (spec §Approach) —
   cross-language rules: file naming, layering (per ADR 0002),
   error-handling posture, logging shape, test-adjacency, DI at the
   service boundary. No language-specific style rules. ~30 LOC.
10. **Write `docs/conventions/events.md`** (spec §Approach) — event
    naming scheme (per ADR 0006), stream-vs-consumer-group
    semantics, versioning rule ("version on stream name; version
    bump = new stream + new consumer group; cutover is explicit").
    Lists the three canonical stream names verbatim. ~25 LOC.
11. **Write `docs/conventions/api.md`** (spec §Approach, OQ 4) —
    HTTP API conventions: URL shape (`/api/*` behind Caddy
    same-origin), health-endpoint contract (`/healthz`), error
    envelope, idempotency headers, versioning posture (URL-path
    `/v1/`, subject to OQ 4). ~25 LOC.
12. **Verify** (spec §Acceptance criteria) — run the five
    checks in the spec's Verification section; tighten fork-ADR
    Considered Options prose if diff exceeds 350 LOC.

## Files touched

Ten new files under `docs/`. No edits to existing files.

- `docs/adr/README.md` — new. ADR index + numbering rules.
- `docs/adr/template.md` — new. MADR-full skeleton.
- `docs/adr/0001-adopt-madr.md` — new. Framework ADR.
- `docs/adr/0002-layered-hexagonal-architecture.md` — new.
  Framework ADR.
- `docs/adr/0003-polyglot-monorepo.md` — new. Fork ADR.
- `docs/adr/0004-docker-compose-single-vps.md` — new. Fork ADR.
- `docs/adr/0005-google-oauth-server-sessions.md` — new. Fork ADR.
- `docs/adr/0006-redis-streams-versioned-naming.md` — new. Fork ADR.
- `docs/conventions/coding.md` — new. Cross-language rules.
- `docs/conventions/events.md` — new. Stream naming + versioning.
- `docs/conventions/api.md` — new. HTTP API conventions.

## Depends on

None. This is the first slice of the epic; no upstream PR blocks it.
Downstream slices (2, 3, 5, 6, 8) will cite this slice's ADRs and
convention docs by number and path.
