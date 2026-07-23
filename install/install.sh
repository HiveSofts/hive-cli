#!/usr/bin/env bash

set -e

REPO="HiveSofts/hive-cli"
BINARY="hive"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo ""
echo -e "${CYAN}${BOLD}  🐝 Hive CLI Installer${RESET}"
echo -e "${BLUE}  ━━━━━━━━━━━━━━━━━━━━${RESET}"
echo ""

# Detect latest version
echo -e "${YELLOW}  ⏳ Detecting latest version...${RESET}"
VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest \
  | grep '"tag_name":' \
  | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo -e "${RED}  ❌ Failed to detect latest Hive version${RESET}"
    exit 1
fi

echo -e "${GREEN}  ✅ Version: ${BOLD}${VERSION}${RESET}"
echo ""

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        PLATFORM="linux"
        ;;
    Darwin)
        PLATFORM="macos"
        ;;
    *)
        echo -e "${RED}  ❌ Unsupported operating system: $OS${RESET}"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}  ❌ Unsupported architecture: $ARCH${RESET}"
        exit 1
        ;;
esac

ASSET="hive-$PLATFORM"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"

echo -e "${YELLOW}  ⬇️  Downloading Hive...${RESET}"
echo -e "${BLUE}  ${DOWNLOAD_URL}${RESET}"

TMP=$(mktemp)

# Download with progress
curl -L --progress-bar "$DOWNLOAD_URL" -o "$TMP"

if [ ! -s "$TMP" ]; then
    echo -e "${RED}  ❌ Download failed — file is empty${RESET}"
    exit 1
fi

chmod +x "$TMP"

echo ""
echo -e "${YELLOW}  📦 Installing Hive...${RESET}"

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP" "$INSTALL_DIR/$BINARY"
else
    sudo mv "$TMP" "$INSTALL_DIR/$BINARY"
fi

echo ""
echo -e "${GREEN}${BOLD}  ✅ Hive installed successfully!${RESET}"
echo ""

# Test installation
if command -v hive &> /dev/null; then
    echo -e "${CYAN}  Version:${RESET}"
    hive --version 2>/dev/null || echo -e "${YELLOW}  (version info not available)${RESET}"
else
    echo -e "${YELLOW}  ⚠️  Hive not found in PATH. Try:${RESET}"
    echo -e "    export PATH=\$PATH:$INSTALL_DIR"
fi

echo ""
echo -e "${CYAN}${BOLD}  🚀 Get started:${RESET}"
echo -e "    ${BOLD}hive --help${RESET}"
echo ""