#!/bin/bash

# Tilde Binary Distribution Build Script
# Builds release binaries for multiple platforms

set -e

VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
RELEASE_DIR="releases/v$VERSION"

echo "🐈‍⬛ Building Tilde v$VERSION for distribution..."

# Create release directory
mkdir -p "$RELEASE_DIR"

# Build targets
TARGETS=(
    "x86_64-unknown-linux-gnu"      # Linux x64
    "x86_64-pc-windows-gnu"         # Windows x64
    "x86_64-apple-darwin"           # macOS Intel
    "aarch64-apple-darwin"          # macOS Apple Silicon
)

# Check if cross compilation tools are available
echo "Checking cross compilation setup..."
for target in "${TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        echo "Installing target: $target"
        rustup target add "$target" || echo "Warning: Failed to add target $target"
    fi
done

# Build for each target
for target in "${TARGETS[@]}"; do
    echo "Building for $target..."

    # Determine binary extension
    if [[ "$target" == *"windows"* ]]; then
        binary_name="tilde.exe"
    else
        binary_name="tilde"
    fi

    # Build the binary
    if cargo build --release --target "$target"; then
        # Create target-specific directory
        target_dir="$RELEASE_DIR/$target"
        mkdir -p "$target_dir"

        # Copy binary
        cp "target/$target/release/$binary_name" "$target_dir/"

        # Copy documentation
        cp README.md "$target_dir/" 2>/dev/null || echo "README.md not found, skipping"
        cp LICENSE "$target_dir/" 2>/dev/null || echo "LICENSE not found, skipping"
        cp SYNTAX.md "$target_dir/" 2>/dev/null || echo "SYNTAX.md not found, skipping"

        # Copy examples
        if [ -d "examples" ]; then
            cp -r examples "$target_dir/"
        fi

        # Create archive
        cd "$RELEASE_DIR"
        if [[ "$target" == *"windows"* ]]; then
            zip -r "tilde-v$VERSION-$target.zip" "$target"
        else
            tar -czf "tilde-v$VERSION-$target.tar.gz" "$target"
        fi
        cd - > /dev/null

        echo "✅ Built for $target"
    else
        echo "❌ Failed to build for $target"
    fi
done

echo "📦 Release packages created in $RELEASE_DIR"
ls -la "$RELEASE_DIR"/*.{zip,tar.gz} 2>/dev/null || echo "No release packages found"

echo "🎉 Build complete!"