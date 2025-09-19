#!/bin/bash

# Version Update Script for Tails
# Updates version numbers across all project files

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION="$1"
echo "🔄 Updating version to $NEW_VERSION"

# Update Cargo.toml (main source of truth)
echo "📦 Updating Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update .wasm-pack.toml
echo "🌐 Updating .wasm-pack.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" .wasm-pack.toml

# Update web/package.json
echo "📱 Updating web/package.json..."
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" web/package.json

# web/index.html version is now dynamic (loaded from WASM)
echo "🖥️  web/index.html version is now dynamic"

# Update build-web.sh
echo "🔧 Updating build-web.sh..."
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" build-web.sh

# Update GitHub Actions release workflow default
echo "⚙️  Updating release workflow..."
sed -i.bak "s/default: 'v.*'/default: 'v$NEW_VERSION'/" .github/workflows/release.yml

# Update CHANGELOG.md (add entry for unreleased)
echo "📝 Updating CHANGELOG.md..."
if ! grep -q "## \[Unreleased\]" CHANGELOG.md; then
    sed -i.bak "/^## \[Unreleased\]/a\\
\\
## [$NEW_VERSION] - $(date +%Y-%m-%d)\\
\\
### Added\\
- Version bump to $NEW_VERSION" CHANGELOG.md
fi

# Clean up backup files
rm -f *.bak web/*.bak .github/workflows/*.bak tools/*.bak

# Update Cargo.lock
echo "🔒 Updating Cargo.lock..."
cargo update

echo "✅ Version updated to $NEW_VERSION"
echo ""
echo "Next steps:"
echo "1. Review changes: git diff"
echo "2. Update CHANGELOG.md with actual changes"
echo "3. Commit: git add -A && git commit -m 'Bump version to v$NEW_VERSION'"
echo "4. Tag: git tag v$NEW_VERSION && git push origin v$NEW_VERSION"