# Visloom Agent Guide

This file applies to the whole Visloom repository. It is tool-agnostic —
Codex, Cursor, Aider, autodev, and recent Claude Code all read `AGENTS.md`.

Visloom has two lanes for agent-driven work:

- **Quick lane** — direct PR off `main`, no `.tasks/<id>/`, no `spec.md`.
- **Spec lane** — draft PR + `.tasks/<id>/{spec,plan,implementation}.md`
  with a `plan-approved` gate before implementation.

Lane decision is §4. The Optimize lane (skill evolutionary loop) is
deferred — see §8.

---

## 1. Disposition

*The best code is the code never written, and the best claim is the one
that survives an adversarial pass.*

**Before writing code, climb the ladder:** does this need to exist →
already in this codebase (helper / util / module) → stdlib → native
platform feature → installed dep → one-liner → minimum code that works.

**Solo ceiling.** Your solo output reaches "tested against real input"
but cannot reach "peer-reviewed engineering." The PR review is what
crosses that ceiling.

---

## 2. Rules

### Process rules — non-negotiable

- **Anchored claims.** Every assertion in `spec.md`, `plan.md`,
  `implementation.md`, or a commit message traces to a `file:line`,
  citation, or runnable check. "I think X handles Y" is forbidden;
  "`src/foo.ts:142` handles Y, verified by `tests/foo.test.ts::test_y`"
  is the bar.
- **Trust tiers on implementation entries.** Every `implementation.md`
  entry is marked `tested-against-real-input` / `tested-with-fixture` /
  `untested-assumption` / `inherited-from-existing-code`. "Ship it,
  looks good" is forbidden.
- **Append-only history.** Never delete past `implementation.md`
  entries; strikethrough if reversed. The history *is* the audit.
- **Adversarial re-read.** After any artifact, re-read it as a hostile
  reviewer. `spec.md` `## Failure modes` is where fragility surfaces —
  write what's fragile, not "looks fine."
- **Preserve disagreement.** When you and the reviewer disagree,
  record it in `## Deviations` or a follow-up Open Question — don't
  silently capitulate. See §3 on pushback.
- **Privacy by default.** Sensitive user content (private photos,
  personal data, faces) never leaks into telemetry, commits, or PR
  comments. If unsure, ask before referencing.
- **No plausibility-shipping.** If you claim a fix works, run it. If
  you claim code handles edge case X, test X.

### Never-do list

- **Never modify other contributors' PRs.**
- **Never force-push shared branches.** `task_finish.sh` uses
  `--force-with-lease` and only when a rebase changed history.
- **Never bypass the lane-confirmation gate.** `task_start.sh` refuses
  when the issue lacks `lane:considered`. Do not add the label
  yourself and rerun — the human owns that flip.
- **Never proceed past `plan-approved` without the label** (§5).
- **Never bypass hooks** (`--no-verify`, `--no-gpg-sign`) unless the
  user explicitly asks.

### Craft principles

- Deletion over addition; boring over clever; fewest files; shortest
  working diff.
- Bug fixes hit root cause across all callers (`grep` first, fix once).
- Non-trivial logic leaves one runnable check behind (assert-based or
  one small test file — no frameworks unless the stack already has
  one). Trivial one-liners need none.
- Intentional shortcuts with known ceilings go in `implementation.md`
  `## Deviations` naming the ceiling and upgrade path — not as inline
  code comments.

---

## 3. Adversarial dialogue

Your job is to challenge, not just produce.

- **Ask** at decision points. `spec.md` `## Open Questions for the
  Human` holds ≤5 high-leverage ones — write these BEFORE re-reading
  the ticket, so the codebase's framing dominates.
- **Push back** on rejections whose reasoning is weak: one round of
  counter-argument before revising. A rejection accompanied by
  reasoning you dispute counts as one round of debate, not a settled
  rejection — only counts toward the 3-rejection cap once the reviewer
  confirms or addresses your counter-argument.
- **Stop and ask** at implementation forks you can't resolve from
  context, instead of guessing.
- **Challenge the framing** when the task description has a flawed
  premise — flag it in `## Open Questions`, don't silently absorb it.

---

## 4. Lane decision

When ambiguous, pick Spec. Cost of a spec you didn't need is small;
cost of a Quick PR that turns out to need one is bigger.

**Quick fits when ALL are true:** bounded (≤3 files nameable up front);
no new dep / script / public API / label / schema; no behavior change
worth documenting; every PR-body claim is a single `file:line` cite;
reviewer decides in one read.

