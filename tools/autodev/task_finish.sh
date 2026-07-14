#!/usr/bin/env bash
#
# task_finish.sh — finish a Spec-lane task.
#
# Requires the PR to carry the `plan-approved` label. Stages/commits any
# outstanding task changes, rebases onto <pr-base>, pushes, replaces the PR
# description with the `## Summary` section from `.tasks/<id>/implementation.md`,
# and marks the PR ready via `gh pr ready`.
#
# See AGENTS.md §3 step 7.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/task_finish.sh <task-dir> -m "<commit message>" [options]

Options:
  --remote <name>    Remote to push to (default: origin).
  --pr-base <ref>    Base branch for the PR / rebase target (default: main).
  --no-pr            Skip the PR update / ready flip (still rebases + pushes).

Exit codes:
  0  success
  1  usage / preflight error
  2  `plan-approved` label missing (blocked — run tools/autodev/poll_plan_approved.sh)

Emits a one-line JSON summary on stdout:
  {"branch":"...","commit_sha":"...","pr_url":"...","status":"ready"}
USAGE
}

log() { echo "[task_finish] $*" >&2; }
die() { echo "[task_finish] error: $*" >&2; exit 1; }

[[ $# -ge 1 ]] || { usage; exit 1; }
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then usage; exit 0; fi

task_dir="$1"; shift
commit_msg=""
remote="origin"
pr_base="main"
skip_pr=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -m)         commit_msg="${2:-}"; [[ -n "$commit_msg" ]] || die "-m requires a message"; shift 2 ;;
    --remote)   remote="${2:-}"; [[ -n "$remote" ]] || die "--remote requires a name"; shift 2 ;;
    --pr-base)  pr_base="${2:-}"; [[ -n "$pr_base" ]] || die "--pr-base requires a ref"; shift 2 ;;
    --no-pr)    skip_pr=true; shift ;;
    -h|--help)  usage; exit 0 ;;
    *)          die "unknown argument: $1" ;;
  esac
done

command -v gh >/dev/null 2>&1 || die "gh CLI not installed"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated"
command -v git >/dev/null 2>&1 || die "git not installed"

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || die "not inside a git repository"
cd "$repo_root"

[[ -d "$task_dir" ]] || die "task dir not found: $task_dir"
[[ -f "$task_dir/meta.env" ]] || die "meta.env missing in $task_dir — did task_start.sh run?"
[[ -f "$task_dir/implementation.md" ]] || die "implementation.md missing in $task_dir"

# shellcheck disable=SC1090
source "$task_dir/meta.env"
: "${TASK_ID:?}"
: "${BRANCH:?}"
: "${PR_NUMBER:?}"

current_branch="$(git rev-parse --abbrev-ref HEAD)"
[[ "$current_branch" == "$BRANCH" ]] || die "expected to be on $BRANCH, got $current_branch"

# Plan-approved gate — the whole point of this script's Kepler-specific delta.
if [[ "$skip_pr" != "true" ]]; then
  log "checking plan-approved label on PR #$PR_NUMBER"
  labels="$(gh pr view "$PR_NUMBER" --json labels --jq '.labels[].name')"
  if ! grep -qx 'plan-approved' <<<"$labels"; then
    echo "[task_finish] error: PR #$PR_NUMBER lacks the 'plan-approved' label." >&2
    echo "[task_finish] Run: tools/autodev/poll_plan_approved.sh $task_dir" >&2
    exit 2
  fi
fi

# Stage & commit anything outstanding.
if [[ -n "$(git status --porcelain)" ]]; then
  [[ -n "$commit_msg" ]] || die "-m is required when there are uncommitted changes"
  log "committing outstanding changes"
  git add -A
  git commit -q -m "$commit_msg"
fi

# Rebase onto pr-base.
log "fetching $remote/$pr_base"
git fetch -q "$remote" "$pr_base"

pre_rebase_sha="$(git rev-parse HEAD)"
log "rebasing $BRANCH onto $remote/$pr_base"
if ! git rebase "$remote/$pr_base"; then
  git rebase --abort || true
  die "rebase failed. Resolve conflicts by hand, then re-run."
fi
post_rebase_sha="$(git rev-parse HEAD)"

log "pushing $BRANCH to $remote"
if [[ "$pre_rebase_sha" != "$post_rebase_sha" ]]; then
  git push -q --force-with-lease "$remote" "$BRANCH"
else
  git push -q "$remote" "$BRANCH"
fi

commit_sha="$post_rebase_sha"

if [[ "$skip_pr" == "true" ]]; then
  log "done (--no-pr)"
  printf '{"branch":"%s","commit_sha":"%s","pr_url":null,"status":"pushed"}\n' \
    "$BRANCH" "$commit_sha"
  exit 0
fi

# Derive the PR body from implementation.md `## Summary`.
log "extracting implementation.md ## Summary into PR body"
summary_file="$(mktemp)"
{
  printf 'Closes #%s\n\n' "${ISSUE_NUMBER:-<issue>}"
  printf 'Linked task dir: `%s/`\n\n' "$task_dir"
  printf '## Implementation summary\n\n'
  awk '
    /^## Summary/ {in_sum=1; next}
    in_sum && /^## / {in_sum=0}
    in_sum {print}
  ' "$task_dir/implementation.md"
} > "$summary_file"

gh pr edit "$PR_NUMBER" --body-file "$summary_file" >/dev/null
rm -f "$summary_file"

log "marking PR #$PR_NUMBER ready"
gh pr ready "$PR_NUMBER" >/dev/null

pr_url="$(gh pr view "$PR_NUMBER" --json url --jq '.url')"

log "done"
printf '{"branch":"%s","commit_sha":"%s","pr_url":"%s","status":"ready"}\n' \
  "$BRANCH" "$commit_sha" "$pr_url"
