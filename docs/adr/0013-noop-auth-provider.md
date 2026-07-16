# 0013 — NoopAuthProvider wire-up + guard checks

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

ADR 0005 committed to a `NoopAuthProvider` that mints a fixed
`dev` principal with the `owner` role when no real credentials
are configured (`docs/adr/0005-owner-auth-and-rbac.md:119-120`).
The tech details of how Noop wires in were deferred to this ADR.
The failure mode we care about most: Noop accidentally boots in
production, silently authenticating every request as `dev`. A
convenient dev fallback that fails soft in prod is worse than
having no fallback at all.

## Decision Drivers

* Local dev must boot with **no** environment configuration
  beyond one flag.
* Any non-dev boot with Noop selected must fail **loud** — not
  authenticate as `dev` and emit a warning.
* `AuthProvider` port (ADR 0002) selection happens once at
  startup, not per request — no runtime cost to the guard.

## Considered Options

* **Env flag + panic-in-prod.** Single `VISLOOM_ENV=dev` gate;
  panic if Noop is selected and env is anything other than
  `dev`. Simple, hard to accidentally bypass.
* **Config-driven with allowlist.** A config field lists
  environments where Noop is allowed. More flexible; easier to
  misconfigure.
* **Silent fall-through** (default to real provider, warn if
  Noop wanted but env is prod). Rejected outright — the failure
  mode we want to avoid.

## Decision Outcome

Chosen: **`VISLOOM_ENV=dev` gate, enforced twice.** Startup
selection guard **and** a constructor-level guard inside
`NoopAuthProvider` itself. Unset ≠ dev; the check is
case-sensitive, exact-match.

### Selection guard (at boot)

Rules, evaluated in order at API startup:

1. **If real OAuth or password credentials are configured**, the
   real provider is selected. `NoopAuthProvider` is unreachable
   through this path, regardless of `VISLOOM_ENV`.
2. **Otherwise, if `VISLOOM_ENV` is exactly the string `"dev"`**
   (byte-for-byte, no case-fold, no whitespace trim),
   `NoopAuthProvider::new()` is called and the resulting
   provider is selected. Any other value — including unset,
   empty string, `"DEV"`, `" dev"`, `"prod"`, `"staging"`, or a
   typo like `"development"` — proceeds to rule 3.
3. **Otherwise, the API panics at startup** with a message
   naming (a) which env var was read, (b) the observed value
   (or "unset"), and (c) a link back to this ADR. It does not
   boot in a degraded state, and it does not fall back to Noop.

### Constructor guard (defense in depth)

`NoopAuthProvider::new()` re-runs the same `VISLOOM_ENV=="dev"`
check and panics on mismatch. This exists for the case where
slice 6 code (or a test, or a future refactor) instantiates
`NoopAuthProvider` directly without going through the selection
guard above — the constructor refuses. Same panic message
shape. Cost is one env-var read per construction; benefit is
that there is no code path in the API that can produce a live
`NoopAuthProvider` outside `VISLOOM_ENV=dev`.

The selected provider is logged at startup (one line, INFO
level) so operators can observe which one is live: for Noop,
the log line explicitly includes the string `"NoopAuthProvider
active — VISLOOM_ENV=dev"` so `grep`-based prod-readiness
checks can flag it.

### Rationale

Two-layer guard because production Docker images that forgot to
set `VISLOOM_ENV` are the highest-risk failure mode: a silent
"Noop" default would authenticate every request as `dev`. The
selection guard closes the intended path; the constructor guard
closes the unintended one (accidental direct instantiation).
Together they make it impossible to boot a live
`NoopAuthProvider` outside an explicitly opted-in dev
environment.

## Consequences

* Local dev workflow: `VISLOOM_ENV=dev` (default in
  `.env.example`, per slice 5), no OAuth setup needed, Noop
  boots, contributor sees the `dev` principal on every request.
* Prod deploy: `VISLOOM_ENV=prod`, real credentials configured
  — Noop is unreachable.
* Prod misconfiguration (real creds forgotten, `VISLOOM_ENV`
  still `prod`): API refuses to boot. Loud > quiet-degraded.
* Downstream: slice 6 (Rust API) implements **both** the
  boot-time selection guard and the constructor-level guard
  inside `NoopAuthProvider::new()`, plus the startup panic on
  env mismatch and the `"NoopAuthProvider active"` INFO log.
  The constructor guard is a Rust `assert!` (or equivalent) —
  a unit test in slice 6 asserts that `NoopAuthProvider::new()`
  panics when `VISLOOM_ENV` is unset or non-`"dev"`.
