# Epic: architecture scaffold

Tracking issue for the initial architecture-scaffold epic. Approved
plan: `/Users/siddharthgolecha/.claude/plans/you-are-the-lead-wobbly-prism.md`.

## Why an epic

The scaffold decomposes into 9 **independently reviewable** subtasks —
docs, contracts, diagrams, infra, three per-runtime walking skeletons,
and dev-workflow glue. Each stands on its own merits (an ADR PR can
be judged without seeing the Compose PR; the Rust skeleton reviews
independently of the Python skeleton). Landing them as one PR would
force reviewers to judge unrelated concerns in one sitting; landing
them as an epic lets each get its own `plan-approved` gate.

Ordering exists (Rust workspace precedes the API skeleton, etc.) but
is not total — slices 6/7/8 (the three walking skeletons) can proceed
in parallel once slice 5 (infra) lands. This is what makes it a real
epic rather than a chain of forced splits.

## Locked architectural forks

| Fork | Decision |
|---|---|
| Repo shape | Polyglot monorepo, no meta-tool |
| Deploy target | Single-VPS Docker Compose (dev + prod overlays) |
| Auth | Google OAuth + password login + Postgres-backed server-side sessions |
| Media scope | Media-abstracted day 1 (photo · video-keyframe) |
| Tenancy | Photographer-owned events; attendees search via share tokens |
| Indexing transport | Redis Streams (async, event-driven) |
| Search transport | API embeds selfie inline via CPU ONNX |
| Inference runtime | Worker: CUDA + CPU fallback; API: CPU-only |
| Event naming | Version on stream name (`jobs.media.index.v1`) |
| Generated contracts | Committed; CI enforces zero drift |
| Python deps | `uv` + `uv.lock` |

## Slice sequence

Each item becomes a child issue. Ordering matters — later slices
depend on earlier ones landing.

- [ ] `arch-conventions-and-adrs-part-1` — MADR template, ADRs 0001–0006, `docs/conventions/{coding,events,api}.md`. Docs only.
- [ ] `arch-conventions-and-adrs-part-2` — ADRs 0007–0018, remaining conventions, `docs/privacy.md`. Docs only.
- [ ] `arch-contracts-package` — `packages/contracts/` schemas + OpenAPI + generator + CI drift-check + overlap-list entries.
- [ ] `arch-diagrams-and-overview` — `docs/architecture/overview.md` + 7 Mermaid diagrams. Docs only.
- [ ] `arch-compose-and-infra` — `infra/compose/`, Postgres init, Redis, Caddy, OTel. API + worker + web not yet included.
- [ ] `arch-rust-api-walking-skeleton` — Rust workspace, `services/api/` with `/healthz`, empty module tree, Dockerfile.
- [ ] `arch-python-worker-walking-skeleton` — `services/worker/` with `uv`, empty consumer loop, ABC ports, CUDA/CPU Dockerfile.
- [ ] `arch-nextjs-web-walking-skeleton` — root `package.json` + `pnpm-workspace.yaml`, `apps/web/` scaffold, placeholder page.
- [ ] `arch-development-workflow` — top-level `Makefile`, bootstrap scripts, `docs/workflow/*`. Verifies full-epic acceptance.

Slices propose in order via `--parent <this-issue-number>`. `task_propose.sh`
will keep this checklist current as children land.
