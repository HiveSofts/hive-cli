#!/usr/bin/env bash

set -e

REPO="HiveSofts/hive-cli"
BINARY="hive"

INSTALL_DIR="/usr/local/bin"

VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest \
| grep '"tag_name":' \
| sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "❌ Failed to detect latest Hive version"
    exit 1
fi


echo ""
echo "🐝 Hive CLI Installer"
echo "━━━━━━━━━━━━━━━━━━━━"
echo "Version: $VERSION"
echo ""


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
        echo "❌ Unsupported operating system"
        exit 1
        ;;
esac


case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;

    aarch64|arm64)
        ARCH="aarch64"
        ;;

    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac


ASSET="hive-$PLATFORM"


DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"


echo "⬇️  Downloading Hive..."
echo "$DOWNLOAD_URL"


TMP=$(mktemp)


curl -L "$DOWNLOAD_URL" -o "$TMP"


chmod +x "$TMP"


echo ""
echo "📦 Installing Hive..."


if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP" "$INSTALL_DIR/$BINARY"
else
    sudo mv "$TMP" "$INSTALL_DIR/$BINARY"
fi


echo ""
echo "✅ Hive installed successfully"
echo ""

echo "Version:"
hive --version || true

echo ""

echo "Run:"
echo "  hive --help"

echo ""