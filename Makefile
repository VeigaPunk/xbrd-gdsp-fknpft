.PHONY: install build test-xask-shell verify verify-docs

build:
	cargo build --release
	mkdir -p "$(HOME)/.local/bin"
	cp --remove-destination target/release/xbreed "$(HOME)/.local/bin/xbreed"

## test-xask-shell: shell-level integration test for xask {{EFFORT}} substitution
test-xask-shell:
	@bash tests/xask_effort_substitution.sh

## verify: cargo test + shell integration tests
verify:
	@set -e
	@echo "verify: cargo clippy --all-targets"; cargo clippy --all-targets
	@echo "verify: cargo test"; cargo test
	@echo "verify: cargo fmt --check"; cargo fmt --check
	@echo "verify: tests/ssot_build_binding.sh"; bash tests/ssot_build_binding.sh
	@echo "verify: tests/required_sections_mutation.sh"; bash tests/required_sections_mutation.sh
	@echo "verify: tests/mirror_drift_mutation.sh"; bash tests/mirror_drift_mutation.sh
	@echo "verify: tests/xask_gemini_effort_transport.sh"; bash tests/xask_gemini_effort_transport.sh
	@echo "verify: tests/xask_effort_substitution.sh"; bash tests/xask_effort_substitution.sh
	@echo "verify: tests/xask_failloud.sh"; bash tests/xask_failloud.sh
	@echo "verify: tests/xask_cross_model_divergence.sh"; bash tests/xask_cross_model_divergence.sh
	@echo "verify: tests/xask_full_flag.sh"; bash tests/xask_full_flag.sh
	@echo "verify: tests/axis_family_schema_check.sh"; bash tests/axis_family_schema_check.sh

## verify-docs: check connector routing consistency across SSoT copies
verify-docs:
	@bash scripts/verify-docs.sh

## install: build xbreed binary + sync scripts/xask → ~/.local/bin/xask
install: build
	cp scripts/xask "$(HOME)/.local/bin/xask"
	chmod +x "$(HOME)/.local/bin/xask"
	@echo "Installed xbreed + xask → $(HOME)/.local/bin/"
