#!/usr/bin/env bash
#
# quick_finish.sh — finish a Quick-lane change.
#
# Stages/commits outstanding changes, pushes, opens (or updates) a non-draft PR
# off <pr-base> with labels `lane:quick` and `agent-driven`. No `plan-approved`
# gate. See AGENTS.md §4.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/quick_finish.sh -m "<commit and PR subject>" [options]

Options:
  --body-file <path>   PR body file (Quick template: Problem / Change / Tested).
                       Defaults to the commit body (or the subject if none).
  --remote <name>      Remote to push to (default: origin).
  --pr-base <ref>      Base branch for the PR (default: main).
  --no-pr              Push only; don't open/update a PR.

Refuses to run if the current branch is `main` or doesn't look like a
Quick-lane branch (`fix/*`, `feat/*`, `docs/*`).

Emits a one-line JSON summary on stdout:
  {"branch":"...","commit_sha":"...","pr_url":"..."}
USAGE
}

log() { echo "[quick_finish] $*" >&2; }
die() { echo "[quick_finish] error: $*" >&2; exit 1; }

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

commit_msg=""
body_file=""
remote="origin"
pr_base="main"
skip_pr=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    -m)            commit_msg="${2:-}"; [[ -n "$commit_msg" ]] || die "-m requires a message"; shift 2 ;;
    --body-file)   body_file="${2:-}"; [[ -f "$body_file" ]] || die "--body-file not found: $body_file"; shift 2 ;;
    --remote)      remote="${2:-}"; [[ -n "$remote" ]] || die "--remote requires a name"; shift 2 ;;
    --pr-base)     pr_base="${2:-}"; [[ -n "$pr_base" ]] || die "--pr-base requires a ref"; shift 2 ;;
    --no-pr)       skip_pr=true; shift ;;
    -h|--help)     usage; exit 0 ;;
    *)             die "unknown argument: $1" ;;
  esac
done

command -v git >/dev/null 2>&1 || die "git not installed"
if [[ "$skip_pr" != "true" ]]; then
  command -v gh >/dev/null 2>&1 || die "gh CLI not installed"
  gh auth status >/dev/null 2>&1 || die "gh is not authenticated"
fi

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || die "not inside a git repository"
cd "$repo_root"

branch="$(git rev-parse --abbrev-ref HEAD)"
[[ "$branch" != "main" && "$branch" != "master" ]] || die "on $branch — Quick lane requires a fix/*, feat/*, or docs/* branch"

case "$branch" in
  fix/*|feat/*|docs/*) : ;;
  *) die "branch '$branch' doesn't look like a Quick-lane branch (fix/*, feat/*, docs/*). Use tools/autodev/quick_start.sh." ;;
esac

# Stage & commit anything outstanding.
if [[ -n "$(git status --porcelain)" ]]; then
  [[ -n "$commit_msg" ]] || die "-m is required when there are uncommitted changes"
  log "committing outstanding changes"
  git add -A
  git commit -q -m "$commit_msg"
fi

# Refuse an empty branch (no commits ahead of pr-base).
if ! git rev-list --count "$remote/$pr_base..$branch" >/dev/null 2>&1; then
  log "fetching $remote/$pr_base"
  git fetch -q "$remote" "$pr_base" || true
fi
ahead="$(git rev-list --count "$remote/$pr_base..$branch" 2>/dev/null || echo 0)"
[[ "$ahead" -gt 0 ]] || die "branch has no commits ahead of $remote/$pr_base — nothing to submit"

log "pushing $branch to $remote"
git push -q -u "$remote" "$branch"

commit_sha="$(git rev-parse HEAD)"

if [[ "$skip_pr" == "true" ]]; then
  log "done (--no-pr)"
  printf '{"branch":"%s","commit_sha":"%s","pr_url":null}\n' "$branch" "$commit_sha"
  exit 0
fi

# Derive PR title from -m (first line) or the branch's latest commit subject.
if [[ -n "$commit_msg" ]]; then
  pr_title="$(printf '%s' "$commit_msg" | head -n1)"
else
  pr_title="$(git log -1 --pretty=%s)"
fi

# Derive PR body: --body-file wins, then commit body, then subject.
tmp_body=""
if [[ -n "$body_file" ]]; then
  tmp_body="$body_file"
else
  tmp_body="$(mktemp)"
  if [[ -n "$commit_msg" ]]; then
    printf '%s\n' "$commit_msg" > "$tmp_body"
  else
    git log -1 --pretty=%B > "$tmp_body"
  fi
fi

# Check if a PR already exists for this branch.
existing_pr="$(gh pr list --head "$branch" --json number --jq '.[0].number' 2>/dev/null || true)"

if [[ -z "$existing_pr" || "$existing_pr" == "null" ]]; then
  log "opening PR against $pr_base"
  pr_url="$(gh pr create \
    --base "${pr_base#origin/}" \
    --title "$pr_title" \
    --body-file "$tmp_body" \
    --label "lane:quick" \
    --label agent-driven)"
else
  log "updating existing PR #$existing_pr"
  gh pr edit "$existing_pr" --title "$pr_title" --body-file "$tmp_body" >/dev/null
  pr_url="$(gh pr view "$existing_pr" --json url --jq '.url')"
fi

# Clean up temp body if we made it.
if [[ -z "$body_file" && -n "$tmp_body" ]]; then
  rm -f "$tmp_body"
fi

log "done"
printf '{"branch":"%s","commit_sha":"%s","pr_url":"%s"}\n' "$branch" "$commit_sha" "$pr_url"
