# Contributing to Visloom

Visloom is developed as a human + AI-agent collaboration. This document is
the human-facing companion to [`AGENTS.md`](AGENTS.md).

## What to expect from the agent

The agent's posture: evidence-first, adversarial-by-default, trust-tiered,
append-only, privacy-aware. Concretely:

- Engineering claims in `spec.md` / `plan.md` / `implementation.md` /
  commit messages are anchored to `file:line` / test-name cites, not
  "I think."
- Implementation entries carry a **trust-tier marker**
  (`tested-against-real-input` / `tested-with-fixture` /
  `untested-assumption` / `inherited-from-existing-code`) so you know
  what to scrutinize.
- Mandatory `## Open Questions` and `## Failure modes` sections in
  `spec.md` — answer the Open Questions before reviewing the rest.
- The agent pushes back on weak rejections (one round of
  counter-argument before revising). This is intentional — meek
  compliance is a failure mode. Rejecting *with* reasoning is what
  ends the debate.
- The agent stops and asks at implementation forks it can't resolve
  from context.
- Disagreements are preserved in `## Deviations` or as follow-up Open
  Questions — never silently capitulated.
- Past `implementation.md` entries are never deleted — strikethrough
  only. The log *is* the audit.
- Sensitive user content (private photos, faces, personal data) never
  leaks into PR bodies or commit messages.
- The agent's solo ceiling is "tested against real input" — reaching
  "peer-reviewed engineering" needs your review.

## Two lanes: Quick and Spec

The agent picks the lane at task creation. When it's ambiguous, the
agent defaults to Spec. If you disagree — flip the label yourself
(`lane:quick` or `lane:considered`) and the agent will adjust.

### Quick lane

For small, bounded changes with no design decisions worth writing down.
Typical: typo fixes, `.md` clarifications, a one-file bugfix that
restores documented behavior, a dep bump inside compat range.

1. Agent branches off `main` as `fix/<slug>`, `feat/<slug>`, or
   `docs/<slug>` (via `tools/autodev/quick_start.sh`).
2. Agent makes the change and runs whatever tests exist for the
   touched area.
3. Agent opens a PR direct off `main` (via
   `tools/autodev/quick_finish.sh`) with labels `lane:quick` and
   `agent-driven`. PR body: problem, change, how it was tested.
