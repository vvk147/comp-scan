#!/usr/bin/env bash
set -euo pipefail

# CompScan Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/vvk147/comp-scan/main/install.sh | bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

REPO="https://github.com/vvk147/comp-scan.git"
INSTALL_DIR="${COMPSCAN_INSTALL_DIR:-$HOME/.compscan}"

echo ""
echo -e "${CYAN}   ___                      ____${NC}"
echo -e "${CYAN}  / __\\___  _ __ ___  _ __ / ___|  ___ __ _ _ __${NC}"
echo -e "${CYAN} / /  / _ \\| '_ \` _ \\| '_ \\___ \\ / __/ _\` | '_ \\${NC}"
echo -e "${CYAN}/ /__| (_) | | | | | | |_) |__) | (_| (_| | | | |${NC}"
echo -e "${CYAN}\\____/\\___/|_| |_| |_| .__/____/ \\___\\__,_|_| |_|${NC}"
echo -e "${CYAN}                      |_|${NC}"
echo ""
echo -e "${BOLD}  Local AI Agent Installer${NC}"
echo ""

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"
echo -e "  ${GREEN}System:${NC} $OS $ARCH"

# Check for Rust
if command -v cargo &>/dev/null; then
    RUST_VER=$(rustc --version | awk '{print $2}')
    echo -e "  ${GREEN}Rust:${NC}   $RUST_VER (found)"
else
    echo -e "  ${YELLOW}Rust:${NC}   not found — installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    export PATH="$HOME/.cargo/bin:$PATH"
    echo -e "  ${GREEN}Rust:${NC}   $(rustc --version | awk '{print $2}') (installed)"
fi

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Clone or update
if [ -d "$INSTALL_DIR" ]; then
    echo -e "  ${GREEN}Repo:${NC}   updating existing installation..."
    cd "$INSTALL_DIR"
    git pull --quiet
else
    echo -e "  ${GREEN}Repo:${NC}   cloning..."
    git clone --quiet "$REPO" "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

# Build
echo -e "  ${GREEN}Build:${NC}  compiling release binary (this takes 1-2 minutes)..."
cargo build --release --quiet 2>/dev/null || cargo build --release

# Install
echo -e "  ${GREEN}Install:${NC} copying binary..."
cargo install --path . --quiet 2>/dev/null || cargo install --path .

# Verify
if command -v compscan &>/dev/null; then
    VERSION=$(compscan --version | awk '{print $2}')
    echo ""
    echo -e "  ${GREEN}${BOLD}CompScan v${VERSION} installed successfully!${NC}"
else
    BINARY="$INSTALL_DIR/target/release/compscan"
    echo ""
    echo -e "  ${GREEN}${BOLD}CompScan built successfully!${NC}"
    echo -e "  ${YELLOW}Binary at: $BINARY${NC}"
    echo -e "  ${YELLOW}Add to PATH: export PATH=\"\$HOME/.cargo/bin:\$PATH\"${NC}"
fi

echo ""
echo -e "  ${BOLD}Get started:${NC}"
echo -e "  ${CYAN}  compscan scan${NC}        Run your first system health check"
echo -e "  ${CYAN}  compscan observe${NC}     Start background activity tracking"
echo -e "  ${CYAN}  compscan report${NC}      Generate AI-powered insights"
echo -e "  ${CYAN}  compscan dashboard${NC}   Launch interactive terminal dashboard"
echo -e "  ${CYAN}  compscan web${NC}         Open web dashboard at localhost:7890"
echo ""
echo -e "  ${BOLD}Full guide:${NC} https://github.com/vvk147/comp-scan/blob/main/docs/USER_GUIDE.md"
echo ""
