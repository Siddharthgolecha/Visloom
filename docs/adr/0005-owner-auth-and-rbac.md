# 0005 — Owner auth (Google OAuth + password backup) and role-based access

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

Visloom has two access classes with very different postures:

* **Owners** — people who create and manage events (photographers,
  event coordinators, venue staff). Log in often; need
  frictionless, durable identity.
* **Attendees** — people who arrive at an event to find their
  photos. **Do not log in.** They open a share-token URL and
  browse — no account, no session, no email required.

Because attendees are unauthenticated, the auth ADR only has to
solve the owner path. And because owners are not a single job
title, "photographer" is the wrong noun: an event can be owned by
anyone the platform grants access to.

Once owners can log in, we also need to answer: what can they do
inside an event? A single "logged in = full control" role
collapses under any shared-event use case (a venue staff member
adding photos shouldn't be able to delete the event).

## Decision Drivers

* Attendees must reach their photos without an account.
* Owners want one-click sign-in on their primary device.
* Owners must have a backup path when Google is unavailable or
  their account is locked (photographers regularly hit this
  before shoots).
* Sessions must be revocable per device (offboarding, lost
  phones).
* Multiple humans can co-own an event — roles must be first-class,
  not bolted on later.
* Local dev boots without real Google credentials.

## Considered Options

### For owner identity

* **Google OAuth only** — one IdP, no fallback. Rejected: single
  point of failure at the worst moment.
* **Password only** — no OAuth, own the credential store.
  Rejected: unnecessary friction for the primary path.
* **Auth0 / Clerk** — hosted identity. Rejected: extra vendor for
  a small, well-defined surface.
* **Google OAuth + password backup + Postgres sessions**
  (chosen).

### For authorization

* **Binary owner/not-owner** — the "one logged-in role" trap.
  Rejected.
* **Full ABAC** (attribute-based, per-photo policies). Rejected
  as premature.
* **Role-based access with three roles per event** (chosen):
  `owner`, `editor`, `reader`.

## Decision Outcome

### Decision Outcome — Attendee access

Attendees are **unauthenticated**. They receive a share-token URL
(opaque, per-event, revocable) that grants scoped read access to
the event's public photos. No cookie, no principal, no session —
just token validation on each request. Rate-limiting and abuse
prevention land in the API scaffold.

### Decision Outcome — Owner identity

Owners authenticate via **Google OAuth** as the primary path, or
**email + password** as a backup for the same account. Both paths
mint the same Postgres-backed opaque session cookie. The API's
`AuthProvider` port (ADR 0002) has two adapters —
`GoogleOAuthProvider` and `PasswordProvider` — both producing a
`Principal` for the session layer. An owner's account can have
both credentials attached; the password path is opt-in from
account settings.

### Decision Outcome — Roles

Three roles per event, evaluated per principal + event:

* `owner` — full control (rename, delete event, invite/remove
  users, grant/revoke roles, upload, edit metadata).
* `editor` — upload and edit media, cannot manage users or the
  event lifecycle.
* `reader` — view only. Distinct from the attendee share-token
  path because a reader is an authenticated principal (auditable,
  revocable per user without invalidating a share-token URL).

The first owner of an event is the principal that created it;
additional roles are granted by any existing owner. Role
evaluation happens in `application/` handlers; the check is a
port on the API side (`AuthzPolicy`) implemented against the
Postgres `event_memberships` table.

## Consequences

* Attendee flow stays login-less: share-token in, photos out.
  Simplest possible funnel.
* Owner surface widens: credential hashing, rate-limiting on the
  password path, and account recovery all become in-scope for
  slice 6's follow-up ADRs. This ADR records the **choice** to
  support both credentials + roles, not the crypto or policy
  details.
* Sessions revocable per device (Postgres-backed cookies).
* Role model is small on purpose — three roles cover the
  observed workflows; a fourth (e.g. `moderator`) can be added
  without an ADR revision because the model is data-driven.
* **Local-dev fallback**: when neither OAuth nor password
  credentials are configured, the API selects a
  `NoopAuthProvider` minting a fixed `dev` principal with the
  `owner` role on every event. Gated by `VISLOOM_ENV=dev`; fails
  loud elsewhere. Noop's wire-up + guard checks land in a future
  ADR with slice 6.
* Downstream: slice 6 creates the `AuthProvider` +
  `AuthzPolicy` ports and stubs `NoopAuthProvider`; slice 5 adds
  the Postgres `sessions`, `event_memberships`, and share-token
  tables via init scripts. Slice 3's OpenAPI spec marks
  attendee-facing routes as `security: []` (unauthenticated) and
  owner-facing routes with the session-cookie scheme.