4. You review and merge (or request revisions *with reasoning* so the
   agent's pushback is informed).

No `.tasks/<id>/` directory. No `spec.md`. No `plan-approved` gate.
The PR description is the whole audit.

**Escalation.** If the agent discovers mid-flight that a Quick task is
actually Spec-sized (hidden invariant surfaces, a second file needs
touching, a design decision emerges), it will stop, comment on the PR,
close it, open an issue, and restart via Spec lane. Don't ask the agent
to inflate a Quick PR in place — the missing `spec.md` history is
exactly the audit trail Spec lane exists to produce.

### Spec lane

For everything else: new modules, new evidence rules, changes to public
interfaces, work that touches files in the [overlap list](AGENTS.md#6-overlap-list),
or any change where a design decision needs to be recorded.

1. **Propose** — `tools/autodev/task_propose.sh` opens a GitHub issue
   describing the problem, applies labels `lane-pending` +
   `agent-driven`, and posts the agent's first comment: a one-line
   lane rationale plus the `task_start.sh` command to run once
   confirmed. **No branch or PR is created yet.**
2. **Lane-confirmation gate (you)** — on the issue, replace
   `lane-pending` with `lane:considered`:
   `gh issue edit <N> --remove-label lane-pending --add-label lane:considered`.
   If you disagree with the agent's rationale — or think it's Quick
   sized — comment on the issue and close it; the agent will restart
   in the correct lane. This is your first veto point.
3. **Branch + scaffold + draft PR** — after the gate flips, the agent
   runs `tools/autodev/task_start.sh <issue-number>`. It refuses
   unless `lane:considered` is present, then creates `spec/<id>` off
   `main`, scaffolds `.tasks/<id>/{spec,plan,implementation}.md`, and
   opens a draft PR linked with `Closes #<N>` and labels
   `lane:considered`, `lane-pending`, `agent-driven`.
4. **Spec + plan commit** — the agent commits `spec.md` first, then
   `plan.md`, and requests your review.
5. **Plan review** — this is where your attention matters most. See
   the reviewer checklist below.
6. **Plan-approved** — when you're satisfied, add the `plan-approved`
   label (`gh pr edit <pr> --add-label plan-approved`). The agent
   unblocks.
7. **Implementation** — code lands with each commit appending to
   `implementation.md`. Structural deviations halt for re-approval
   (agent asks you to remove the `plan-approved` label).
8. **Ready** — the agent runs `tools/autodev/task_finish.sh`, which
   rebases, extracts the `## Summary` into the PR description, and
   marks the PR ready.
9. **Merge** — squash-merge.

## Reviewer checklists

### Quick lane

Fast read. The PR body is the audit.

- [ ] Problem, change, and test statements are each one or two
      sentences. If the change explanation exceeds a short paragraph,
      it probably wants Spec lane — ask the agent to escalate.
- [ ] Diff scope matches the description. If the diff touches files
      the description doesn't mention, ask for the escalation.
- [ ] The tested-how line names something concrete (a test file, a
      command run, a manual step) — not "verified locally."
- [ ] No overlap-list files touched. If any are, the change belongs in
      Spec lane; ask the agent to restart there.

### Spec lane (at the lane-confirmation gate, step 2)

Ten-second read of the issue. You're deciding whether the agent's
lane call is right — not whether the task is a good idea.

- [ ] The agent's lane rationale in the first comment names a *concrete*
      reason (new module / touches an overlap-list file / needs design
      decision recorded). "This felt bigger" is not a rationale.
- [ ] If any of the [Quick-lane checklist](#quick-lane) bullets fit,
      the agent probably called it wrong — close the issue and ask the
      agent to restart in Quick.
- [ ] Otherwise: flip `lane-pending` → `lane:considered` on the issue.
      The agent picks up from there.

### Spec lane (at plan review, step 5)

- [ ] **Answer the agent's `## Open Questions` first.** These drove the
      agent's approach; answering them last means you're reviewing a
      spec built on unresolved questions.
- [ ] **`## Research findings` claims are anchored.** Every claim
      should cite a specific file, existing module, or reference doc.
      Free-form opinion in this section is a red flag.
- [ ] **`## Approach` presents at least one alternative.** If not, ask
      why. A spec with no alternative considered is one where the
      agent may have absorbed the ticket's framing without challenge.
- [ ] **`## Acceptance criteria` are automated-or-observable.** Each
      should have a "what would falsify it" line.
- [ ] **`## Failure modes` names concrete, actionable failures** — not
      a generic risk list.
- [ ] **`## Files touched` in `plan.md` matches the intent.** Scope
      creep starts here.
- [ ] **If rejecting: write the reason.** The agent pushes back on
      weak rejections. Rejections with reasoning end the debate.

## Label vocabulary

Six labels for v1:

- `lane:quick` — Quick lane PR.
- `lane:considered` — Spec lane PR.
- `lane-pending` — issue is triaged but no lane confirmed yet. Used
  during Spec-lane setup.
- `plan-approved` — you've reviewed and accepted `spec.md` + `plan.md`;
  agent may implement. Spec lane only.
- `agent-driven` — audit signal that this PR was produced by an agent
  under this workflow.
- `reverted` — applied post-merge if a change is rolled back.

Deferred (added when the Optimize lane lands): `lane:optimize`,
`optimization-approved`.

## 3-rejection escape hatch (Spec lane)

If a plan is rejected three times, the agent stops revising and asks
you to take over, reformulate the task, or close it. This exists so
that a rejection loop doesn't burn cycles indefinitely.

A rejection whose reasoning the agent disputes counts as one round of
debate, not a settled rejection — it only counts toward the cap once
you confirm or address the counter-argument.

## Overriding the agent

- **Reject with reasoning** — informs the agent's pushback.
- **Flip the lane label** — if you think a Quick PR should have been
  Spec (or vice versa), change the label and the agent will restart in
  the correct lane.
- **Take over** — check out the branch and edit the files yourself.
  Remove the `agent-driven` label if the PR is now human-driven.
- **Close the task** — close the PR and the linked issue (if any).
  `.tasks/<id>/` files in the branch will disappear with the branch.
