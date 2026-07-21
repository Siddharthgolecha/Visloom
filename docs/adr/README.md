# Architecture Decision Records

Visloom records architecture decisions as ADRs using the
[MADR](https://adr.github.io/madr/) format (full variant, five
sections). One decision per file, numbered monotonically.

## Index

| # | Title | Status |
|---|---|---|
| [0001](0001-adopt-madr.md) | Adopt MADR for architecture decisions | Accepted |
| [0002](0002-layered-hexagonal-architecture.md) | Layered + hex-where-appropriate; VSA + lightweight CQRS | Accepted |
| [0003](0003-polyglot-monorepo.md) | Polyglot monorepo, no meta-tool | Accepted |
| [0004](0004-docker-compose-single-vps.md) | Docker Compose on a single VPS | Accepted |
| [0005](0005-owner-auth-and-rbac.md) | Owner auth (Google OAuth + password backup) and role-based access | Accepted |
| [0006](0006-redis-streams-versioned-naming.md) | Redis Streams for indexing + versioned stream naming | Accepted |
| [0007](0007-media-scope-photo-and-video-keyframe.md) | Media scope: photo + video-keyframe day-1 | Accepted |
| [0008](0008-tenancy-owner-events-and-share-tokens.md) | Tenancy: owner-owned events + share tokens | Accepted |
| [0009](0009-search-transport-cpu-onnx-inline.md) | Search transport: API embeds selfie inline via CPU ONNX | Accepted |
| [0010](0010-inference-runtime-worker-cuda-api-cpu.md) | Inference runtime: Worker CUDA+CPU / API CPU-only | Accepted |
| [0011](0011-generated-contracts-committed-with-drift-check.md) | Generated contracts: committed + CI drift-check | Accepted |
| [0012](0012-python-deps-uv-and-uv-lock.md) | Python deps: `uv` + `uv.lock` | Accepted |
| [0013](0013-noop-auth-provider.md) | NoopAuthProvider wire-up + guard checks | Accepted |
| [0014](0014-password-crypto-and-rate-limit.md) | Password crypto + rate limit + recovery | Accepted |
| [0015](0015-observability-otel-first.md) | Observability: OTel-first (logs / traces / metrics folded) | Accepted |
| [0016](0016-redis-usage.md) | Redis usage: key naming + TTL taxonomy | Accepted |
| [0017](0017-versioning-policy.md) | Versioning policy: URL-path + event streams + contracts | Accepted |
| [0018](0018-documentation-tooling.md) | Documentation tooling: MyST-Sphinx / rustdoc / TypeDoc | Accepted |
| [0019](0019-architecture-diagrams-mermaid.md) | Architecture diagrams: inline Mermaid, flowchart-styled C4 | Accepted |
| [0020](0020-postgres-migration-format.md) | Postgres migration format + provisional embeddings dimension | Accepted |

## Numbering

Four-digit, monotonic, no gaps. `0021` is next. Numbers are
allocated at PR-open time. Superseded ADRs keep their number and
gain a `Superseded by [NNNN](NNNN-...md)` line in the `Status`
field — never renumber.

## Status vocabulary

* **Proposed** — under review, not yet load-bearing.
* **Accepted** — merged; slices may cite it.
* **Superseded by NNNN** — replaced; keep the file for the audit
  trail, add the `Superseded by` link at the top.

## Authoring

Copy `template.md`, replace `NNNN` with the next number, and fill
all five sections. `Considered Options` must name at least one
real alternative (per `AGENTS.md` §2). Fork ADRs (those recording
a locked architectural fork from the epic) must additionally name
at least one downstream slice in `Consequences` that will consume
the decision.
