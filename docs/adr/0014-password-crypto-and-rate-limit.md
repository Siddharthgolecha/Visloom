# 0014 — Password crypto + rate limit + recovery

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

ADR 0005 committed the password-backup path for owners and
flagged credential hashing, rate-limiting, and account recovery
as follow-up ADR work (`docs/adr/0005-owner-auth-and-rbac.md:106-108`).
This ADR records the algorithm and policy choices; the concrete
parameter tuning (memory cost, parallelism, exact backoff
constants) is code-time work in slice 6.

## Decision Drivers

* Passwords are user-provided; the storage side must be
  hardened against offline brute-force after any leak.
* Rate-limits must survive the "credential-stuffing script"
  failure mode — per-account throttling alone leaves the
  service open to distributed guessing across accounts.
* Recovery must not rely on security-question-style secrets
  (well-known to be weaker than the password itself).
* Owner UX is not attendee UX: password reset can be a
  once-in-a-blue-moon email round-trip, not an in-app flow.

## Considered Options

For hashing:

* **bcrypt.** Widely deployed, well-audited, but memory-hard
  properties are weaker than argon2id; OWASP now lists
  bcrypt as acceptable-not-preferred.
* **scrypt.** Memory-hard, less commonly deployed, fewer
  vetted implementations per language.
* **argon2id** (chosen). Current OWASP recommendation;
  well-audited implementations exist for every language.

For rate-limiting:

* **Per-account only.** Fails against credential stuffing.
* **Per-IP only.** Fails against distributed attacks + hurts
  legitimate users behind NAT.
* **Per-account + per-IP exponential backoff** (chosen).

For recovery:

* **Security questions.** Rejected — weaker than the
  password itself.
* **SMS OTP.** Rejected — SIM-swap surface, cost per SMS.
* **Magic-link email** (chosen). Email itself is the trust
  root; if attacker owns the email account, they already own
  the OAuth path in ADR 0005.

## Decision Outcome

Chosen: **argon2id + dual rate-limit + magic-link recovery.**

* **Hashing:** argon2id. Parameters (memory cost `m`, time
  cost `t`, parallelism `p`) chosen at slice-6 code time based
  on the API's target latency budget; recorded in the code, not
  here.
* **Rate limiting:** exponential backoff **both** per-account
  and per-IP. Failed-login counter resets on successful
  authentication. Lockout after N consecutive failures triggers
  an email notification to the account owner.
* **Recovery:** magic-link over email. Link is single-use,
  expires within a short window (concrete number chosen at
  slice-6 code time). Successful recovery invalidates all
  existing sessions for the account.

## Consequences

* API image ships an argon2id implementation (Rust crate
  `argon2` or equivalent).
* Rate-limit state is short-lived; ADR 0016 (Redis usage)
  covers the storage location and TTL.
* Recovery requires an outbound-email adapter — a new port on
  the API side, landing in slice 6.
* Parameter tuning is a code-time knob, not a doc decision:
  slice 6's implementation notes record the chosen values.
* Downstream: slice 6 (Rust API) implements argon2id hashing,
  the dual-scope rate limiter (backed by Redis per ADR 0016),
  the magic-link flow, and the outbound-email port.
