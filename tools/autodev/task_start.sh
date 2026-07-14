#!/usr/bin/env bash
#
# task_start.sh — start a Spec-lane task in Kepler.
#
# Reads a proposed issue (created by `task_propose.sh`), REFUSES unless
# `lane:considered` is present, then creates a `spec/<id>` branch off
# <base>, scaffolds `.tasks/<id>/` from the `.tasks/_template/` skeleton,
# commits it, and opens a linked draft PR with labels `lane:considered`,
# `lane-pending`, `agent-driven`.
#
# See AGENTS.md §5 (Spec lane lifecycle), step 3 onward.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  tools/autodev/task_start.sh <issue-number> [--base <branch>] [--allow-dirty]

Options:
  --base <branch>   Branch to fork from (default: main).
  --allow-dirty     Skip the clean-tree check.

Refuses if the issue does not carry the `lane:considered` label — the
human must confirm the lane first (typically by removing `lane-pending`
and adding `lane:considered` in the GitHub UI).

Emits a one-line JSON summary on stdout:
  {"task_id":"...","branch":"...","issue_url":"...","pr_url":"..."}
USAGE
}

log() { echo "[task_start] $*" >&2; }
die() { echo "[task_start] error: $*" >&2; exit 1; }

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

[[ $# -ge 1 ]] || { usage; exit 1; }

issue_number="$1"; shift
[[ "$issue_number" =~ ^[0-9]+$ ]] || die "first argument must be a numeric issue number (got: $issue_number)"

base_branch="main"
allow_dirty=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base)         base_branch="${2:-}"; [[ -n "$base_branch" ]] || die "--base requires a branch name"; shift 2 ;;
    --allow-dirty)  allow_dirty=true; shift ;;
    -h|--help)      usage; exit 0 ;;
    *)              die "unknown argument: $1" ;;
  esac
done

# Preflight
command -v gh >/dev/null 2>&1 || die "gh CLI not installed. See https://cli.github.com/"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated. Run 'gh auth login'."
command -v git >/dev/null 2>&1 || die "git not installed"

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || die "not inside a git repository"
cd "$repo_root"

[[ -d ".tasks/_template" ]] || die "missing .tasks/_template/ — is this a Kepler checkout?"

if [[ "$allow_dirty" != "true" && -n "$(git status --porcelain)" ]]; then
  die "working tree is not clean. Commit/stash first, or pass --allow-dirty."
fi

# Fetch the issue: title, body, labels.
log "fetching issue #${issue_number}"
issue_json="$(gh issue view "$issue_number" --json title,body,labels,url,state 2>/dev/null)" \
  || die "issue #${issue_number} not found or unreadable"

issue_state="$(printf '%s' "$issue_json" | jq -r '.state')"
[[ "$issue_state" == "OPEN" ]] || die "issue #${issue_number} is not open (state: $issue_state)"

labels="$(printf '%s' "$issue_json" | jq -r '.labels[].name')"

# Lane gate — MUST be lane:considered.
if ! grep -qx "lane:considered" <<<"$labels"; then
  cat >&2 <<EOF
[task_start] error: issue #${issue_number} does not carry the \`lane:considered\` label.

  Current labels:
