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

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    "Linux")
        case "$ARCH" in
            "x86_64") TARGET="x86_64-unknown-linux-gnu" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ;;
    "Darwin")
        case "$ARCH" in
            "x86_64") TARGET="x86_64-apple-darwin" ;;
            "arm64") TARGET="aarch64-apple-darwin" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${NC}"
        exit 1
        ;;
esac

echo "Detected platform: $TARGET"

# Download URL
GITHUB_REPO="tj-mc/tails"
DOWNLOAD_URL="https://github.com/$GITHUB_REPO/releases/download/v$VERSION/tails-v$VERSION-$TARGET.tar.gz"

# Create temporary directory
cd "$TEMP_DIR"

echo "Downloading Tails binary..."
if command -v curl >/dev/null 2>&1; then
    curl -L "$DOWNLOAD_URL" -o "tails.tar.gz" || {
        echo -e "${RED}Download failed. Please check your internet connection and try again.${NC}"
        echo "Manual download URL: $DOWNLOAD_URL"
        exit 1
    }
elif command -v wget >/dev/null 2>&1; then
    wget "$DOWNLOAD_URL" -O "tails.tar.gz" || {
        echo -e "${RED}Download failed. Please check your internet connection and try again.${NC}"
        echo "Manual download URL: $DOWNLOAD_URL"
        exit 1
    }
else
    echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

echo "Extracting..."
tar -xzf tails.tar.gz

# Find the binary in the extracted directory
BINARY_PATH=$(find . -name "tails" -type f | head -1)
if [ -z "$BINARY_PATH" ]; then
    echo -e "${RED}Binary not found in downloaded package${NC}"
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