#!/bin/bash

# Tails Web REPL - One-command development setup
# This script installs dependencies, builds WASM, and starts the development server

set -e

echo "🐈‍⬛ Tails Web REPL Development Setup"
echo "=================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the root of the Tails project"
    exit 1
fi

echo "📋 Step 1: Checking dependencies..."

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check for wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "⚠️  wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Add WASM target
echo "🎯 Step 2: Adding WebAssembly target..."
rustup target add wasm32-unknown-unknown

# Build WASM package
echo "🔧 Step 3: Building WebAssembly package..."
./build-web.sh

# Check if Python is available
echo "🐍 Step 4: Checking for Python..."
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 not found. Please install Python 3"
    exit 1
fi

echo "🚀 Step 5: Starting development server..."
echo ""
echo "✅ Setup complete! The web REPL will be available at:"
echo "   http://localhost:8000"
echo ""
echo "   Press Ctrl+C to stop the server"
echo ""

# Start the development server
./serve-web.sh