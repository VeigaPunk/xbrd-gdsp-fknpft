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
	cargo clippy && cargo test && cargo fmt --check
	@bash tests/xask_effort_substitution.sh

## verify-docs: check connector routing consistency across SSoT copies
verify-docs:
	@bash scripts/verify-docs.sh

## install: build xbreed binary + sync scripts/xask → ~/.local/bin/xask
install: build
	cp scripts/xask "$(HOME)/.local/bin/xask"
	chmod +x "$(HOME)/.local/bin/xask"
	@echo "Installed xbreed + xask → $(HOME)/.local/bin/"
