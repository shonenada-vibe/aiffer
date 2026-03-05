.PHONY: dev build check test lint clippy fmt clean install install-deps install-cli setup

# Development
dev:
	bun run tauri dev

# Build production release
build:
	cargo tauri build

# Install dependencies
install-deps:
	bun install
	cd src-tauri && cargo check

# Install dependencies (alias)
install: install-deps

# Install CLI binary to /usr/local/bin
# Usage: make install-cli
# Requires: make build first
install-cli: build
	@echo "Installing aiffer to /usr/local/bin..."
	cp src-tauri/target/release/aiffer /usr/local/bin/aiffer
	@echo "Done. Run 'aiffer' or 'aiffer /path/to/repo' to launch."

# Full setup (install + verify)
setup:
	chmod +x setup.sh && ./setup.sh

# Run all checks
check: lint clippy test

# Frontend type checking
lint:
	bun run check

# Rust linting
clippy:
	cd src-tauri && cargo clippy -- -D warnings

# Rust formatting check
fmt:
	cd src-tauri && cargo fmt --check

# Format code
fmt-fix:
	cd src-tauri && cargo fmt

# Rust tests
test:
	cd src-tauri && cargo test

# Clean build artifacts
clean:
	cd src-tauri && cargo clean
	rm -rf dist
