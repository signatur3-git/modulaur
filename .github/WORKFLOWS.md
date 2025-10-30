# GitHub Actions Workflows

This document describes the CI/CD workflows configured for Modulaur.

## Overview

Modulaur uses GitHub Actions for automated validation and releases:

1. **Validation Workflow** - Runs on every push/PR to `main`
2. **Release Workflow** - Builds multi-platform releases when a version tag is pushed

## Validation Workflow

**File:** `.github/workflows/validate.yml`

**Triggers:**
- Push to `main` branch
- Pull requests targeting `main`

**What It Does:**
1. ✅ Checks out the repository
2. ✅ Sets up Node.js and Rust toolchains
3. ✅ Installs system dependencies (Linux)
4. ✅ Installs all npm dependencies (root, frontend, plugins)
5. ✅ Builds all plugins
6. ✅ Runs TypeScript type checking
7. ✅ Runs ESLint
8. ✅ Builds frontend
9. ✅ Checks Rust formatting (`cargo fmt`)
10. ✅ Runs Clippy linter (`cargo clippy`)
11. ✅ Runs Rust tests
12. ✅ Builds Tauri app (debug mode, no packaging)

**Purpose:** Ensures code quality and that the app builds successfully before merging.

## Release Workflow

**File:** `.github/workflows/release.yml`

**Triggers:**
- Push of version tag (e.g., `v0.5.0`)
- Manual workflow dispatch with version input

**Matrix Build Strategy:**
The workflow builds for three platforms in parallel:

| Platform | OS | Target | Bundles |
|----------|----|----|---------|
| **Windows** | `windows-latest` | `x86_64-pc-windows-msvc` | `.msi`, `.exe` (NSIS) |
| **macOS** | `macos-latest` | `universal-apple-darwin` | `.dmg`, `.app` |
| **Linux** | `ubuntu-latest` | `x86_64-unknown-linux-gnu` | `.deb`, `.AppImage` |

**Workflow Steps:**

### 1. Create Release
- Creates a draft GitHub release
- Generates release notes template
- Outputs release ID for uploading artifacts

### 2. Build Release (Matrix)
For each platform:
1. ✅ Checks out repository
2. ✅ Sets up Node.js and Rust with platform-specific targets
3. ✅ Installs system dependencies
4. ✅ Installs all npm dependencies
5. ✅ Builds all plugins
6. ✅ Builds frontend
7. ✅ Builds Tauri app with release configuration
8. ✅ Uploads built artifacts to GitHub release

### 3. Publish Release
- Waits for all platform builds to complete
- Publishes the draft release (makes it public)

## Creating a Release

### Option 1: Tag-Based Release (Recommended)

```bash
# Make sure you're on main and up to date
git checkout main
git pull

# Create and push a version tag
git tag v0.5.0
git push origin v0.5.0
```

The workflow will automatically:
1. Create a draft release
2. Build for all platforms
3. Upload installers
4. Publish the release

### Option 2: Manual Workflow Dispatch

1. Go to GitHub → Actions → Release workflow
2. Click "Run workflow"
3. Enter version (e.g., `v0.5.0`)
4. Click "Run workflow"

## Version Management

All version numbers are now synchronized at **0.5.0**:

- ✅ `package.json` (root)
- ✅ `src/package.json` (frontend)
- ✅ `src-tauri/Cargo.toml`
- ✅ `src-tauri/tauri.conf.json`

**To update version for next release:**

1. Update all four files above
2. Update `CHANGELOG.md` with new version section
3. Commit changes
4. Create and push version tag

## Bundle Types

### Windows
- **MSI** - Windows Installer Package (recommended)
- **NSIS** - Nullsoft Scriptable Install System executable

### macOS
- **DMG** - Disk Image (recommended for distribution)
- **APP** - Application bundle

### Linux
- **DEB** - Debian package (Ubuntu, Debian, etc.)
- **AppImage** - Universal Linux executable (no installation needed)

## Artifacts

After a successful release, you'll find installers at:

```
https://github.com/yourusername/modulaur/releases/tag/v0.5.0
```

Example artifacts:
- `Modulaur_0.5.0_x64_en-US.msi`
- `Modulaur_0.5.0_x64-setup.exe`
- `Modulaur_0.5.0_universal.dmg`
- `Modulaur_0.5.0_amd64.deb`
- `Modulaur_0.5.0_amd64.AppImage`

## Secrets Required

The workflows use these GitHub secrets:

- `GITHUB_TOKEN` - Automatically provided by GitHub Actions
  - Used for creating releases and uploading artifacts
  - No configuration needed

## Workflow Permissions

The workflows require these permissions (configured in workflow files):

- `contents: write` - Create releases and tags
- `packages: write` - Upload release artifacts

## Troubleshooting

### Build Fails on Plugin Step

**Problem:** Plugin build fails with npm errors

**Solution:** Check that all plugin `package.json` files are valid and dependencies are correct

### Release Build Fails on One Platform

**Problem:** One platform fails while others succeed

**Solution:** 
- Check platform-specific system dependencies
- Review build logs for that platform
- The workflow uses `fail-fast: false` so other platforms continue

### Version Mismatch

**Problem:** Different version numbers in different files

**Solution:** Update all four version locations:
1. Root `package.json`
2. `src/package.json`
3. `src-tauri/Cargo.toml`
4. `src-tauri/tauri.conf.json`

## Local Testing

Before pushing a release tag, test the build locally:

```bash
# Build plugins
npm run plugins:build

# Build release
npm run build:release
```

This ensures your code will build in CI.

## Caching

Both workflows use caching to speed up builds:

- **Node.js**: `actions/setup-node@v4` with `cache: 'npm'`
- **Rust**: `swatinem/rust-cache@v2` caching Cargo registry and build artifacts

First builds may take 10-15 minutes, subsequent builds typically 3-5 minutes.

## Notifications

GitHub will notify you (via email/notifications) when:
- Validation checks fail on PR
- Release builds complete
- Workflow fails

## Future Enhancements

Planned workflow improvements:
- [ ] Automated version bumping
- [ ] Changelog generation from commits
- [ ] Automated plugin testing
- [ ] Performance benchmarking
- [ ] Code coverage reports
- [ ] Security scanning
- [ ] Dependency updates via Dependabot

## Resources

- [Tauri Action Documentation](https://github.com/tauri-apps/tauri-action)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Tauri Building Guide](https://tauri.app/v1/guides/building/)

