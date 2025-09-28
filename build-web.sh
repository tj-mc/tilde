#!/bin/bash

# Tilde Web REPL Build Script
# This script builds the Rust code to WebAssembly and prepares it for web deployment

set -e

echo "🐈‍⬛ Building Tilde Web REPL..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package
echo "🔧 Building WebAssembly package..."
wasm-pack build --target web --out-dir web/pkg --features wasm

# Remove unnecessary files
echo "🧹 Cleaning up generated files..."
rm -f web/pkg/.gitignore
rm -f web/pkg/package.json
rm -f web/pkg/README.md

# Copy files to web directory if needed
echo "📂 Copying web assets..."
# The HTML, CSS, and JS files are already in the web directory

# Create a simple package.json for the web directory if it doesn't exist
if [ ! -f web/package.json ]; then
    echo "📦 Creating package.json..."
    cat > web/package.json << EOF
{
  "name": "tilde-web-repl",
  "version": "0.6.1",
  "description": "Tilde Scripting Language Web REPL",
  "main": "repl.js",
  "type": "module",
  "scripts": {
    "build": "../build-web.sh",
    "serve": "python3 -m http.server 8000"
  },
  "keywords": ["tilde", "repl", "wasm", "scripting"],
  "author": "Tom McIntosh",
  "license": "MIT"
}
EOF
fi

echo "✅ Build complete! Web REPL is ready."
echo ""
echo "To serve locally:"
echo "  cd web && python3 -m http.server 8000"
echo "  Then visit http://localhost:8000"
echo ""
echo "To deploy to GitHub Pages:"
echo "  The GitHub Action will handle this automatically on push to main"