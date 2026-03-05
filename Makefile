.PHONY: dev build check test lint clippy fmt clean install install-deps install-cli setup cask

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

# Generate Homebrew cask file from a GitHub release tag
# Usage: make cask TAG=v0.1.0 [REPO=owner/name] [OUT=dist/homebrew/aiffer.rb]
cask:
	@if [ -z "$(TAG)" ]; then echo "Usage: make cask TAG=v0.1.0 [REPO=owner/name] [OUT=path]"; exit 1; fi
	@./scripts/generate-homebrew-cask.sh --tag "$(TAG)" $(if $(REPO),--repo "$(REPO)",) $(if $(OUT),--output "$(OUT)",)
