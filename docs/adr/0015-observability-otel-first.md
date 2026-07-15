# 0015 — Observability: OTel-first (logs / traces / metrics folded)

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

Three runtimes (Rust API, Python worker, Next.js web) will emit
telemetry. Without a shared vocabulary and pipeline, each service
picks its own logger + trace exporter + metric SDK, and debugging
a cross-service flow becomes read-three-consoles-and-guess. The
observability decision needs to land once, at the docs layer,
before slices 6/7/8 pick libraries.

The three pillars (logs, traces, metrics) share the same core
choice — which SDK+wire-format everyone speaks — so they fold
into one ADR with three `### Decision Outcome` sub-sections, per
the ADR 0006 precedent.

## Decision Drivers

* One wire format across three runtimes, so a collector can
  ingest all three without per-service transforms.
* Vendor-neutral: swapping backends (Grafana Cloud → Honeycomb
  → self-hosted) must not require code changes in the services.
* Trace context must propagate across service boundaries (API →
  worker via Redis Streams) so a request survives async hops.
* Every runtime must have a first-class OTel SDK we can pin.

## Considered Options

* **Split log/trace/metric stacks** (Loki + Tempo + Prometheus,
  each with a per-runtime shim). Rejected — three overlapping
  vocabularies + three propagation stories.
* **Vendor-locked** (Datadog / Honeycomb agent everywhere).
  Rejected — code-level lock-in cost outweighs the plug-and-
  play convenience.
* **OTel-first** (chosen).

## Decision Outcome

Chosen: **OpenTelemetry across all three pillars, in all three
runtimes.** Every service emits OTLP; an OTel collector
(deployed by slice 5) fans out to whatever backends we settle
on. The three pillars share this decision but differ in the
per-runtime detail below.

### Decision Outcome — Logs

Structured logs (JSON in prod, human-readable in dev, per
`docs/conventions/coding.md`) shipped through the OTel logs
signal. Log records carry the active trace + span IDs so a log
line can be jumped to its trace. Log level is a runtime
config, not a code constant.

### Decision Outcome — Traces

W3C `traceparent` header on every HTTP request; the same trace
context propagates onto Redis Stream messages as a payload
field (`trace_id` per `docs/conventions/events.md`). Spans
follow the `application/` handler boundary — one span per
handler, adapters open child spans.

### Decision Outcome — Metrics

OTel metrics API for counters (requests, errors, jobs), gauges
(queue depth, active sessions), and histograms (request
duration, search latency). Metric names follow OTel
conventions (`http.server.duration`, `messaging.publish.duration`
etc.) — no custom naming schemes.

## Consequences

* Every runtime pins its OTel SDK (opentelemetry-rust, opentelemetry-python,
  `@opentelemetry/api` + `@opentelemetry/sdk-*` for TypeScript).
  Concrete versions land at slice-6/7/8 code time.
* Compose stack (slice 5) runs an OTel collector service that
  every runtime points at via `OTEL_EXPORTER_OTLP_ENDPOINT`.
* Backend choice (Grafana Cloud / Honeycomb / self-hosted) is a
  collector-config decision, not an SDK decision. Deferred to
  ops-time.
* PII handling in telemetry: `docs/privacy.md` names log-lines
  and span attributes as high-risk surfaces. Adapters must
  scrub before emitting; convention lives in
  `docs/conventions/observability.md`.
* Downstream: slice 5 (`infra/compose/`) brings up the OTel
  collector; slice 6 (Rust API), slice 7 (Python worker), and
  slice 8 (Next.js web) each wire their SDK at boot and emit
  the three signals.