$(sed 's/^/    - /' <<<"$labels")

  This is the lane-confirmation gate. The human confirms the agent's
  lane proposal by replacing \`lane-pending\` with \`lane:considered\`
  on the issue:

    gh issue edit ${issue_number} --remove-label lane-pending --add-label lane:considered

  Then rerun this command. If the intent was Quick-lane, run
  \`tools/autodev/quick_start.sh\` instead and close the issue.
EOF
  exit 2
fi

# Refuse if a task dir already links back to this issue — likely a re-run.
if grep -rslq "^ISSUE_NUMBER=${issue_number}$" .tasks/*/meta.env 2>/dev/null; then
  existing="$(grep -rls "^ISSUE_NUMBER=${issue_number}$" .tasks/*/meta.env | head -n1)"
  die "issue #${issue_number} already has a task dir: $(dirname "$existing")"
fi

title="$(printf '%s' "$issue_json" | jq -r '.title')"
issue_url="$(printf '%s' "$issue_json" | jq -r '.url')"

# Extract the '## Task statement' section from the issue body; fall back to
# the whole body if the section header isn't present.
request="$(printf '%s' "$issue_json" | jq -r '.body' | awk '
  BEGIN{p=0}
  /^## Task statement/{p=1; next}
  p && /^## /{p=0}
  p {print}
' | sed -E '/^[[:space:]]*$/d')"
[[ -n "$request" ]] || request="$(printf '%s' "$issue_json" | jq -r '.body')"

if ! git rev-parse --verify "$base_branch" >/dev/null 2>&1; then
  if git rev-parse --verify "origin/$base_branch" >/dev/null 2>&1; then
    base_branch="origin/$base_branch"
  else
    die "base branch not found: $base_branch"
  fi
fi

timestamp="$(date -u +%Y%m%d-%H%M%SZ)"
slug="$(printf '%s' "$title" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//')"
[[ -n "$slug" ]] || die "issue title produced an empty slug"

task_id="${slug}-${timestamp}"
branch="spec/${task_id}"
task_dir=".tasks/${task_id}"

[[ ! -d "$task_dir" ]] || die "task dir already exists: $task_dir"
git show-ref --verify --quiet "refs/heads/$branch" && die "branch already exists: $branch"

log "creating branch $branch off $base_branch"
git checkout -b "$branch" "$base_branch"

log "scaffolding $task_dir from .tasks/_template/"
mkdir -p "$task_dir"
cp .tasks/_template/spec.md "$task_dir/spec.md"
cp .tasks/_template/plan.md "$task_dir/plan.md"
cp .tasks/_template/implementation.md "$task_dir/implementation.md"

# Splice the issue's Task statement into spec.md `## Context`.
tmp="$(mktemp)"
awk -v req="$request" '
  /^## Context/ {print; print ""; print req; print ""; skip=1; next}
  skip && /^## / {skip=0; print; next}
  skip {next}
  {print}
' "$task_dir/spec.md" > "$tmp"
mv "$tmp" "$task_dir/spec.md"

# Replace <task title> placeholders across all three files.
for f in "$task_dir/spec.md" "$task_dir/plan.md" "$task_dir/implementation.md"; do
  tmp="$(mktemp)"
  sed "s|<task title>|${title//|/\\|}|g" "$f" > "$tmp"
  mv "$tmp" "$f"
done

# Gitignored working file for cross-session context handoff (see AGENTS.md §9).
: > "$task_dir/context.md"

# Metadata — PR_NUMBER filled below.
cat > "$task_dir/meta.env" <<EOT
TASK_ID=${task_id}
BRANCH=${branch}
BASE_BRANCH=${base_branch}
CREATED_AT=${timestamp}
ISSUE_NUMBER=${issue_number}
PR_NUMBER=
EOT

log "committing scaffold"
git add "$task_dir/spec.md" "$task_dir/plan.md" "$task_dir/implementation.md" "$task_dir/meta.env"
git commit -q -m "task: scaffold ${task_id}"

log "pushing branch"
git push -q -u origin "$branch"

log "opening draft PR against ${base_branch#origin/}"
pr_body_file="$(mktemp)"
{
  printf 'Closes #%s\n\n' "$issue_number"
  printf 'Linked task dir: `%s/`\n\n' "$task_dir"
  printf '## Implementation summary\n\n'
  printf '_Filled at ready-time by `tools/autodev/task_finish.sh` from `%s/implementation.md` `## Summary`._\n' "$task_dir"
} > "$pr_body_file"

pr_url="$(gh pr create \
  --draft \
  --base "${base_branch#origin/}" \
  --title "$title" \
  --body-file "$pr_body_file" \
  --label "lane:considered" \
  --label lane-pending \
  --label agent-driven)"
rm -f "$pr_body_file"
pr_number="${pr_url##*/}"

# Update meta.env with PR number.
sed -i.bak "s|^PR_NUMBER=.*|PR_NUMBER=${pr_number}|" "$task_dir/meta.env"
rm -f "$task_dir/meta.env.bak"

git add "$task_dir/meta.env"
git commit -q --amend --no-edit
git push -q --force-with-lease origin "$branch"

log "done"
printf '{"task_id":"%s","branch":"%s","issue_url":"%s","pr_url":"%s"}\n' \
  "$task_id" "$branch" "$issue_url" "$pr_url"
