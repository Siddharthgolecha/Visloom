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

Chosen: **`VISLOOM_ENV=dev` gate with startup assertion. Unset
= not dev.**

Rules, evaluated in order at API startup:

1. **If real OAuth or password credentials are configured**, the
   real provider is selected. `NoopAuthProvider` is unreachable
   in this case, regardless of `VISLOOM_ENV`.
2. **Otherwise, if `VISLOOM_ENV` is exactly the string `"dev"`**,
   `NoopAuthProvider` is selected. Any other value — including
   unset, empty string, `"prod"`, `"staging"`, or a typo like
   `"development"` — proceeds to rule 3.
3. **Otherwise, the API panics at startup** with a message
   pointing at this ADR. It does not boot in a degraded state,
   and it does not fall back to Noop.

Rationale for unset = panic: production Docker images that
forgot to set `VISLOOM_ENV` are the highest-risk failure mode —
a silent "Noop" default would authenticate every request as
`dev` in prod. Requiring `VISLOOM_ENV=dev` to be an **explicit,
opt-in** string means the dev fallback is impossible to reach
by accident.

The chosen provider is logged at startup (one line, INFO
level) so operators see which provider is live.

## Consequences

* Local dev workflow: `VISLOOM_ENV=dev` (default in
  `.env.example`, per slice 5), no OAuth setup needed, Noop
  boots, contributor sees the `dev` principal on every request.
* Prod deploy: `VISLOOM_ENV=prod`, real credentials configured
  — Noop is unreachable.
* Prod misconfiguration (real creds forgotten, `VISLOOM_ENV`
  still `prod`): API refuses to boot. Loud > quiet-degraded.
* Downstream: slice 6 (Rust API) implements the boot-time
  provider selection, the startup panic on env mismatch, and
  the one-line log.
