#!/usr/bin/env bash
#
# task_propose.sh — propose a Spec-lane task in Kepler.
#
# Opens a GitHub issue with the task statement, applies labels
# `lane-pending` and `agent-driven`, and posts the agent's first
# comment with a one-line lane rationale. Does NOT create a branch,
# scaffold `.tasks/<id>/`, or open a draft PR — those happen in
# `task_start.sh` AFTER the human confirms the lane by flipping
# `lane-pending` → `lane:considered`.
#
# See AGENTS.md §5 (Spec lane lifecycle), steps 1–2.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/task_propose.sh "<task-title>" "<original-request>" [--rationale "<one line>"]
  tools/autodev/task_propose.sh "<task-title>" -f|--file "<path>"     [--rationale "<one line>"]

Options:
  -f, --file <path>    Read the task request from a file (Markdown accepted).
  --rationale "<line>" One-line lane rationale posted as the first issue
                       comment. Default: "lane:considered — <title>".

Emits a one-line JSON summary on stdout:
  {"issue_url":"...","issue_number":"..."}

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
    -h|--help)   usage; exit 0 ;;
    *)           die "unknown argument: $1" ;;
  esac
done

command -v gh >/dev/null 2>&1 || die "gh CLI not installed. See https://cli.github.com/"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated. Run 'gh auth login'."
command -v git >/dev/null 2>&1 || die "git not installed"
git rev-parse --show-toplevel >/dev/null 2>&1 || die "not inside a git repository"

[[ -n "$rationale" ]] || rationale="lane:considered — ${title}"

log "opening GitHub issue"
issue_body_file="$(mktemp)"
{
  printf '## Task statement\n\n%s\n' "$request"
} > "$issue_body_file"

issue_url="$(gh issue create \
  --title "$title" \
  --body-file "$issue_body_file" \
  --label lane-pending \
  --label agent-driven)"
rm -f "$issue_body_file"
issue_number="${issue_url##*/}"

log "posting lane-proposal comment on #${issue_number}"
comment_body_file="$(mktemp)"
{
  printf 'Lane proposal: **%s**\n\n' "$rationale"
  printf 'To confirm, replace label `lane-pending` with `lane:considered`, then run:\n\n'
  printf '```\ntools/autodev/task_start.sh %s\n```\n' "$issue_number"
} > "$comment_body_file"
gh issue comment "$issue_number" --body-file "$comment_body_file" >/dev/null
rm -f "$comment_body_file"

log "done"
printf '{"issue_url":"%s","issue_number":"%s"}\n' "$issue_url" "$issue_number"
