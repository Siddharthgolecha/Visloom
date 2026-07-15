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
| [0005](0005-auth-oauth-and-password.md) | Auth: Google OAuth + password login, Postgres sessions | Accepted |
| [0006](0006-redis-streams-versioned-naming.md) | Redis Streams for indexing + versioned stream naming | Accepted |

## Numbering

Four-digit, monotonic, no gaps. `0007` is next. Numbers are
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
