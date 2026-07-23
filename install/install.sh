#!/usr/bin/env bash

set -e

REPO="HiveSofts/hive-cli"
BINARY="hive"
INSTALL_DIR="/usr/local/bin"

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


echo -e "${YELLOW}  ⏳ Detecting latest version...${RESET}"

VERSION=$(curl -fsSL \
"https://api.github.com/repos/$REPO/releases/latest" \
| grep '"tag_name":' \
| sed -E 's/.*"([^"]+)".*/\1/')


if [ -z "$VERSION" ]; then
    echo -e "${RED}  ❌ Failed to detect latest version${RESET}"
    exit 1
fi


echo -e "${GREEN}  ✅ Version: ${BOLD}$VERSION${RESET}"
echo ""


echo -e "${YELLOW}  ⬇️  Downloading Hive...${RESET}"


DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/hive"


echo -e "${BLUE}  $DOWNLOAD_URL${RESET}"
echo ""


TMP=$(mktemp)


curl -fL --progress-bar \
"$DOWNLOAD_URL" \
-o "$TMP"


if [ ! -s "$TMP" ]; then
    echo -e "${RED}  ❌ Download failed${RESET}"
    exit 1
fi


# Check downloaded file
if file "$TMP" | grep -qi "text"; then
    echo -e "${RED}  ❌ Downloaded file is not a binary${RESET}"
    cat "$TMP"
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


sudo chmod +x "$INSTALL_DIR/$BINARY"


echo ""
echo -e "${GREEN}${BOLD}  ✅ Hive installed successfully!${RESET}"
echo ""


if command -v hive >/dev/null 2>&1; then

    echo -e "${CYAN}  Version:${RESET}"
    hive --version || true

else

    echo -e "${YELLOW}  ⚠️ Hive not found in PATH${RESET}"

fi


echo ""
echo -e "${CYAN}${BOLD}  🚀 Get started:${RESET}"
echo ""
echo "    hive --help"
echo ""