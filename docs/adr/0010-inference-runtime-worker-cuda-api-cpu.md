# 0010 — Inference runtime: Worker CUDA+CPU / API CPU-only

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Visloom runs two inference paths (see ADR 0009): the **worker**
batch-embeds indexed media (throughput-bound), and the **API**
embeds one selfie per search request (latency-bound). Picking
one runtime for both trades off wrong on at least one path.
Locking the split now lets slice 6/7's Dockerfiles + boot paths
build against a known contract.

## Decision Drivers

* Worker throughput is proportional to GPU utilisation on large
  event backlogs — CUDA is required to keep indexing latency
  bounded when an event drops thousands of frames at once.
* Development environments (contributor laptops, CI runners) do
  not always have a GPU. Worker must still boot and run in a
  CPU-only environment for tests.
* API's inference is a single forward pass per search — CPU is
  fast enough (ADR 0009) and keeps the API image small.
* One shared runtime image everywhere would either (a) force the
  API to pull ~2 GB of CUDA it never uses, or (b) starve the
  worker of GPU access.

## Considered Options

* **Single runtime, CPU-everywhere.** API is fine; worker
  indexing latency explodes on real event backlogs.
* **Single runtime, CUDA-everywhere.** API image bloats to ~2 GB
  for a single forward pass it doesn't need CUDA for. Worse
  cold-start.
* **Split: worker CUDA-with-CPU-fallback / API CPU-only**
  (chosen).

## Decision Outcome

Chosen: **split runtime.**

* **Worker (slice 7):** ONNX Runtime with CUDA execution
  provider preferred, CPU provider as fallback. Selection
  happens at boot: try CUDA, fall back on init failure, log the
  chosen provider (surfaced via `/healthz` per
  `docs/conventions/api.md` — worker exposes an equivalent
  health endpoint). Dockerfile base is
  `nvidia/cuda:*-runtime-ubuntu*` with `onnxruntime-gpu`.
* **API (slice 6):** ONNX Runtime with CPU execution provider
  only. Dockerfile base is `debian:*-slim` (or equivalent) with
  `onnxruntime` (CPU wheel).

Model files themselves are the same across both — the runtime
provider differs.

## Consequences

* Two Dockerfiles, two base images. Accepted for the
  latency/throughput split.
* CI can run worker on CPU-only runners (falls back to CPU
  provider); production nodes with a GPU get CUDA.
* Model updates need to be tested on both providers — one
  regression can appear on GPU that doesn't on CPU.
* Downstream: slice 6 (Rust API) uses the CPU-only image
  contract; slice 7 (Python worker) uses the CUDA/CPU-fallback
  image and the boot-time provider selection logic.
