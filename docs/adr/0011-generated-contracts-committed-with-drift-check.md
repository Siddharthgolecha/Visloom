# 0011 — Generated contracts: committed + CI drift-check

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Slice 3 lands `packages/contracts/` with JSON Schemas for event
streams (per ADR 0006), an OpenAPI 3.1 skeleton (per
`docs/conventions/api.md`), and generated language bindings for
Rust / Python / TypeScript. The generated files are derivable
from the source schemas — question is whether they live in the
repo or are produced at build time. This affects reviewer
visibility, PR diffs, and startup cost.

## Decision Drivers

* Reviewers must see what changes when a schema changes — a PR
  that touches contracts should show the propagated diff in the
  generated files, not hide it behind a build step.
* No runtime code generation on service boot: startup should not
  fork out to a codegen tool.
* CI must detect the case where a contributor edits a schema
  but forgets to regenerate — zero drift between source and
  generated at merge time.
* `AGENTS.md` §6 overlap-list needs entries for anything a
  downstream slice would step on (contract file paths qualify).

## Considered Options

* **Runtime-generated.** Services generate bindings at boot.
  Cold-start tax; drift is invisible until runtime; reviewer
  can't see the propagated change.
* **Gitignored-generated.** Files exist locally but not in the
  repo. Reviewer never sees them; onboarding requires running
  the generator to get any binding.
* **Committed + CI drift-check** (chosen).

## Decision Outcome

Chosen: **commit generated bindings + CI drift-check.**

* Generated files live under `packages/contracts/{ts,rust,py}/`
  and are checked in.
* A `scripts/gen-contracts.sh` (slice 3) drives the codegen.
* CI runs `make contracts && git diff --exit-code` — non-empty
  diff fails the build. The workflow lives under
  `.github/workflows/` and lands with slice 3.
* Generator versions are **pinned** in the script so the
  drift-check is deterministic across contributor machines.
* Slice 3 adds `packages/contracts/**` and the stream names
  from ADR 0006 to `AGENTS.md` §6 overlap list.

## Consequences

* Reviewers see the full blast radius of a schema change in one
  diff.
* Contributors must run `make contracts` locally when touching a
  schema. Documented in slice 9's development-workflow docs.
* Generator upgrade = one PR that reruns codegen against pinned
  new version + regenerates everything; the diff is the audit.
* Downstream: slice 3 owns `packages/contracts/`,
  `scripts/gen-contracts.sh`, the CI workflow, and the overlap-
  list entries; every later slice that consumes a contract just
  imports the generated file.
