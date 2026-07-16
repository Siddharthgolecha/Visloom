#!/usr/bin/env bash
#
# gen-contracts.sh — regenerate packages/contracts/{ts,rust,py}/ from
# packages/contracts/{events,openapi}/ per ADR 0011.
#
# Pins live literally below. CI runs `make contracts` (which invokes
# this script from `packages/contracts/`) then `git diff --exit-code`
# against the generated trees — any diff fails the build.
#
# See docs/adr/0011-generated-contracts-committed-with-drift-check.md
# and .tasks/arch-contracts-package-.../spec.md §scripts/gen-contracts.sh.

set -euo pipefail

# ── Pinned generator versions ───────────────────────────────────────
# Bumping any of these is a Spec-lane change per AGENTS.md §6. Rerun
# `make contracts` in the same PR — CI enforces the resulting diff.
DMCG_VERSION=0.68.1               # datamodel-code-generator (JSON Schema → Pydantic v2)
TYPIFY_VERSION=0.7.0              # cargo-typify (JSON Schema → serde structs)
JSTT_VERSION=15.0.4               # json-schema-to-typescript
REDOCLY_VERSION=2.39.0            # @redocly/cli
# ────────────────────────────────────────────────────────────────────

log() { printf '[gen-contracts] %s\n' "$*" >&2; }
die() { printf '[gen-contracts] error: %s\n' "$*" >&2; exit 1; }

# Locate `packages/contracts/` regardless of caller's cwd — script
# lives at scripts/gen-contracts.sh in the repo root.
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
pkg="$repo_root/packages/contracts"
[[ -d "$pkg/events" ]] || die "expected $pkg/events (repo layout changed?)"

# ── Step 0: prerequisites ───────────────────────────────────────────
for cmd in uv cargo npx; do
    command -v "$cmd" >/dev/null 2>&1 || die "$cmd not on PATH — see slice 9's bootstrap script for install"
done

# Load VERSION for manifest stamping.
version="$(<"$pkg/VERSION")"
log "regenerating contracts at v$version"

# ── Step 1: clean output trees ──────────────────────────────────────
log "cleaning ts/ rust/ py/"
rm -rf "$pkg/ts" "$pkg/rust" "$pkg/py"
mkdir -p "$pkg/ts" "$pkg/rust/src/events" "$pkg/py/visloom_contracts/events"

# ── Step 2: bundle stream schemas for generators that don't follow $ref
# `cargo-typify`, older JSTT versions, and datamodel-code-generator's
# multi-file mode all trip on bare relative $refs like
# "_envelope.v1.json". Inline the envelope into each stream schema
# under a private id; source-of-truth files stay untouched. Bundled
# copies land in a tmp dir keyed to this run.
bundle_dir="$(mktemp -d)"
trap 'rm -rf "$bundle_dir"' EXIT
log "bundling schemas into $bundle_dir"

python3 - "$pkg/events" "$bundle_dir" <<'PY'
import json, sys, pathlib
src, dst = map(pathlib.Path, sys.argv[1:3])
envelope = json.loads((src / "_envelope.v1.json").read_text())
env_props = envelope["properties"]
env_required = envelope["required"]
for schema_path in sorted(src.glob("*.v1.json")):
    if schema_path.name == "_envelope.v1.json":
        (dst / schema_path.name).write_text(schema_path.read_text())
        continue
    doc = json.loads(schema_path.read_text())
    props = {**env_props, **doc.get("properties", {})}
    required = list({*env_required, *doc.get("required", [])})
    bundled = {
        "$schema": doc["$schema"],
        "$id": doc["$id"],
        "title": doc["title"],
        "description": doc["description"],
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
    }
    (dst / schema_path.name).write_text(json.dumps(bundled, indent=2) + "\n")
PY

# ── Step 3: Python bindings via datamodel-code-generator ────────────
log "python codegen (datamodel-code-generator==$DMCG_VERSION)"
uvx --from "datamodel-code-generator==$DMCG_VERSION" --with ruff datamodel-codegen \
    --input "$bundle_dir" \
    --input-file-type jsonschema \
    --output "$pkg/py/visloom_contracts/events" \
    --output-model-type pydantic_v2.BaseModel \
    --target-python-version 3.12 \
    --use-schema-description \
    --use-title-as-name \
    --use-standard-collections \
    --use-annotated \
    --field-constraints \
    --disable-timestamp \
    --formatters ruff-format \
    >&2

# ruff-format leaves a .ruff_cache/ next to the formatted files.
# Not part of the contract — drop it so the drift-check is deterministic.
rm -rf "$pkg/py/visloom_contracts/events/.ruff_cache"

# datamodel-code-generator writes the input path into its output
# header ("# filename: …"). Our bundle path is a mktemp dir → the
# header is nondeterministic across runs → drift-check would fail.
# Rewrite each generated file's `filename:` line to the source-of-
# truth schema name so reruns are byte-identical.
python3 - "$pkg/py/visloom_contracts/events" <<'PY'
import re, sys, pathlib
outdir = pathlib.Path(sys.argv[1])
pat = re.compile(r"^(#\s+filename:\s+).*$", re.M)
for py_file in outdir.glob("*.py"):
    if py_file.name == "__init__.py":
        # __init__.py has no matching source; strip the header line entirely.
        text = py_file.read_text()
        text = pat.sub("# (generated)", text)
        py_file.write_text(text)
        continue
    # Per-schema module: recover the original schema stem from the module name.
    stem = py_file.stem.replace("_v1", ".v1")
    stem = stem.replace("_media_", ".media.")
    stem = stem.replace("index_failed", "index_failed")
    text = py_file.read_text()
    text = pat.sub(f"# filename:  {stem}.json", text)
    py_file.write_text(text)
