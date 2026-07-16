"""Contract-tier tests for `packages/contracts/`.

Every test reads sources under `../events/`, `../openapi/`, or
`../schema.sql`. Failures here fail CI even when generated files
are unchanged — that's the point of the tier.
"""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

import pytest
import yaml
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]  # packages/contracts/
EVENTS = ROOT / "events"
OPENAPI = ROOT / "openapi" / "openapi.v1.yaml"
REDOCLY_CONFIG = ROOT / "openapi" / "redocly.yaml"
SCHEMA_SQL = ROOT / "schema.sql"
EXAMPLES = Path(__file__).resolve().parent / "examples"

STREAM_SCHEMAS = sorted(
    p for p in EVENTS.glob("*.v1.json") if not p.name.startswith("_")
)
ALL_SCHEMAS = sorted(EVENTS.glob("*.v1.json"))
REDOCLY_VERSION = "2.39.0"


def _load(path: Path) -> dict:
    return json.loads(path.read_text())


# ─── test 1: schemas self-validate as 2020-12 ──────────────────────────
@pytest.mark.parametrize("schema_path", ALL_SCHEMAS, ids=lambda p: p.name)
def test_schemas_self_validate(schema_path: Path) -> None:
    doc = _load(schema_path)
    Draft202012Validator.check_schema(doc)


# ─── test 2: every example validates against its schema ────────────────
@pytest.mark.parametrize("schema_path", STREAM_SCHEMAS, ids=lambda p: p.name)
def test_examples_round_trip(schema_path: Path) -> None:
    example_path = EXAMPLES / schema_path.name
    assert example_path.exists(), (
        f"missing fixture for {schema_path.name} — expected at {example_path}"
    )
    # Resolve the shared `_envelope.v1.json` $ref against the events dir.
    resolver_base_uri = f"file://{EVENTS.as_posix()}/"
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT202012

    resources = [
        (
            f"{resolver_base_uri}{p.name}",
            Resource.from_contents(_load(p), default_specification=DRAFT202012),
        )
        for p in ALL_SCHEMAS
    ]
    registry = Registry().with_resources(resources)
    validator = Draft202012Validator(_load(schema_path), registry=registry)
    validator.validate(_load(example_path))


# ─── test 3: OpenAPI file lints clean under pinned redocly ─────────────
def test_openapi_lints() -> None:
    result = subprocess.run(
        [
            "npx",
            "--yes",
            f"--package=@redocly/cli@{REDOCLY_VERSION}",
            "redocly",
            "lint",
            "--config",
            str(REDOCLY_CONFIG),
            str(OPENAPI),
        ],
        capture_output=True,
        text=True,
        timeout=180,
    )
    assert result.returncode == 0, (
        f"redocly lint failed:\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
    )


# ─── test 4: every stream schema $refs the shared envelope ─────────────
@pytest.mark.parametrize("schema_path", STREAM_SCHEMAS, ids=lambda p: p.name)
def test_envelope_ref_shared(schema_path: Path) -> None:
    doc = _load(schema_path)
    refs = _collect_refs(doc)
    assert any("_envelope.v1.json" in r for r in refs), (
        f"{schema_path.name} does not $ref _envelope.v1.json — envelope drift risk"
    )


def _collect_refs(node: object) -> list[str]:
    refs: list[str] = []
    if isinstance(node, dict):
        for k, v in node.items():
            if k == "$ref" and isinstance(v, str):
                refs.append(v)
            else:
                refs.extend(_collect_refs(v))
    elif isinstance(node, list):
        for item in node:
            refs.extend(_collect_refs(item))
    return refs


# ─── test 5: schema.sql media_kind enum matches JSON schema ────────────
def test_schema_sql_enums_match() -> None:
    """The `media_kind` enum in schema.sql matches the JSON-schema
    enum in `jobs.media.index.v1.json`. Catches SQL/JSON drift at
    review time; full cross-check lands in slice 5.
    """
    sql = SCHEMA_SQL.read_text()
    m = re.search(r"media_kind\s+text\s+NOT NULL\s+CHECK\s*\(media_kind\s+IN\s*\(([^)]+)\)\)", sql)
    assert m, "media_kind CHECK constraint missing or unparseable in schema.sql"
    sql_kinds = {v.strip().strip("'") for v in m.group(1).split(",")}
    doc = _load(EVENTS / "jobs.media.index.v1.json")
    json_kinds = set(doc["properties"]["data"]["properties"]["media_kind"]["enum"])
    assert sql_kinds == json_kinds, (
        f"media_kind drift — schema.sql: {sql_kinds}, JSON schema: {json_kinds}"
    )


# ─── test 6: openapi is a parseable 3.1 doc ────────────────────────────
def test_openapi_is_31() -> None:
    doc = yaml.safe_load(OPENAPI.read_text())
    assert doc.get("openapi") == "3.1.0", f"expected openapi: 3.1.0, got {doc.get('openapi')!r}"
