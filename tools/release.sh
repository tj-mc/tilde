#!/bin/bash

# Automated Release Script for Tilde
# This script automates the entire release process

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Utility functions
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

print_step() {
    echo -e "${BLUE}\n🚀 $1${NC}"
}

# Check if version argument is provided
if [ -z "$1" ]; then
    print_error "Usage: $0 <version> [--dry-run]"
    echo "Example: $0 0.2.0"
    echo "         $0 0.2.0 --dry-run (to test without pushing)"
    exit 1
fi

NEW_VERSION="$1"
DRY_RUN=""

if [ "$2" = "--dry-run" ]; then
    DRY_RUN="true"
    print_warning "DRY RUN MODE - No changes will be pushed to Git"
fi

# Validate version format
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    print_error "Invalid version format. Use semantic versioning (e.g., 1.2.3)"
fi

print_step "Starting release process for v$NEW_VERSION"

# Check if we're on the main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    print_warning "Current branch is '$CURRENT_BRANCH', not 'main'"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_error "Release cancelled"
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    print_error "There are uncommitted changes. Please commit or stash them first."
fi

# Check if we're up to date with remote
print_info "Checking if local branch is up to date..."
git fetch origin
if [ "$(git rev-parse HEAD)" != "$(git rev-parse @{u})" ]; then
    print_error "Local branch is not up to date with remote. Please pull latest changes."
fi

# Step 1: Update version numbers
print_step "Updating version numbers"
./tools/update_version.sh "$NEW_VERSION"
print_success "Version updated to $NEW_VERSION"

# Step 2: Run tests
print_step "Running test suite"
print_info "Running unit tests..."
cargo test --quiet
print_success "All tests passed"

print_info "Running integration tests..."
cargo test --test "*integration*" --quiet
print_success "Integration tests passed"

# Step 3: Build and verify
print_step "Building release binary"
cargo build --release --quiet
print_success "Release binary built successfully"

# Verify version in binary
BINARY_VERSION=$(./target/release/tilde --version | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' | sed 's/v//')
if [ "$BINARY_VERSION" != "$NEW_VERSION" ]; then
    print_error "Binary version ($BINARY_VERSION) doesn't match expected version ($NEW_VERSION)"
fi
print_success "Binary version verified: v$BINARY_VERSION"

# Step 4: Build WASM (optional, for verification)
print_step "Testing WASM build"
print_info "Building WASM module..."
cargo check --target wasm32-unknown-unknown --quiet
print_success "WASM build successful"

# Step 5: Generate release notes summary
print_step "Generating release summary"
CHANGELOG_SECTION=$(sed -n "/## \[$NEW_VERSION\]/,/## \[/p" CHANGELOG.md | sed '$d' | tail -n +2)

if [ -z "$CHANGELOG_SECTION" ]; then
    print_error "No changelog entry found for version $NEW_VERSION. Please update CHANGELOG.md first."
fi

echo "Release notes preview:"
echo "────────────────────────────────────────"
echo "$CHANGELOG_SECTION"
echo "────────────────────────────────────────"

# Step 6: Confirm release
if [ -z "$DRY_RUN" ]; then
    echo
    print_warning "Ready to release v$NEW_VERSION"
    echo "This will:"
    echo "  • Commit all changes"
    echo "  • Create and push git tag v$NEW_VERSION"
    echo "  • Trigger GitHub Actions release workflow"
    echo
    read -p "Continue with release? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_error "Release cancelled"
    fi
fi

# Step 7: Commit and tag
print_step "Creating release commit and tag"

if [ -z "$DRY_RUN" ]; then
    # Commit changes
    git add .
    git commit -m "Release v$NEW_VERSION

- Updated version to $NEW_VERSION
- Updated CHANGELOG.md with release notes
- All tests passing
- Ready for distribution

🐈‍⬛ Generated with release automation script"

    print_success "Release commit created"

    # Create annotated tag
    git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION

$(echo "$CHANGELOG_SECTION" | head -20)

See CHANGELOG.md for complete release notes."

    print_success "Git tag v$NEW_VERSION created"

    # Push changes and tag
    print_step "Pushing to GitHub"
    git push origin main
    git push origin "v$NEW_VERSION"
    print_success "Changes and tag pushed to GitHub"

    # Step 8: Wait for and monitor GitHub Actions
    print_step "Monitoring GitHub Actions release"
    print_info "GitHub Actions workflow should start automatically..."
    print_info "You can monitor the release at: https://github.com/tj-mc/tilde/actions"
    print_info "Release will be available at: https://github.com/tj-mc/tilde/releases/tag/v$NEW_VERSION"

else
    print_warning "DRY RUN: Would commit and push tag v$NEW_VERSION"
fi

# Step 9: Post-release information
print_step "Release process completed!"

if [ -z "$DRY_RUN" ]; then
    echo
    print_success "🎉 Release v$NEW_VERSION initiated successfully!"
    echo
    echo "Next steps:"
    echo "  📦 Monitor GitHub Actions: https://github.com/tj-mc/tilde/actions"
    echo "  📋 Check release page: https://github.com/tj-mc/tilde/releases/tag/v$NEW_VERSION"
    echo "  🌐 Web REPL will auto-update: https://tj-mc.github.io/tilde/"
    echo "  📢 Consider announcing the release!"
    echo
    print_info "The GitHub Actions workflow will:"
    echo "  • Build binaries for all platforms"
    echo "  • Create GitHub release with assets"
    echo "  • Update the web REPL automatically"
else
    echo
    print_info "DRY RUN completed - no changes were made"
    echo "To perform the actual release, run:"
    echo "  $0 $NEW_VERSION"
fi