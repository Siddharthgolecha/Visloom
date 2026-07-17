# Visloom architecture overview

Visloom lets an **owner** organize an event, upload its photos and videos,
and lets **attendees** find themselves in that media by uploading a selfie.
It runs as three cooperating runtimes on a single VPS: a **Rust API**, a
**Python worker**, and a **Next.js web** app, backed by Postgres (with
pgvector) and Redis. Media is **indexed asynchronously** on the worker's GPU
and **searched synchronously** inside the API on CPU — the two data-flows that
shape the whole system.

This page is a synthesized view. The [ADRs](../adr/README.md) and
`packages/contracts/schema.sql` are the **source of truth**; where a diagram
and an ADR disagree, the ADR wins. Each section links the ADR(s) it
visualizes. Diagram format is recorded in [ADR 0019](../adr/0019-architecture-diagrams-mermaid.md);
C4 levels are drawn with Mermaid flowcharts rather than the experimental
`C4*` types.

---

## System context

**C4 Level 1.** Two kinds of people use Visloom, and it depends on one
external system. Owners authenticate (Google OAuth or a password backup);
attendees are unauthenticated and reach an event only through an opaque,
revocable share-token URL.

```mermaid
flowchart TB
    owner["Owner<br/>(event organizer)"]
    attendee["Attendee<br/>(unauthenticated guest)"]
    google["Google OAuth<br/>(external identity provider)"]

    subgraph visloom["Visloom"]
        app["Event photo &amp; video discovery<br/>upload media · search by selfie"]
    end

    owner -->|"log in, create events,<br/>upload media"| app
    attendee -->|"open share-token URL,<br/>search with a selfie"| app
    app -->|"authenticate owner"| google
```

Actors and the auth split come from [ADR 0005](../adr/0005-owner-auth-and-rbac.md)
(owner auth + RBAC) and [ADR 0008](../adr/0008-tenancy-owner-events-and-share-tokens.md)
(tenancy + share tokens).

## Containers & deployment

**C4 Level 2.** All runtimes and infrastructure sit on one VPS under Docker
Compose. Caddy terminates TLS and fronts web + API. The API and worker never
call each other directly — they hand off over **Redis Streams**. Both write
Postgres and export telemetry to an OTel collector.

```mermaid
flowchart TB
    owner["Owner"]
    attendee["Attendee"]

    subgraph vps["Single VPS — Docker Compose"]
        caddy["Caddy<br/>(reverse proxy / TLS)"]
        web["apps/web<br/>Next.js"]
        api["services/api<br/>Rust · CPU-only"]
        worker["services/worker<br/>Python · CUDA + CPU fallback"]
        pg[("Postgres<br/>+ pgvector")]
        redis[("Redis<br/>streams · cache · rate-limit")]
        otel["OTel collector"]
    end

    owner --> caddy
    attendee --> caddy
    caddy --> web
    web --> api
    api -->|"jobs.media.index.v1"| redis
    redis -->|"consumer group"| worker
    worker -->|"events.media.indexed.v1 /<br/>events.media.index_failed.v1"| redis
    api --> pg
    worker --> pg
    api -.->|"traces · logs · metrics"| otel
    worker -.->|"traces · logs · metrics"| otel
```

Runtime split is [ADR 0003](../adr/0003-polyglot-monorepo.md); the layered +
hex structure is [ADR 0002](../adr/0002-layered-hexagonal-architecture.md);
the single-VPS Compose topology is [ADR 0004](../adr/0004-docker-compose-single-vps.md).
Redis's three roles are [ADR 0016](../adr/0016-redis-usage.md).

## Component — API (Rust)

**C4 Level 3.** The API follows the per-runtime baseline of `domain /
application / adapters / telemetry`. Writes with external effects go through
hex **ports** (traits) that adapters implement; reads stay layered. Search
embeds the selfie **in-process** on CPU ONNX — there is no worker hop on the
read path.

```mermaid
flowchart TB
    subgraph api["services/api — Rust (CPU-only)"]
        http["adapters/ · HTTP handlers<br/>(/api/v1/*)"]
        search["application/ · search handler<br/>(read, synchronous)"]
        index["application/ · index-request handler<br/>(write, publishes a job)"]
        domainmod["domain/ · events · media ·<br/>memberships · share tokens"]
        onnx["adapters/ · CPU ONNX<br/>(SelfieEmbedder port impl)"]
        authp["adapters/ · AuthProvider impls<br/>(GoogleOAuth · Password · Noop)"]
        producer["adapters/ · Redis Streams producer"]
        tel["telemetry/ · OTel spans"]
    end

    http --> search
    http --> index
    http --> authp
    search -->|"SelfieEmbedder port"| onnx
    search --> domainmod
    index --> producer
    api -.-> tel
```

Layering + ports are [ADR 0002](../adr/0002-layered-hexagonal-architecture.md);
inline CPU-ONNX search is [ADR 0009](../adr/0009-search-transport-cpu-onnx-inline.md);
the API's CPU-only inference is [ADR 0010](../adr/0010-inference-runtime-worker-cuda-api-cpu.md);
the `NoopAuthProvider` wiring is [ADR 0013](../adr/0013-noop-auth-provider.md).

## Component — Worker (Python)

**C4 Level 3.** The worker reads jobs from a Redis **consumer group**
(at-least-once, `XACK` on completion) and runs the index pipeline. Video is
reduced to keyframes first; both photo and video frames flow through the same
embedder, which prefers CUDA and falls back to CPU at boot.

