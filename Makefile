.PHONY: install build

build:
	cargo build --release
	mkdir -p "$(HOME)/.local/bin"
	cp --remove-destination target/release/xbreed "$(HOME)/.local/bin/xbreed"

## install: build xbreed binary + sync scripts/xask → ~/.local/bin/xask
install: build
	cp scripts/xask "$(HOME)/.local/bin/xask"
	chmod +x "$(HOME)/.local/bin/xask"
	@echo "Installed xbreed + xask → $(HOME)/.local/bin/"
