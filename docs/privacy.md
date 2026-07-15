# Privacy posture

Consolidates the sensitive-content rules already committed in
`AGENTS.md` §2 and `CONTRIBUTING.md`. This is a **posture** doc,
not a policy doc — it restates existing commitments and
enumerates what the platform has **not** yet decided. Anything
under [`## Deferred (needs product input)`](#deferred-needs-product-input)
is intentionally not a promise.

## What is sensitive

The following user content is sensitive and treated as PII:

* **Faces** in uploaded event media (photos or video keyframes,
  per ADR [0007](adr/0007-media-scope-photo-and-video-keyframe.md)).
* **Selfie uploads** submitted by attendees for search (per
  ADR [0009](adr/0009-search-transport-cpu-onnx-inline.md)).
* **Share-token URLs** — an attendee's share-token URL is a
  credential; leaking it grants event access.

## Where sensitive content must not appear

Anchored to `AGENTS.md` §2 and `CONTRIBUTING.md`. Sensitive
content **must not** appear in:

* Git commit messages or commit content.
* PR bodies, PR comments, or review discussions.
* Issue bodies or issue comments.
* Telemetry — logs, spans, or metric labels (see
  [conventions/observability.md](conventions/observability.md)
  for the scrubbing rules).

This is a floor, not a ceiling. Anything else that could carry
sensitive content by accident should be reviewed the same way.

## Known leaky spots

Pre-existing hazards that downstream slices must design around:

* **Request paths carrying share tokens.** A URL like
  `/api/v1/events/E/share/TOKEN/photos` leaks the token into
  any log line that records the request path. Scrub before
  emitting.
* **Span attributes named `user_id` or `email`.** Adapters
  should carry account ids, not personal data, on span
  attributes.
* **Full URLs in log lines.** Same failure mode as request
  paths — redact the share-token segment to `<token>`.

## Deferred (needs product input)

These are commitments the platform has **not** made. Each will
land as a future ADR + `docs/privacy.md` edit with product
input.

* **Encryption at rest** for stored media and selfie uploads.
  Not committed here. Decision + storage backend land alongside
  a future storage ADR.
* **Log retention window.** How long telemetry backends keep
  scrubbed logs. Not committed here. Decision lands with
  ops-time backend selection (see ADR
  [0015](adr/0015-observability-otel-first.md)).
* **Deletion on request.** The mechanism (who can request,
  what gets deleted, verification flow) is not committed here.
  Slice 6 (API) will surface an endpoint stub when the ADR
  lands.
* **Deletion SLA.** How quickly a deletion request is honored.
  Not committed here — this is a product commitment, not an
  architecture one.
* **IP-log posture.** Whether request IPs are stored at all
  and, if so, for how long. Not committed here.
* **Cross-border data transfer.** Which regions the platform
  serves and where data may cross. Not committed here.

If a future slice needs to make one of these commitments, it
lands as its own ADR + an edit to this file — never a silent
in-line addition.
