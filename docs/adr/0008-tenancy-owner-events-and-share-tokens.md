# 0008 — Tenancy: owner-owned events + share tokens

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Two access classes (from ADR 0005): **owners** manage events, and
**attendees** browse them through a share-token URL. The tenancy
question is where events live in the data model relative to those
two classes, and how attendees reach an event without an account.
A wrong choice here forces schema surgery later.

## Decision Drivers

* Attendees must reach an event without signing in (ADR 0005
  share-token path).
* Multiple humans can co-own an event (ADR 0005 RBAC roles
  `owner`/`editor`/`reader`).
* One tenant boundary per event — cross-event data leaks are the
  worst-case failure mode.
* Row-level access checks must be cheap enough to run per
  request; per-request policy fetches from an external service
  are not on the table.

## Considered Options

* **Multi-tenant SaaS** with a top-level tenant/organisation
  entity above events. Overkill for a product where the natural
  unit is the event; adds a whole layer of admin UX we don't
  need.
* **Per-owner isolation** where each owner has their own logical
  namespace of events. Fights the ADR 0005 co-owner model —
  co-ownership becomes cross-namespace sharing.
* **Owner-managed events + share tokens** (chosen). Event is
  the tenant boundary; the `event_memberships` table (ADR 0005)
  attaches principals to events with a role; share tokens are
  a separate table scoped to an event with a revocable opaque
  identifier.

## Decision Outcome

Chosen: **event-as-tenant, owner-managed, attendee-share-token.**

* An **event** is the primary tenancy boundary. Every media,
  session, and search result carries an `event_id` that
  authorization checks in.
* Authenticated access flows through `event_memberships`
  (principal + event + role, per ADR 0005). The first
  principal to create the event gets the `owner` role.
* Unauthenticated attendee access flows through a `share_token`
  row (opaque token + `event_id` + revocation flag + optional
  expiry). Attendees do not appear in `event_memberships`.
* All row-level access checks in the API's `AuthzPolicy` port
  (ADR 0005) key off `(event_id, principal_or_token)`.

## Consequences

* Cross-event data leaks are a single `WHERE event_id = ?`
  omission away — the API's default query builder must inject
  `event_id` scoping, not rely on hand-written SQL.
* Revoking an attendee's access is a share-token flip, not a
  user-account operation.
* Migrating to a multi-tenant/organisation model later (if the
  product grows) means adding a new tenant table above events;
  the event-as-tenant contract still holds under it.
* Downstream: slice 5 lands the `events`, `event_memberships`,
  and `share_tokens` tables via init scripts; slice 6 (API)
  wires the event-scoping middleware and the share-token
  validator; slice 8 (web) implements the share-token URL UX.
