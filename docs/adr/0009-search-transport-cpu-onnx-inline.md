# 0009 — Search transport: API embeds selfie inline via CPU ONNX

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Attendees search by uploading a selfie: the API must produce an
embedding for that selfie and run a pgvector nearest-neighbour
query against pre-indexed event media. The transport question is
*who embeds the selfie*: the API itself (inline, one hop), or the
worker via Redis Streams (async, two hops + wait). Search
latency is the user-visible metric.

## Decision Drivers

* Attendee search must feel synchronous — target < 2 s from
  selfie upload to result list.
* API runs on the single VPS (ADR 0004) with CPU-only inference
  (parent fork table row: "API: CPU-only ONNX"). Embedding one
  selfie on CPU is bounded (~200 ms with a small CLIP-family
  model).
* Worker's CUDA path is optimised for batch indexing, not
  per-request latency.
* No cross-service ML plumbing to maintain if search stays in
  the API.

## Considered Options

* **Worker-embedded search.** API pushes a `jobs.search.v1`
  message onto Redis Streams; worker embeds; API waits on a
  correlation-id reply. Extra Redis hops on the critical path.
* **API embeds inline via CPU ONNX** (chosen). API loads a
  CPU-friendly ONNX embedding model at boot and runs a single
  forward pass per search request. No cross-service call.
* **External inference service** (e.g. Triton, a bespoke gRPC
  service). Extra deployment surface for one model on one CPU;
  the ADR 0004 single-VPS shape doesn't earn it.

## Decision Outcome

Chosen: **inline CPU ONNX in the API.** The API's `application/`
search handler calls a `SelfieEmbedder` port (per ADR 0002); the
adapter loads a CPU ONNX model (specific model + version chosen
at slice-6 code time, and reported by `/healthz` per
`docs/conventions/api.md`). The embedding runs synchronously in
the same request; the pgvector query follows immediately.

## Consequences

* Search latency is bounded by CPU embedding + pgvector query.
  Both are on-VPS and measurable.
* The API's Dockerfile carries an ONNX runtime dependency
  (CPU wheel; no CUDA in the API image, keeping the image small
  per parent fork "API: CPU-only").
* Worker's CUDA path stays batch-only — cleaner separation of
  concerns.
* If a model change requires GPU-only inference, this ADR
  supersedes and we revisit worker-embedded search.
* Downstream: slice 6 (Rust API) creates the `SelfieEmbedder`
  port and loads the CPU ONNX adapter; slice 3 (contracts) does
  **not** need a search-transport schema — the search path is
  intra-API.
