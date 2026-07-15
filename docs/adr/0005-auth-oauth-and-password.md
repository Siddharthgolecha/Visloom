# 0005 — Auth: Google OAuth + password login, Postgres sessions

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

Two user classes: photographers (frequent, want frictionless
Google sign-in) and attendees (one-shot, may not have Google
accounts). One IdP covers one class well and the other poorly.
Sessions themselves are uniform — server-side, revocable.

## Decision Drivers

* Photographers expect one-click Google.
* Attendees may lack Google — need a local email+password path.
* Sessions must be **revocable** (offboarding shouldn't wait for
  a JWT to expire).
* Local dev boots without real Google credentials.

## Considered Options

* **JWT stateless** — no server sessions.
* **Auth0 / Clerk** — hosted identity, dual paths.
* **Google-OAuth-only + Postgres sessions** — one IdP.
* **Google OAuth + password + Postgres sessions** (chosen).

## Decision Outcome

Chosen: **dual identity, single session backend.** Google OAuth
*or* email + password; both mint the same Postgres-backed opaque
session cookie. The `AuthProvider` port (ADR 0002) has two
adapters — `GoogleOAuthProvider`, `PasswordProvider` — both
producing a `Principal` for the session layer.

## Consequences

* Covers both user classes; sessions stay revocable.
* Adding a third IdP later = one more adapter (ADR 0002 port).
* Widens the auth surface — credential hashing, rate limiting,
  account recovery — those land in follow-up ADRs with slice 6;
  this ADR records the **choice** to support both, not the
  crypto details.
* **Local-dev fallback**: when no OAuth or password credentials
  are configured, the API selects a `NoopAuthProvider` minting a
  fixed `dev` principal, gated by `VISLOOM_ENV=dev` (fails loud
  elsewhere). Noop's wire-up + guard checks land in a future ADR
  with slice 6.
* Downstream: slice 6 creates the `AuthProvider` port + stubs
  `NoopAuthProvider`; slice 5 adds the Postgres `sessions` table
  init.