```mermaid
flowchart TB
    subgraph worker["services/worker — Python (CUDA + CPU fallback)"]
        consumer["adapters/ · Redis Streams consumer<br/>(consumer group · XACK)"]
        pipeline["application/ · index pipeline<br/>detect → embed → store"]
        keyframe["adapters/ · keyframe extractor<br/>(video → N frames)"]
        embedder["adapters/ · embedder<br/>(CUDA-preferred ONNX, CPU fallback)"]
        pgw["adapters/ · pgvector writer"]
        domainw["domain/ · MediaKind {photo, video}<br/>· MediaFrame"]
        telw["telemetry/ · OTel spans<br/>(traceparent child)"]
    end

    consumer --> pipeline
    pipeline --> keyframe
    pipeline --> embedder
    pipeline --> pgw
    pipeline --> domainw
    worker -.-> telw
```

Layering is [ADR 0002](../adr/0002-layered-hexagonal-architecture.md); the
photo + video-keyframe media model is [ADR 0007](../adr/0007-media-scope-photo-and-video-keyframe.md);
the CUDA + CPU-fallback runtime is [ADR 0010](../adr/0010-inference-runtime-worker-cuda-api-cpu.md).

## Indexing flow (sequence)

The write path is **asynchronous**. The API persists the media row and
publishes `jobs.media.index.v1`; the worker consumes it, embeds, and replies
on one of two result streams. The trace context (`traceparent`) rides inside
the event envelope so the worker's spans attach as children of the API's.

```mermaid
sequenceDiagram
    actor Owner
    participant API as services/api (Rust)
    participant Redis as Redis Streams
    participant Worker as services/worker (Python)
    participant PG as Postgres + pgvector

    Owner->>API: upload media
    API->>PG: insert media row
    API->>Redis: XADD jobs.media.index.v1<br/>envelope: event_id, traceparent, trace_id, occurred_at, data
    Note over API,Worker: traceparent propagates the trace —<br/>worker spans are children of the API span (ADR 0015)
    Redis->>Worker: consumer-group read
    Worker->>Worker: detect faces → embed<br/>(video: keyframes → N frames)
    alt indexed
        Worker->>PG: write embeddings (pgvector)
        Worker->>Redis: XADD events.media.indexed.v1<br/>data: media_id, embedding_ref, frames,<br/>embedder_model_id, embedder_version
    else failed
        Worker->>Redis: XADD events.media.index_failed.v1<br/>data: media_id, failure {code, message},<br/>retry {attempt, next_at}
    end
    Worker->>Redis: XACK the job
    Redis->>API: consumer-group read (result)
```

Transport + the three canonical stream names are
[ADR 0006](../adr/0006-redis-streams-versioned-naming.md) and
`../conventions/events.md`; the media model is
[ADR 0007](../adr/0007-media-scope-photo-and-video-keyframe.md); trace
propagation through the envelope is
[ADR 0015](../adr/0015-observability-otel-first.md).

## Search flow (sequence)

The read path is **synchronous** and stays inside the API, targeting under
two seconds. The attendee's share token resolves to an event, the selfie is
embedded in-process on CPU ONNX (~200 ms), and a pgvector nearest-neighbour
query returns the matches. No worker hop, no event schema.

```mermaid
sequenceDiagram
    actor Attendee
    participant API as services/api (Rust)
    participant ONNX as CPU ONNX (in-process)
    participant PG as Postgres + pgvector

    Attendee->>API: POST selfie (via share-token URL)
    API->>API: AuthzPolicy check<br/>(share token → event_id)
    API->>ONNX: embed selfie inline (~200 ms)
    ONNX-->>API: embedding vector
    API->>PG: pgvector nearest-neighbour query (scoped to event_id)
    PG-->>API: matching media
    API-->>Attendee: results (target < 2 s)
```

Inline CPU-ONNX search transport is
[ADR 0009](../adr/0009-search-transport-cpu-onnx-inline.md); the share-token
authorization boundary is
[ADR 0008](../adr/0008-tenancy-owner-events-and-share-tokens.md).

## Data model (ER)

The tenant boundary is the **event**: media, memberships, and share tokens
all hang off `events`, and every membership and event ties back to an
`account`. This mirrors `packages/contracts/schema.sql` exactly — the seven
tables present today. Embeddings, pgvector columns, and per-frame rows are
**not** shown: they arrive with the slice-5 migration and are out of scope
for this reference.

```mermaid
erDiagram
    accounts ||--o{ sessions : "has"
    accounts ||--o{ events : "owns"
    accounts ||--o{ event_memberships : "member of"
    events ||--o{ event_memberships : "grants"
    events ||--o{ share_tokens : "exposed by"
    events ||--o{ media : "contains"
    accounts ||--o{ idempotency_keys : "issued by"

    accounts {
        text account_id PK
        text email UK
        text display_name
        timestamptz created_at
    }
    sessions {
        text session_token PK
        text account_id FK
        timestamptz created_at
        timestamptz expires_at
    }
    events {
        text event_id PK
        text owner_account_id FK
        text title
        timestamptz created_at
    }
    event_memberships {
        text event_id PK
        text account_id PK
        text role "owner | editor | reader"
        timestamptz created_at
    }
    share_tokens {
        text share_token PK
        text event_id FK
        boolean revoked
        timestamptz expires_at
        timestamptz created_at
    }
    media {
        text media_id PK
        text event_id FK
        text media_kind "photo | video"
        text source_uri
        timestamptz created_at
    }
    idempotency_keys {
        text idempotency_key PK
        text account_id PK
        text request_fingerprint
        jsonb response_body
        integer response_status
        timestamptz created_at
    }
```

Tables and the `role` / `media_kind` enums are
`packages/contracts/schema.sql`; the account/session model is
[ADR 0005](../adr/0005-owner-auth-and-rbac.md); events, memberships, and share
tokens are [ADR 0008](../adr/0008-tenancy-owner-events-and-share-tokens.md);
`media_kind` is [ADR 0007](../adr/0007-media-scope-photo-and-video-keyframe.md).
