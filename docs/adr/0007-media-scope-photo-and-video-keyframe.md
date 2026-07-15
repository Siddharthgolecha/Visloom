# 0007 — Media scope: photo + video-keyframe day-1

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

The `README.md` frames Visloom as "AI-powered event photo **and
video** discovery." Locking a narrow media scope now (photos only)
means video support later becomes a schema + pipeline rewrite;
locking too wide (arbitrary media types) means the worker's
embedding pipeline has to handle formats it has no ML story for.
Slice 7 (Python worker) needs a media abstraction it can build
against without rework.

## Decision Drivers

* Product spec commits to both photos and video from day one.
* Same embedding pipeline (face detection → CLIP-family embedding
  → pgvector) applies to photos and to keyframes extracted from
  video.
* Formats without a face/embedding story (audio, PDF) are out
  of scope for the search product.
* Adding a media *type* later must not require touching the
  event/session/share-token schema.

## Considered Options

* **Photo-only.** Simplest; excludes wedding-video and highlight-
  reel use cases the product already targets.
* **Photo + video-keyframe.** One media abstraction, two
  ingestion adapters: still-frame passes through to the embedder,
  video passes through a keyframe-extractor first.
* **Arbitrary media (photo, video, audio, PDF).** No ML pipeline
  for audio/PDF; adds schema surface for zero product value.

## Decision Outcome

Chosen: **photo + video-keyframe.** `domain/media` treats every
uploaded artifact as a `Media` value with a `MediaKind ∈ {photo,
video}` discriminator. Video ingestion runs a keyframe extractor
adapter (per-second or per-scene, decided at slice-7 code time)
that emits one or more `MediaFrame` values into the same embedder
path photos use. The event/session/share-token schema is
media-agnostic.

## Consequences

* Adding a third media *type* later means a new
  `MediaKind` variant + a new extractor adapter — no
  event-schema change.
* The worker (slice 7) needs a video-keyframe extractor stub
  before any real ingestion path is wired.
* Storage size for video is 1-N frames per source, not one blob
  — deferred to slice 5 (compose) to decide how the object store
  is laid out.
* Downstream: slice 7 lands `MediaKind` + extractor stubs; slice
  3 lands the media contract schemas (`packages/contracts/`); the
  `jobs.media.index.v1` stream (ADR 0006) already uses the
  media-abstract wording.
