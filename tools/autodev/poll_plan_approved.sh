#!/usr/bin/env bash
#
# poll_plan_approved.sh — block until the `plan-approved` label appears on
# the task's PR. Prints one line per poll to stdout (current labels) so it's
# tail-friendly.
#
# See AGENTS.md §3 step 5.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/poll_plan_approved.sh <task-dir> [--timeout <secs>] [--interval <secs>]
  tools/autodev/poll_plan_approved.sh --pr <N>   [--timeout <secs>] [--interval <secs>]

Defaults:
  --timeout   3600   (max seconds to wait)
  --interval    30   (seconds between polls)

Exit codes:
  0  `plan-approved` observed on the PR
  1  timeout reached without observing the label
USAGE
}

log() { echo "[poll] $*" >&2; }
die() { echo "[poll] error: $*" >&2; exit 1; }

[[ $# -ge 1 ]] || { usage; exit 1; }
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then usage; exit 0; fi

pr_number=""
task_dir=""
timeout=3600
interval=30

while [[ $# -gt 0 ]]; do
  case "$1" in
    --pr)        pr_number="${2:-}"; [[ -n "$pr_number" ]] || die "--pr requires a number"; shift 2 ;;
    --timeout)   timeout="${2:-}"; shift 2 ;;
    --interval)  interval="${2:-}"; shift 2 ;;
    -h|--help)   usage; exit 0 ;;
    *)           task_dir="$1"; shift ;;
  esac
done

command -v gh >/dev/null 2>&1 || die "gh CLI not installed"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated"

if [[ -z "$pr_number" ]]; then
  [[ -n "$task_dir" ]] || die "either <task-dir> or --pr <N> is required"
  [[ -f "$task_dir/meta.env" ]] || die "meta.env missing in $task_dir"
  # shellcheck disable=SC1090
  source "$task_dir/meta.env"
  pr_number="${PR_NUMBER:-}"
  [[ -n "$pr_number" ]] || die "PR_NUMBER not set in $task_dir/meta.env"
fi

log "polling PR #$pr_number for label 'plan-approved' (timeout ${timeout}s, interval ${interval}s)"

deadline=$(( $(date +%s) + timeout ))
prev=""

while :; do
  now=$(date +%s)
  if (( now >= deadline )); then
    log "timeout after ${timeout}s"
    exit 1
  fi

  labels="$(gh pr view "$pr_number" --json labels --jq '.labels[].name' 2>/dev/null | sort | paste -sd, - || true)"

  if [[ "$labels" != "$prev" ]]; then
    echo "labels: ${labels:-<none>}"
    prev="$labels"
  fi

  if grep -qx 'plan-approved' <<<"${labels//,/$'\n'}"; then
    log "'plan-approved' observed — unblocking"
    exit 0
  fi

  sleep "$interval"
done
