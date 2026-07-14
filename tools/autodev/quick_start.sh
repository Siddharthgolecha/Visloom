#!/usr/bin/env bash
#
# quick_start.sh — start a Quick-lane change in Kepler.
#
# Creates a `<kind>/<slug>` branch off <base> (default main). No task dir,
# no scaffolding — the PR body is the whole audit. See AGENTS.md §4.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/quick_start.sh fix|feat|docs "<slug>" [--base <branch>] [--allow-dirty]

Options:
  --base <branch>   Branch to fork from (default: main).
  --allow-dirty     Skip the clean-tree check.

Emits a one-line JSON summary on stdout:
  {"branch":"...","base":"..."}
USAGE
}

log() { echo "[quick_start] $*" >&2; }
die() { echo "[quick_start] error: $*" >&2; exit 1; }

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

[[ $# -ge 2 ]] || { usage; exit 1; }

kind="$1"; shift
slug_raw="$1"; shift

case "$kind" in
  fix|feat|docs) : ;;
  *) die "kind must be one of: fix, feat, docs (got: $kind)" ;;
esac

base_branch="main"
allow_dirty=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base)        base_branch="${2:-}"; [[ -n "$base_branch" ]] || die "--base requires a branch name"; shift 2 ;;
    --allow-dirty) allow_dirty=true; shift ;;
    -h|--help)     usage; exit 0 ;;
    *)             die "unknown argument: $1" ;;
  esac
done

command -v git >/dev/null 2>&1 || die "git not installed"
repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || die "not inside a git repository"
cd "$repo_root"

slug="$(printf '%s' "$slug_raw" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//')"
[[ -n "$slug" ]] || die "slug is empty after normalization"

if [[ "$allow_dirty" != "true" && -n "$(git status --porcelain)" ]]; then
  die "working tree is not clean. Commit/stash first, or pass --allow-dirty."
fi

if ! git rev-parse --verify "$base_branch" >/dev/null 2>&1; then
  if git rev-parse --verify "origin/$base_branch" >/dev/null 2>&1; then
    base_branch="origin/$base_branch"
  else
    die "base branch not found: $base_branch"
  fi
fi

branch="${kind}/${slug}"
git show-ref --verify --quiet "refs/heads/$branch" && die "branch already exists: $branch"

log "creating branch $branch off $base_branch"
git checkout -b "$branch" "$base_branch"

log "done"
printf '{"branch":"%s","base":"%s"}\n' "$branch" "$base_branch"
