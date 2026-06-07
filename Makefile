.PHONY: install build test-xask-shell verify verify-docs verify-repo verify-install
.ONESHELL:
SHELL := /bin/bash
.SHELLFLAGS := -euo pipefail -c

## build: pure compile (no deploy). Use `make install` to sync binary + xask + templates.
## Rationale: xask-gate-regress-0420 R2 connector finding — prior `build` deployed only
## the binary, creating R1-twin (fresh xbreed with stale templates/dispatch). Install is
## the only deploy path; build stays side-effect-free.
## NOTE: lower-level targets (build, manual cp) bypass the gate — known limitation.
## Only `install` and `verify-install` provide the full atomic post-deploy check.
build:
	cargo build --release

## test-xask-shell: shell-level integration test for xask {{EFFORT}} substitution
test-xask-shell:
	@bash tests/xask_effort_substitution.sh

## verify: clippy, test, format, and all install checks (full local gate)
verify: verify-repo verify-install

## verify-repo: git-tree-only checks (cargo + shell tests with no home-dir reads)
verify-repo:
	@echo "verify: cargo clippy --all-targets"; cargo clippy --all-targets
	@echo "verify: cargo test"; cargo test
	@echo "verify: cargo fmt --check"; cargo fmt --check
	@echo "verify: tests/ssot_build_binding.sh"; bash tests/ssot_build_binding.sh
	@echo "verify: tests/required_sections_mutation.sh"; bash tests/required_sections_mutation.sh
	@echo "verify: tests/mirror_drift_mutation.sh"; bash tests/mirror_drift_mutation.sh
	@echo "verify: tests/xask_effort_substitution.sh"; bash tests/xask_effort_substitution.sh
	@echo "verify: tests/xask_full_flag.sh"; bash tests/xask_full_flag.sh
	@echo "verify: tests/xask_gemini_effort_transport.sh"; bash tests/xask_gemini_effort_transport.sh
	@echo "verify: tests/xask_cross_model_divergence.sh"; bash tests/xask_cross_model_divergence.sh
	@echo "verify: scripts/verify-routing.sh"; bash scripts/verify-routing.sh
	@echo "verify: scripts/verify-docs.sh"; bash scripts/verify-docs.sh
	@echo "verify: tests/routing_drift_mutation.sh"; bash tests/routing_drift_mutation.sh

## verify-install: home-state checks (symlinks, binaries, dispatch copies, axis schema)
## XBREED_AGENTS_DIR is derived from XBREED_HOME so fixture runs never leak to real $HOME.
verify-install:
	@H="$${XBREED_HOME:-$$HOME}"
	XBREED_AGENTS_DIR="$$H/.claude/agents" bash scripts/verify-install.sh
	XBREED_AGENTS_DIR="$$H/.claude/agents" bash tests/axis_family_schema_check.sh
	@echo "verify: tests/xask_template_missing_fail_loud.sh"; bash tests/xask_template_missing_fail_loud.sh
	@echo "verify: tests/xask_thinking_budget_reachable.sh"; bash tests/xask_thinking_budget_reachable.sh

## verify-docs: check connector routing consistency across SSoT copies
verify-docs:
	@bash scripts/verify-docs.sh

## install: build + deploy xbreed binary, scripts/xask, dispatch templates atomically
## Final step runs verify-install as an unconditional post-install gate.
install: build
	@H="$${XBREED_HOME:-$$HOME}"
	mkdir -p "$$H/.local/bin"
	cp --remove-destination target/release/xbreed "$$H/.local/bin/xbreed"
	cp scripts/xask "$$H/.local/bin/xask"
	chmod +x "$$H/.local/bin/xask"
	mkdir -p "$$H/.local/templates/dispatch"
	cp templates/dispatch/*.md "$$H/.local/templates/dispatch/"
	echo "Installed xbreed + xask + dispatch templates → $$H/.local/bin/ and $$H/.local/templates/dispatch/"
	$(MAKE) verify-install
