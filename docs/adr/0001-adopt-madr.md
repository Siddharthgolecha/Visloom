# 0001 — Adopt MADR for architecture decisions

* Status: Accepted
* Date: 2026-07-15
* Deciders: @Siddharthgolecha

## Context and Problem Statement

Greenfield repo, several locked forks about to land
(`.tasks/epics/arch-scaffold/parent.md:21-35`). Free-form prose
loses alternatives; PR bodies vanish on merge. Need a uniform
in-tree format before fork 1.

## Decision Drivers

* Alternatives must be explicit (`AGENTS.md:37-38`).
* Under two minutes to scan.
* Supersession without renumbering.

## Considered Options

* **MADR full** — 5 sections incl. `Considered Options`.
* **MADR short** — 3 sections, drops alternatives.
* **Nygard** — free-form, no fixed headers.

## Decision Outcome

Chosen: **MADR full**. The five-section shape forces alternatives
into the document, matching the adversarial-re-read rule
(`AGENTS.md:41-45`). Short drops the section that catches
alternative-free ADRs; Nygard is too loose for a review gate keyed
off document structure.

## Consequences

* Uniform, greppable headers; supersession via the `Status` line.
* `Considered Options` cannot be silently omitted.
* ~30 LOC of ceremony per decision — accepted tax.
* Slice 2 (ADRs 0007–0018) exercises this template at volume.
