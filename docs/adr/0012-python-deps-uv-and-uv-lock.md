# 0012 — Python deps: `uv` + `uv.lock`

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha

## Context and Problem Statement

The worker (slice 7) is Python. The Python packaging ecosystem
has multiple tools — pip + pip-tools, Poetry, PDM, `uv` — each
with different install speeds, lockfile formats, and Docker
ergonomics. Picking one now avoids re-tooling later and lets
slice 7's Dockerfile and CI cache against a known lockfile
shape.

## Decision Drivers

* Deterministic builds: a lockfile that pins every transitive.
* Fast local installs — contributor onboarding shouldn't wait
  minutes for a `pip install`.
* Docker-friendly: a stage that runs
  `<tool> sync --frozen --no-dev` in the build image and copies
  the resulting environment into the runtime image.
* Compatible with `pyproject.toml` (PEP 621) — no custom
  proprietary format.

## Considered Options

* **Poetry.** Mature, `pyproject.toml` native, but historically
  slow installs and an opinionated resolver.
* **pip + pip-tools.** Minimal, well-understood, but `pip
  install` speed is the floor of the ecosystem and `pip-compile`
  is a separate tool with its own lockfile format.
* **PDM.** PEP 621 native, standard resolver, but smaller
  community footprint than the others.
* **`uv`** (chosen). Rust-based, ~10-100× faster installs than
  pip, `pyproject.toml` native, `uv.lock` is a documented
  format.

## Decision Outcome

Chosen: **`uv` + `uv.lock`.**

* `services/worker/pyproject.toml` declares deps (PEP 621).
* `services/worker/uv.lock` is committed, pinned to
  reproducible transitive versions.
* Slice 7's `Dockerfile` uses a `uv sync --frozen --no-dev`
  step in the build stage.
* Contributor workflow: `uv sync` locally to install; `uv add
  <pkg>` to add a dep (which updates `uv.lock`).

## Consequences

* Contributors need `uv` installed once (documented in slice 9's
  bootstrap script). Any Python version `uv` supports works.
* CI caches the `uv` global cache directory between runs —
  install times measured in seconds, not minutes.
* If `uv` becomes unmaintained (low risk given current
  velocity), migration to another PEP 621 tool is a one-time
  cost — `pyproject.toml` stays.
* Downstream: slice 7 lands `pyproject.toml` + `uv.lock` + the
  Dockerfile using `uv sync --frozen --no-dev`.
