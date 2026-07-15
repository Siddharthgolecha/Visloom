#!/usr/bin/env bash
#
# task_propose.sh — propose a Spec-lane task in Visloom.
#
# Opens a GitHub issue with the task statement, applies labels
# `lane-pending` and `agent-driven`, and posts the agent's first
# comment with a one-line lane rationale. Does NOT create a branch,
# scaffold `.tasks/<id>/`, or open a draft PR — those happen in
# `task_start.sh` AFTER the human confirms the lane by flipping
# `lane-pending` → `lane:considered`.
#
# For "big" tasks that need multiple Spec-lane PRs (see AGENTS.md
# §5 — non-trivial implementation pauses at ~400 LOC), split them
# into a tracking parent issue + one child issue per slice:
#
#   parent=$(tools/autodev/task_propose.sh "Epic: scaffold" "…" \
#              | jq -r .issue_number)
#   for i in 1 2 3; do
#     tools/autodev/task_propose.sh "Slice $i" "…" --parent "$parent"
#   done
#
# The `--parent` flag links the child to the parent and appends a
# `- [ ] #<child> <title>` line to the parent's issue body, so the
# parent stays a live checklist of unstarted / in-flight / merged
# slices.
#
# See AGENTS.md §5 (Spec lane lifecycle), steps 1–2.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/task_propose.sh "<task-title>" "<original-request>" [options]
  tools/autodev/task_propose.sh "<task-title>" -f|--file "<path>"     [options]

Options:
  -f, --file <path>       Read the task request from a file (Markdown accepted).
  --rationale "<line>"    One-line lane rationale posted as the first issue
                          comment. Default: "lane:considered — <title>".
  --parent <issue-num>    Mark this issue as a child slice of the given
                          parent tracking issue. Prepends `Parent: #<N>`
                          to the body, appends `- [ ] #<child> <title>`
                          to the parent's body, and posts a cross-link
                          comment on the parent.
  --dry-run               Print the gh commands + rendered bodies to
                          stdout without touching GitHub. Uses a fake
                          issue number so downstream steps can be
                          exercised.

Emits a one-line JSON summary on stdout:
  {"issue_url":"...","issue_number":"...","parent":"..."}

After this returns, hand the issue URL to the human. When they flip
`lane-pending` → `lane:considered` on the issue, run:

  tools/autodev/task_start.sh <issue-number>

to create the branch, scaffold the task dir, and open the draft PR.
USAGE
}

log() { echo "[task_propose] $*" >&2; }
die() { echo "[task_propose] error: $*" >&2; exit 1; }

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

[[ $# -ge 2 ]] || { usage; exit 1; }

title="$1"; shift

request=""
rationale=""
parent=""
dry_run="false"

if [[ "${1:-}" == "-f" || "${1:-}" == "--file" ]]; then
  shift
  file_path="${1:-}"
  [[ -n "$file_path" ]] || die "--file requires a path"
  [[ -f "$file_path" ]] || die "file not found: $file_path"
  request="$(cat "$file_path")"
  shift
else
  request="${1:-}"
  [[ -n "$request" ]] || die "original request is required"
  shift
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --rationale) rationale="${2:-}"; [[ -n "$rationale" ]] || die "--rationale requires a line"; shift 2 ;;
    --parent)    parent="${2:-}";    [[ -n "$parent"    ]] || die "--parent requires an issue number"; shift 2 ;;
    --dry-run)   dry_run="true";     shift ;;
    -h|--help)   usage; exit 0 ;;
    *)           die "unknown argument: $1" ;;
  esac
done

if [[ -n "$parent" ]]; then
  [[ "$parent" =~ ^[0-9]+$ ]] || die "--parent must be a numeric issue number, got: $parent"
fi

if [[ "$dry_run" != "true" ]]; then
  command -v gh >/dev/null 2>&1 || die "gh CLI not installed. See https://cli.github.com/"
  gh auth status >/dev/null 2>&1 || die "gh is not authenticated. Run 'gh auth login'."
fi
command -v git >/dev/null 2>&1 || die "git not installed"
git rev-parse --show-toplevel >/dev/null 2>&1 || die "not inside a git repository"

[[ -n "$rationale" ]] || rationale="lane:considered — ${title}"

log "opening GitHub issue${parent:+ (child of #$parent)}"
issue_body_file="$(mktemp)"
{
  if [[ -n "$parent" ]]; then
    printf 'Parent: #%s\n\n' "$parent"
  fi
  printf '## Task statement\n\n%s\n' "$request"
} > "$issue_body_file"

if [[ "$dry_run" == "true" ]]; then
  issue_number="DRYRUN"
  issue_url="https://github.com/DRYRUN/DRYRUN/issues/DRYRUN"
  echo "--- DRY RUN: gh issue create --title \"$title\" --label lane-pending --label agent-driven --body-file <<"
  cat "$issue_body_file"
  echo "---"
else
  issue_url="$(gh issue create \
    --title "$title" \
    --body-file "$issue_body_file" \
    --label lane-pending \
    --label agent-driven)"
  issue_number="${issue_url##*/}"
fi
rm -f "$issue_body_file"

log "posting lane-proposal comment on #${issue_number}"
comment_body_file="$(mktemp)"
{
  printf 'Lane proposal: **%s**\n\n' "$rationale"
  printf 'To confirm, replace label `lane-pending` with `lane:considered`, then run:\n\n'
  printf '```\ntools/autodev/task_start.sh %s\n```\n' "$issue_number"
} > "$comment_body_file"

if [[ "$dry_run" == "true" ]]; then
  echo "--- DRY RUN: gh issue comment $issue_number --body-file <<"
  cat "$comment_body_file"
  echo "---"
else
  gh issue comment "$issue_number" --body-file "$comment_body_file" >/dev/null
fi
rm -f "$comment_body_file"

if [[ -n "$parent" ]]; then
  log "linking child #${issue_number} into parent #${parent}"
  parent_comment_file="$(mktemp)"
  printf 'Child slice proposed: #%s — %s\n' "$issue_number" "$title" > "$parent_comment_file"

  if [[ "$dry_run" == "true" ]]; then
    echo "--- DRY RUN: gh issue comment $parent --body-file <<"
    cat "$parent_comment_file"
    echo "---"
    echo "--- DRY RUN: append checklist line to parent #$parent body:"
    printf -- '- [ ] #%s %s\n' "$issue_number" "$title"
    echo "---"
  else
    gh issue comment "$parent" --body-file "$parent_comment_file" >/dev/null
    parent_body="$(gh issue view "$parent" --json body --jq .body)"
    updated_parent_body_file="$(mktemp)"
    {
      printf '%s\n' "$parent_body"
      # If the parent body has a `## Slices` section, append inside it;
      # otherwise append a new section at the end. Simpler + resilient:
      # just append to the end. The parent template puts the checklist
      # at the end of the body.
      printf -- '- [ ] #%s %s\n' "$issue_number" "$title"
    } > "$updated_parent_body_file"
    gh issue edit "$parent" --body-file "$updated_parent_body_file" >/dev/null
    rm -f "$updated_parent_body_file"
  fi
  rm -f "$parent_comment_file"
fi

log "done"
printf '{"issue_url":"%s","issue_number":"%s","parent":"%s"}\n' \
  "$issue_url" "$issue_number" "$parent"
