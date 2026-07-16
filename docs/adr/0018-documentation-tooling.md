# 0018 — Documentation tooling: MyST-Sphinx / rustdoc / TypeDoc

* Status: Accepted
* Date: 2026-07-16
* Deciders: @Siddharthgolecha, @MSpider3

## Context and Problem Statement

Every runtime has a preferred code-doc tool. Picking one
cross-language doc-tool would compromise at least one runtime;
picking three native tools multiplies build surface. The
question is which per-language tool each runtime uses, and
whether the Python side speaks the same Markdown the rest of
the docs tree uses.

## Decision Drivers

* Rust and TypeScript already have widely-used native
  toolchains — reaching for a foreign tool loses ecosystem
  integration.
* Python's default (RST) is a different markup from the rest
  of the docs tree (Markdown under `docs/`) — a mismatch
  contributors have to context-switch through.
* Type-annotated docstrings must render with types visible
  (i.e., `sphinx-autodoc-typehints` or equivalent).

## Considered Options

* **MkDocs single-tool across all three.** Rejected — MkDocs
  can't ingest type-annotated Python docstrings the way
  Sphinx autodoc does, and it isn't native to Rust or TS.
* **pdoc for Python.** Rejected — no cross-reference story
  with the docs tree.
* **Raw RST + Sphinx for Python** (native default). Splits
  the docstring markup from the rest of `docs/`.
* **MyST-Parser + Sphinx + Google-style docstrings for
  Python; rustdoc for Rust; TSDoc + TypeDoc for TypeScript**
  (chosen).

## Decision Outcome

Chosen: **per-language native tooling with MyST for Python.**

* **Python (worker, slice 7):** Sphinx with the `myst-parser`
  extension so docstrings are Markdown, matching the rest of
  the docs tree. Docstrings use **Google-style** formatting
  (napoleon). Type hints render via `sphinx-autodoc-typehints`.
* **Rust (API, slice 6):** `rustdoc`. Crate-level docs use
  `//!`; item-level docs use `///`; cross-refs via
  `[SomeType]` intra-doc links.
* **TypeScript (web, slice 8):** TSDoc syntax + TypeDoc as
  the generator. `@public` / `@internal` tags gate the
  generated API surface.

All three produce HTML that can be published alongside the
main `docs/` tree; the publish target is deferred (slice-9
scope).

## Consequences

* Contributor cognitive load stays low: the same Markdown
  they write in `docs/` is what they write in Python
  docstrings.
* Three tools to configure — one per runtime. Config files
  land in slices 6/7/8 (Rust: default, no config needed;
  Python: `docs/conf.py` at slice-7 time; TS:
  `typedoc.json` at slice-8 time).
* **Supersession path:** if MyST-Parser proves unstable or
  its ecosystem stalls, replacing it is a one-extension
  removal from `conf.py` + docstring re-format. This is a
  reversible commitment. `docs/adr/README.md`'s status
  vocabulary allows an ADR to be marked
  `Superseded by NNNN`.
* Downstream: slice 6 (Rust API) sets up rustdoc; slice 7
  (Python worker) sets up Sphinx + MyST + Google-style +
  autodoc-typehints; slice 8 (Next.js web) sets up TypeDoc.
