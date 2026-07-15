# Observability conventions

Cross-runtime shape for logs, traces, and metrics. Anchored by
ADR [0015](../adr/0015-observability-otel-first.md).

## Signals

Every service emits three OTel signals:

* **Logs** — structured (JSON in prod, human-readable in dev
  per [coding.md](coding.md)). Each record carries the active
  `trace_id` and `span_id` so a log line is jumpable to its
  trace.
* **Traces** — one span per `application/` handler; adapters
  open child spans for their external calls (DB, cache, HTTP,
  ONNX). Cross-service context propagates via W3C
  `traceparent` header on HTTP and via the `trace_id` payload
  field on event streams (see [events.md](events.md)).
* **Metrics** — counters (requests, errors, jobs), gauges
  (queue depth, active sessions), histograms (request
  duration, search latency). Names follow OTel semconv
  (`http.server.duration`, `messaging.publish.duration`).

## SDKs per language

| Runtime | SDK |
|---|---|
| Rust (API, slice 6) | `opentelemetry` + `opentelemetry-otlp` |
| Python (worker, slice 7) | `opentelemetry-api` + `opentelemetry-sdk` + `opentelemetry-exporter-otlp` |
| TypeScript (web, slice 8) | `@opentelemetry/api` + `@opentelemetry/sdk-node` + `@opentelemetry/exporter-trace-otlp-http` |

Concrete versions pin at slice-6/7/8 code time.

## PII in telemetry

`docs/privacy.md` names log lines and span attributes as
high-risk PII surfaces. Adapters scrub before emitting:

* No share tokens in log lines. If a URL is logged, the
  share-token segment is redacted to `<token>`.
* Span attributes named `user_id`, `email`, or similar must
  carry account ids, not personal data.
* Selfie uploads never appear in telemetry — not as bytes, not
  as base64, not as filenames.

## Collector

Compose (slice 5) runs one OTel collector service. Every runtime
sets `OTEL_EXPORTER_OTLP_ENDPOINT` to it. Backend fan-out
(Grafana / Honeycomb / self-hosted) is a collector-config knob,
not an SDK decision.
