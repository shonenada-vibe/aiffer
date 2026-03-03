#!/usr/bin/env bash
set -euo pipefail

echo "=== Aiffer Development Environment Setup ==="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1 found: $(command -v "$1")"
        return 0
    else
        echo -e "${RED}✗${NC} $1 not found"
        return 1
    fi
}

echo ""
echo "--- Checking prerequisites ---"

# Check Rust toolchain
check_command rustc && rustc --version
check_command cargo && cargo --version

# Check Bun
check_command bun && bun --version

# Check Tauri CLI
if ! cargo tauri --version &> /dev/null; then
    echo -e "${YELLOW}Installing Tauri CLI...${NC}"
    cargo install tauri-cli --version "^2"
fi
echo -e "${GREEN}✓${NC} Tauri CLI: $(cargo tauri --version)"

echo ""
echo "--- Installing dependencies ---"

# Install frontend dependencies
if [ -f "package.json" ]; then
    echo "Installing frontend dependencies with Bun..."
    bun install
    echo -e "${GREEN}✓${NC} Frontend dependencies installed"
else
    echo -e "${YELLOW}⚠${NC} No package.json found — skip frontend deps (expected for initial scaffold task)"
fi

# Build Rust dependencies
if [ -f "src-tauri/Cargo.toml" ]; then
    echo "Checking Rust dependencies..."
    cd src-tauri
    cargo check
    echo -e "${GREEN}✓${NC} Rust dependencies OK"
    cd ..
else
    echo -e "${YELLOW}⚠${NC} No src-tauri/Cargo.toml found — skip Rust deps (expected for initial scaffold task)"
fi

echo ""
echo "--- Environment ready ---"

# Start dev server if project is scaffolded
if [ -f "package.json" ] && [ -f "src-tauri/Cargo.toml" ]; then
    echo ""
    echo -e "${GREEN}Starting development server...${NC}"
    echo "Run: cargo tauri dev"
    echo ""
    # Don't auto-start — let the agent decide when to start
    # cargo tauri dev &
else
    echo -e "${YELLOW}Project not yet scaffolded. Complete task 001-scaffold-tauri-svelte first.${NC}"
fi

echo ""
echo -e "${GREEN}=== Setup complete ===${NC}"
