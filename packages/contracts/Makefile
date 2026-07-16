# Package-local Makefile for the contracts package.
# CI cd's into this directory and invokes `make contracts` / `make test`.
# Root Makefile is deferred to slice 9 (arch-development-workflow).
#
# See docs/adr/0011-generated-contracts-committed-with-drift-check.md.

.PHONY: contracts lint test clean help
.DEFAULT_GOAL := help

REPO_ROOT := $(shell git rev-parse --show-toplevel)
REDOCLY_VERSION := 2.39.0

contracts: ## Regenerate ts/ rust/ py/ from events/*.json + openapi/*.yaml.
	$(REPO_ROOT)/scripts/gen-contracts.sh

lint: ## Lint the OpenAPI spec via pinned redocly.
	npx --yes --package=@redocly/cli@$(REDOCLY_VERSION) redocly lint --config openapi/redocly.yaml openapi/openapi.v1.yaml

test: ## Run schema self-validation + example round-trip + envelope-ref + openapi-lint tests.
	uv run --frozen --directory tests pytest -v

clean: ## Remove generated trees. Regenerate via `make contracts`.
	rm -rf ts rust py

help:
	@awk 'BEGIN{FS":.*##"} /^[a-zA-Z_-]+:.*##/ {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
