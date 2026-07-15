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

Chosen: **`VISLOOM_ENV=dev` gate with startup assertion.**

* At API startup, the `AuthProvider` selection reads
  `VISLOOM_ENV`. If unset or `dev`, and no real OAuth or
  password credentials are configured, `NoopAuthProvider` is
  selected.
* If `NoopAuthProvider` is selected AND `VISLOOM_ENV` is
  anything other than `dev` (or unset), the API **panics at
  startup** with a message pointing at this ADR. It does not
  boot in a degraded state.
* Noop is not selectable when real credentials **are**
  configured, regardless of `VISLOOM_ENV` — the real provider
  wins.
* The chosen provider is logged at startup (one line, INFO
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