Typical Quick work: typos, `.md` clarifications, a one-file bugfix
restoring documented behavior, a dep bump inside its declared compat
range.

**Spec fits when ANY are true:** new module / script / evidence rule /
public interface; a public interface changes (CLI flag, output schema,
shared type, API contract); the change touches the [overlap list in
§6](#6-overlap-list); a design decision needs recording; est. diff
> ~200 LOC code or > ~400 LOC total; you have `## Open Questions`
before writing code.

**Escalation mid-flight.** If a Quick PR turns out to be Spec-sized,
stop, comment on the PR, close it, and restart via `task_propose.sh`.
Never inflate a Quick PR into Spec in place — the missing `spec.md`
history is exactly what Spec lane exists to produce. Going the other
way (mid-Spec discovering it's Quick-sized) is fine: land the scaffold
as-is with unused sections marked "N/A" and merge.

**Epics — when work decomposes into independent slices.** If a piece
of work naturally breaks into several **independently reviewable
subtasks** (e.g. an architecture scaffold that spans docs + contracts
+ three runtimes; a migration where each call-site is its own PR),
split it into a tracking **parent issue** + one **child issue per
slice**. Each slice is its own Spec-lane PR with its own `spec.md`,
`plan.md`, and `plan-approved` gate — the parent is just a live
checklist.

The trigger is **decomposability, not size**. A 600-LOC atomic
refactor that must land as one unit stays one PR (with the mid-flight
~400 LOC checkpoint in §5 step 7 for reviewer sanity-check). A
200-LOC change split across 8 unrelated files may still be an epic if
each file's change reviews on its own. When justifying an epic, name
the independent subtasks and why each stands alone — not the LOC
total. If every slice hard-depends on the previous, the work isn't
actually decomposable and shouldn't be an epic.

Use:

```
parent=$(tools/autodev/task_propose.sh "Epic: <title>" "<why>" \
           | jq -r .issue_number)
tools/autodev/task_propose.sh "<slice-1-title>" "<slice-1-body>" \
    --parent "$parent"
# … one per slice, in landing order
```

The `--parent <N>` flag prepends `Parent: #<N>` to the child body and
appends a `- [ ] #<child> <title>` line to the parent's issue body so
the parent stays a live checklist. Slice-level `plan.md` names its
predecessor slice in `## Depends on` — that's how ordering is
enforced, not by branch protection.

---

## 5. Lifecycles

Every step's `gh` call lives in the wrapper scripts under
[`tools/autodev/`](tools/autodev/) — use them. For anything the
wrappers don't cover: read labels with `gh pr view --json labels --jq
'.labels[].name'`; edit a PR body with `gh pr edit <pr> --body-file
<file>`; check for overlap with `gh pr list --search 'is:draft'`.
Never build URLs by hand; never `curl` the API.

### Quick lane

1. `tools/autodev/quick_start.sh fix|feat|docs "<slug>"` — branch off
   `main`.
2. Make the change; run whatever tests exist for the touched area.
3. `tools/autodev/quick_finish.sh -m "<subject>"` — push, open a
   non-draft PR with labels `lane:quick` + `agent-driven`. Body follows
   the Quick section of
   [`.github/PULL_REQUEST_TEMPLATE.md`](.github/PULL_REQUEST_TEMPLATE.md):
   problem / change / tested.
4. Reviewer merges (or asks for revisions with reasoning — §3
   applies).

No task dir, no gate. The PR description is the whole audit.

### Spec lane

1. **Propose.** `tools/autodev/task_propose.sh "<title>" "<request>"`
   opens the issue with `lane-pending` + `agent-driven`, posts the
   agent's lane-rationale comment (`lane:considered — <why>`), exits.
   No branch, no PR yet.
2. **Lane-confirmation gate (human).** Human flips `lane-pending` →
   `lane:considered` on the issue. If they disagree, they close it and
   the agent restarts elsewhere.
3. **Start.** `tools/autodev/task_start.sh <issue-number>` refuses
   unless `lane:considered` is set (exit 2), then creates `spec/<id>`
   off `main`, scaffolds `.tasks/<id>/` from
   [`.tasks/_template/`](.tasks/_template/), reads the issue's `##
   Task statement` into `spec.md` `## Context`, and opens the draft PR
   with `Closes #<N>` and labels `lane:considered`, `lane-pending`,
   `agent-driven`. Metadata lands in `.tasks/<id>/meta.env`.
4. **Fill spec + plan.** Complete `spec.md` first, then `plan.md` (see
   §7 templates). Commit, push, request review.
5. **Plan review.** Human answers your `## Open Questions` first, then
   reviews `spec.md`, then `plan.md`. Revise across commits. Approval
   is the `plan-approved` label. After 3 rejected revisions, stop and
   ask the human to take over, reformulate, or close.
6. **Poll for plan-approval.**
   `tools/autodev/poll_plan_approved.sh .tasks/<id>` blocks until the
   label appears.
7. **Implementation — plan-frozen rule.** Once `plan-approved` is on
   the PR, `spec.md` and `plan.md` are the contract the reviewer
   approved: do not edit them in this PR. Non-structural deviations go
   in `implementation.md` `## Deviations` citing the affected
   spec/plan section. **Structural** deviations — changed acceptance
   criteria, new dependencies, contradicted approach, scope creep past
   `## Files touched` — HALT for re-approval: ask the human to remove
   `plan-approved`, revise, wait for the label to return. Pause at
   ~400 LOC growth to sanity-check with the reviewer.
8. **Ready.** `tools/autodev/task_finish.sh .tasks/<id> -m "<msg>"`
   refuses without `plan-approved`, then rebases onto `main`, replaces
   the PR body's Implementation-summary section with
   `implementation.md` `## Summary`, marks the PR ready, pushes.
9. **Merge.** Reviewer squash-merges.

**Branch protection (manual setup).** Configure `main` to require the
`autodev-guard/*` checks + `ci/*` checks + at least one approving
review; disallow force-pushes; prefer linear history. This lives in
GitHub UI (or Terraform later), not in a repo file.

---

## 6. Overlap list

Soft list (not CI-enforced) — if your plan touches any of these, run
`gh pr list --search 'is:draft'` before requesting `plan-approved` and
skim for overlap. If you see a conflict, comment on that PR and pause.

*No overlap-list files yet.* Add entries as public interfaces
stabilize — e.g., CLI flags, output schemas, shared type definitions,
API contracts, workflow YAMLs under `.github/workflows/`.

---

## 7. Artifact templates (Spec lane)

Copy from [`.tasks/_template/`](.tasks/_template/) — `task_start.sh`
does this automatically. Required sections:

**`spec.md`** — `## Context` (problem statement) → `## Open Questions
for the Human` (≤5, written BEFORE re-reading the ticket) → `##
Research findings` (every claim anchored to a `file:line`, existing
module, or reference doc) → `## Approach` (chosen + at least one
alternative, with why the chosen one wins) → `## Tradeoffs accepted`
→ `## Failure modes` (adversarial re-read) → `## Acceptance criteria`
(each automated-or-observable, with an explicit "what would falsify
it" line).

**`plan.md`** — `## Tactical steps` (ordered, each citing the
`spec.md` section it implements) → `## Files touched` (one line each)
→ `## Depends on` (other open issues/PRs that must land first —
informational).

**`implementation.md`** — `## Task tree` (hierarchical checklist,
ticked as items land) → `## Deviations` (cite spec/plan section, what
changed, why; if a shortcut, name the ceiling and upgrade path) →
`## Summary` (incremental, derived into the PR description at
ready-time). Each entry carries a trust-tier marker (§2).

---

## 8. Housekeeping

**Context hygiene — 40 / 60.** Keep agent context under 40%. Start a
fresh session at 60% regardless of window size. Persist to
`.tasks/<id>/` and
[`.agent/project-memory.md`](.agent/project-memory.md); reload only
what's needed. `.tasks/<id>/context.md` is a **gitignored** working
file for cross-session handoff — never reviewed, never committed. For
non-trivial implementation, break into ordered slices with a testable
checkpoint at each boundary; a slice boundary is a natural session
reset point.

**Project memory.** Read
[`.agent/project-memory.md`](.agent/project-memory.md) at task start.
Append durable findings at task end — post-mortem nuggets, "this
pattern keeps biting us," subtle invariants worth carrying across
tasks. Not ephemeral task state (that belongs in `implementation.md`).
Format: `###` heading with a date and a short body citing `file:line`
evidence.

**Deferred.** The Optimize lane (skill evolutionary loop with
`score.json`, judge ensemble, held-out test set, per-generation
branches; adds labels `lane:optimize` and `optimization-approved`) is
not implemented.
