# 🚀 Tilde Release Process

This document outlines the complete process for creating and publishing Tilde releases.

## Prerequisites

- Repository must be pushed to GitHub (`tj-mc/tilde`)
- GitHub Actions must be enabled
- All changes committed and pushed to `main` branch

## Release Steps

### 1. Prepare Release

```bash
# Ensure you're on main with latest changes
git checkout main
git pull origin main

# Run tests to ensure everything works
cargo test

# Build locally to verify
cargo build --release
./target/release/tilde --version
```

### 2. Create and Push Tag

```bash
# Create annotated tag (replace v0.1.0 with your version)
git tag -a v0.1.0 -m "Release v0.1.0"

# Push tag to trigger GitHub Actions
git push origin v0.1.0
```

### 3. Monitor GitHub Actions

1. Go to **Actions** tab in GitHub repository
2. Watch the "Release Builds" workflow execute
3. Verify all platforms build successfully:
   - ✅ Linux x64 (`ubuntu-latest`)
   - ✅ Windows x64 (`windows-latest`)
   - ✅ macOS Intel (`x86_64-apple-darwin`)
   - ✅ macOS Apple Silicon (`aarch64-apple-darwin`)

### 4. Verify Release

1. Go to **Releases** tab in GitHub repository
2. Confirm new release `v0.1.0` was created automatically
3. Check that all binary archives are attached:
   - `tilde-v0.1.0-linux-x64.tar.gz`
   - `tilde-v0.1.0-windows-x64.zip`
   - `tilde-v0.1.0-macos-intel.tar.gz`
   - `tilde-v0.1.0-macos-arm64.tar.gz`

### 5. Test Installation

Test the install script works:

```bash
# Test install script (use a clean environment/container if possible)
curl -sSL https://raw.githubusercontent.com/tj-mc/tilde/main/install.sh | bash

# Verify global installation
tilde --version
tilde --help

# Test basic functionality
echo '~name is "World"' > test.tilde
echo 'say "Hello, " ~name "!"' >> test.tilde
tilde test.tilde
```

## Manual Release (Alternative)

If GitHub Actions fails, you can create releases manually:

### 1. Build Locally

```bash
# Build for current platform
./tools/build_release.sh

# This creates: releases/v0.1.0/tilde-v0.1.0-*.tar.gz
```

### 2. Create GitHub Release

```bash
# Install GitHub CLI if not already installed
# brew install gh  # macOS
# sudo apt install gh  # Ubuntu

# Create release and upload artifacts
gh release create v0.1.0 \
  --title "Tilde v0.1.0" \
  --notes "First release of Tilde scripting language" \
  releases/v0.1.0/*.tar.gz releases/v0.1.0/*.zip
```

## Version Bumping

### For Next Release

1. Update version in `Cargo.toml`:
   ```toml
   version = "0.2.0"  # Increment as appropriate
   ```

2. Update version in `install.sh`:
   ```bash
   VERSION="0.2.0"  # Must match Cargo.toml
   ```

3. Commit changes:
   ```bash
   git add Cargo.toml install.sh
   git commit -m "Bump version to v0.2.0"
   git push origin main
   ```

4. Follow release steps above with new tag

## Troubleshooting

### GitHub Actions Fails

- Check the Actions logs for specific errors
- Common issues:
  - Missing dependencies for cross-compilation
  - Network issues downloading dependencies
  - Permission issues with GitHub token

### Install Script Issues

- Verify the release artifacts exist and are publicly downloadable
- Test download URLs manually:
  ```bash
  curl -I https://github.com/tj-mc/tilde/releases/download/v0.1.0/tilde-v0.1.0-linux-x64.tar.gz
  ```

### Binary Issues

- Ensure all targets build successfully
- Test binaries on actual target platforms
- Check for missing system dependencies

## Release Checklist

- [ ] All tests pass (`cargo test`)
- [ ] Version updated in `Cargo.toml`
- [ ] Version updated in `install.sh`
- [ ] Changes committed and pushed to main
- [ ] Tag created and pushed
- [ ] GitHub Actions completed successfully
- [ ] Release created with all binary artifacts
- [ ] Install script tested
- [ ] Global `tilde` command works
- [ ] Basic functionality verified

## Post-Release

- [ ] Update project documentation if needed
- [ ] Announce release on relevant channels
- [ ] Monitor for user feedback and issues
- [ ] Plan next release features