PY

# ── Step 4: Rust bindings via cargo-typify (bundled inputs) ─────────
log "rust codegen (cargo-typify $TYPIFY_VERSION)"
if ! cargo typify --version 2>/dev/null | grep -q "$TYPIFY_VERSION"; then
    log "  installing cargo-typify@$TYPIFY_VERSION"
    cargo install --locked --version "$TYPIFY_VERSION" cargo-typify >&2
fi

for schema in "$bundle_dir/"jobs.*.json "$bundle_dir/"events.*.json; do
    stem="$(basename "$schema" .json)"
    out="$pkg/rust/src/events/${stem//./_}.rs"
    cargo typify --output "$out" "$schema" >&2
done

# ── Step 5: TypeScript bindings via json-schema-to-typescript ───────
# Feeds the same bundled inputs so the generator never sees an
# unresolved bare $ref.
log "typescript codegen (json-schema-to-typescript@$JSTT_VERSION)"
mkdir -p "$pkg/ts/events"
for schema in "$bundle_dir/"_envelope.v1.json \
              "$bundle_dir/"jobs.*.json \
              "$bundle_dir/"events.*.json; do
    stem="$(basename "$schema" .json)"
    out="$pkg/ts/events/${stem}.ts"
    npx --yes --package="json-schema-to-typescript@$JSTT_VERSION" \
        json2ts --input "$schema" --output "$out" \
        --additionalProperties false --unknownAny true --strictIndexSignatures \
        >&2
done

# ── Step 5: OpenAPI lint (no code emit yet — client stubs land later)
log "openapi lint (@redocly/cli@$REDOCLY_VERSION)"
npx --yes --package="@redocly/cli@$REDOCLY_VERSION" \
    redocly lint \
        --config "$pkg/openapi/redocly.yaml" \
        "$pkg/openapi/openapi.v1.yaml" >&2

# ── Step 6: per-language manifests ──────────────────────────────────
log "writing per-language manifests (v$version)"

cat > "$pkg/py/pyproject.toml" <<PY
# DO NOT EDIT — regenerate via \`make contracts\` (packages/contracts/).
# Manifest for the generated Pydantic bindings under visloom_contracts/.
[project]
name = "visloom-contracts"
version = "$version"
description = "Generated Pydantic v2 bindings for Visloom event schemas."
requires-python = ">=3.12"
dependencies = ["pydantic>=2,<3"]

[tool.setuptools.packages.find]
include = ["visloom_contracts*"]
PY

cat > "$pkg/rust/Cargo.toml" <<RUST
# DO NOT EDIT — regenerate via \`make contracts\` (packages/contracts/).
# Manifest for the generated serde bindings under src/events/.
[package]
name = "visloom-contracts"
version = "$version"
edition = "2021"
publish = false
description = "Generated serde bindings for Visloom event schemas."

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"
regress = "0.9"
chrono = { version = "0.4", features = ["serde"] }
RUST

cat > "$pkg/rust/src/lib.rs" <<'RUSTLIB'
// DO NOT EDIT — regenerate via `make contracts` (packages/contracts/).
// Root module for generated event bindings. See src/events/ for the
// per-stream types.
pub mod events;
RUSTLIB

cat > "$pkg/rust/src/events/mod.rs" <<'RUSTMOD'
// DO NOT EDIT — regenerate via `make contracts` (packages/contracts/).
pub mod jobs_media_index_v1;
pub mod events_media_indexed_v1;
pub mod events_media_index_failed_v1;
RUSTMOD

cat > "$pkg/ts/package.json" <<TS
{
  "//": "DO NOT EDIT — regenerate via 'make contracts' (packages/contracts/). Types-only package: json-schema-to-typescript emits interfaces (no runtime code), so we ship .ts sources under an exports map. Slice 8's web build compiles them at consumption time.",
  "name": "@visloom/contracts",
  "version": "$version",
  "description": "Generated TypeScript types for Visloom event schemas. Types-only — no runtime bindings.",
  "type": "module",
  "types": "./events/_envelope.v1.ts",
  "exports": {
    ".": "./events/_envelope.v1.ts",
    "./events/envelope": "./events/_envelope.v1.ts",
    "./events/jobs.media.index.v1": "./events/jobs.media.index.v1.ts",
    "./events/media.indexed.v1": "./events/events.media.indexed.v1.ts",
    "./events/media.index_failed.v1": "./events/events.media.index_failed.v1.ts"
  },
  "files": ["events"],
  "private": true
}
TS

# ── Step 7: DO-NOT-EDIT banner sentinels ────────────────────────────
# Some generators strip comments; write a top-of-tree marker file so
# the acceptance criterion greps see one guaranteed hit per language.
for lang_dir in "$pkg/ts" "$pkg/rust" "$pkg/py"; do
    cat > "$lang_dir/README.md" <<'BANNER'
# DO NOT EDIT

This tree is generated. Regenerate via `make contracts` from
`packages/contracts/`. Source of truth is
`packages/contracts/{events,openapi}/`.
BANNER
done

log "done"
