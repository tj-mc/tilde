#!/bin/bash

# Tails Global Installation Script
# Installs Tails to system PATH so users can run "tails" globally

set -e

# Get version from GitHub releases API (latest tag)
VERSION=$(curl -s https://api.github.com/repos/tj-mc/tails/releases/latest | grep -o '"tag_name": *"[^"]*"' | cut -d'"' -f4 | sed 's/^v//')

# Fallback if API fails
if [ -z "$VERSION" ]; then
    VERSION="0.1.0"
    echo "⚠️  Could not fetch latest version, using fallback: $VERSION"
fi
INSTALL_DIR="/usr/local/bin"
TEMP_DIR=$(mktemp -d)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🐈‍⬛ Installing Tails v$VERSION..."

# Detect platform and map to release asset names
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    "Linux")
        case "$ARCH" in
            "x86_64") ASSET_NAME="linux-x64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ARCHIVE_EXT="tar.gz"
        ;;
    "Darwin")
        case "$ARCH" in
            "x86_64") ASSET_NAME="macos-intel" ;;
            "arm64") ASSET_NAME="macos-arm64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ARCHIVE_EXT="tar.gz"
        ;;
    "MINGW"*|"MSYS_NT"*|"CYGWIN"*)
        ASSET_NAME="windows-x64"
        ARCHIVE_EXT="zip"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

echo "Detected platform: $ASSET_NAME"

# Download URL
GITHUB_REPO="tj-mc/tails"
DOWNLOAD_URL="https://github.com/$GITHUB_REPO/releases/download/v$VERSION/tails-v$VERSION-$ASSET_NAME.$ARCHIVE_EXT"

# Create temporary directory
cd "$TEMP_DIR"

echo "Downloading Tails binary..."
ARCHIVE_FILE="tails.$ARCHIVE_EXT"

if command -v curl >/dev/null 2>&1; then
    curl -L "$DOWNLOAD_URL" -o "$ARCHIVE_FILE" || {
        echo -e "${RED}Download failed. Please check your internet connection and try again.${NC}"
        echo "Manual download URL: $DOWNLOAD_URL"
        exit 1
    }
elif command -v wget >/dev/null 2>&1; then
    wget "$DOWNLOAD_URL" -O "$ARCHIVE_FILE" || {
        echo -e "${RED}Download failed. Please check your internet connection and try again.${NC}"
        echo "Manual download URL: $DOWNLOAD_URL"
        exit 1
    }
else
    echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

echo "Extracting..."
if [ "$ARCHIVE_EXT" = "tar.gz" ]; then
    tar -xzf "$ARCHIVE_FILE"
elif [ "$ARCHIVE_EXT" = "zip" ]; then
    unzip -q "$ARCHIVE_FILE"
fi

# Find the binary in the extracted directory
if [ "$OS" = "MINGW"* ] || [ "$OS" = "MSYS_NT"* ] || [ "$OS" = "CYGWIN"* ]; then
    BINARY_PATH=$(find . -name "tails.exe" -type f | head -1)
    BINARY_NAME="tails.exe"
else
    BINARY_PATH=$(find . -name "tails" -type f | head -1)
    BINARY_NAME="tails"
fi

if [ -z "$BINARY_PATH" ]; then
    echo -e "${RED}Binary ($BINARY_NAME) not found in downloaded package${NC}"
    exit 1
fi

# Check if we have write permissions to install directory
if [ ! -w "$INSTALL_DIR" ]; then
    echo "Installing to $INSTALL_DIR requires sudo permissions..."
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/tails"
    sudo chmod +x "$INSTALL_DIR/tails"
else
    cp "$BINARY_PATH" "$INSTALL_DIR/tails"
    chmod +x "$INSTALL_DIR/tails"
fi

# Cleanup
cd - > /dev/null
rm -rf "$TEMP_DIR"

# Verify installation
if command -v tails >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Tails installed successfully!${NC}"
    echo ""
    echo "Try it out:"
    echo "  tails                 # Start REPL"
    echo "  tails myprogram.tails # Run a file"
    echo ""
    tails --version 2>/dev/null || echo "Version: $VERSION"
else
    echo -e "${YELLOW}Installation completed, but 'tails' command not found in PATH.${NC}"
    echo "You may need to restart your terminal or add $INSTALL_DIR to your PATH."
    echo ""
    echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\""
fi