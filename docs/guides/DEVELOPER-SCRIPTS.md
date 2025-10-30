# Developer Scripts Guide

This document explains the npm scripts available for developers working on Modulaur.

## Quick Reference

```bash
# Before committing code
npm run precommit

# Full validation (matches CI)
npm run validate

# Format all code
npm run format

# Check without fixing
npm run check:all
```

## Development Scripts

### Starting Development

```bash
# Start frontend dev server only
npm run dev

# Start Tauri app (includes frontend)
npm run dev:tauri

# Build plugins and start Tauri app
npm run dev:full
```

### Building

```bash
# Build frontend only
npm run build

# Build Tauri app (release mode)
npm run build:tauri

# Full release build (plugins + frontend + Tauri)
npm run build:release

# Debug build (faster, for testing)
npm run build:debug
```

### Testing

```bash
# Run frontend tests
npm run test

# Run Rust tests
npm run test:rust
```

## Code Quality Scripts

### Formatting (Auto-Fix)

```bash
# Format all code (Frontend + Rust)
npm run format

# Format frontend only (Prettier + ESLint)
npm run lint:fix

# Format Rust only
cd src-tauri && cargo fmt
```

**What gets formatted:**
- **Frontend**: TypeScript, Vue, JavaScript, JSON, CSS, Markdown
- **Rust**: All .rs files with rustfmt

### Linting (Check Only)

```bash
# Check frontend formatting
npm run format:check

# Lint frontend
npm run lint

# Check Rust formatting
cd src-tauri && cargo fmt --check

# Lint Rust (Clippy)
npm run clippy
```

### Type Checking

```bash
# TypeScript type check
npm run type-check
```

## Validation Scripts

### Individual Checks

```bash
# Frontend validation (type-check + lint + format)
npm run check:frontend

# Rust validation (format + clippy + tests)
npm run check:rust

# All checks (Frontend + Rust)
npm run check:all
```

### Full Validation (Matches CI)

```bash
# This runs the same checks as GitHub Actions
npm run validate
```

**Includes:**
1. Build all plugins
2. Frontend checks (type-check, lint, format)
3. Rust checks (format, clippy, tests)
4. Build frontend

### Pre-Commit Check

```bash
# Format code and run all checks
npm run precommit
```

**Recommended:** Run this before committing to catch issues locally.

## Plugin Scripts

```bash
# Build all plugins
npm run plugins:build

# Deploy plugins (create junction)
npm run plugins:deploy

# Build single plugin
npm run plugin:build <plugin-name>
```

## Cleanup Scripts

```bash
# Clean all build artifacts and reinstall dependencies
npm run clean

# Clean without reinstalling
npm run clean:skip-install

# Clean frontend only
npm run clean:frontend

# Clean Rust only
npm run clean:backend
```

## Utility Scripts

```bash
# Check bundle size after build
npm run check:size

# Install all dependencies (root + frontend + mocks)
npm run install:all
```

## Configuration Files

### Frontend Formatting & Linting

- **`.prettierrc`** - Prettier configuration
  - Semi: false
  - Single quotes: true
  - Tab width: 2
  - Print width: 100

- **`eslint.config.js`** - ESLint configuration
  - Vue recommended rules
  - TypeScript support
  - Prettier integration

### Rust Formatting & Linting

- **`rustfmt.toml`** - Rust formatting configuration
  - Edition: 2021
  - Max width: 100
  - Tab spaces: 4
  - Reorder imports: true

- **`.clippy.toml`** - Clippy linting configuration
  - Cognitive complexity: 30
  - Deny warnings in CI

## Workflow Examples

### Daily Development

```bash
# 1. Start development
npm run dev:full

# 2. Make changes

# 3. Format code
npm run format

# 4. Check for issues
npm run check:all

# 5. Fix any issues
npm run lint:fix
npm run clippy:fix
```

### Before Creating PR

```bash
# 1. Format and validate everything
npm run precommit

# 2. If all checks pass, commit
git add .
git commit -m "feat: your feature"

# 3. Push and create PR
git push origin your-branch
```

### Fixing CI Failures

If GitHub Actions validation fails:

```bash
# 1. Run the same validation locally
npm run validate

# 2. See what failed and fix it

# 3. For formatting issues
npm run format

# 4. For lint issues
npm run lint:fix

# 5. For Clippy issues
npm run clippy:fix

# 6. Re-run validation
npm run validate
```

## Matching GitHub Workflows

The `npm run validate` script matches the GitHub Actions validation workflow:

### GitHub Validation Steps:
1. ✅ Install dependencies
2. ✅ Build plugins
3. ✅ Type check frontend
4. ✅ Lint frontend
5. ✅ Check formatting (Prettier)
6. ✅ Build frontend
7. ✅ Check Rust formatting (`cargo fmt --check`)
8. ✅ Lint Rust (`cargo clippy -- -D warnings`)
9. ✅ Run Rust tests

### Local Validation:
```bash
npm run validate
# Runs: plugins:build + check:frontend + check:rust + build
```

## Strictness Levels

### Warnings vs Errors

**ESLint:**
- Most rules set to `warn` for development flexibility
- `no-var` is `error` (enforced)

**Clippy:**
- All warnings treated as errors in CI (`-D warnings`)
- Locally, warnings are just warnings unless you use `-- -D warnings`

**TypeScript:**
- Type errors always fail the build

### Making Local Checks Stricter

To match CI strictness exactly:

```bash
# Frontend - treat warnings as errors
cd src
npm run lint -- --max-warnings 0

# Rust - treat warnings as errors
cd src-tauri
cargo clippy -- -D warnings
```

Or use the validation script which already does this:
```bash
npm run validate
```

## Auto-Fixing

### What Can Be Auto-Fixed?

**Frontend:**
- ✅ Code formatting (Prettier)
- ✅ Many ESLint issues (`--fix`)
- ❌ Type errors (manual fix required)

**Rust:**
- ✅ Code formatting (`cargo fmt`)
- ✅ Some Clippy issues (`cargo clippy --fix`)
- ❌ Most logic issues (manual fix required)

### Auto-Fix Everything

```bash
# Format and fix what's fixable
npm run format
npm run lint:fix
npm run clippy:fix

# Then check what's left
npm run check:all
```

## VS Code Integration

Install these extensions for automatic formatting on save:

- **Prettier - Code formatter** (esbenp.prettier-vscode)
- **ESLint** (dbaeumer.vscode-eslint)
- **rust-analyzer** (rust-lang.rust-analyzer)

Add to `.vscode/settings.json`:
```json
{
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

## Troubleshooting

### "Command not found" errors

Make sure you've installed all dependencies:
```bash
npm run install:all
```

### Formatting doesn't change files

Check that files aren't in `.prettierignore` or ESLint ignore patterns.

### Clippy errors about unstable features

Update Rust:
```bash
rustup update stable
```

### Type errors in .vue files

Try:
```bash
cd src
npm run type-check
```

Look for `// @ts-ignore` comments that might be hiding issues.

## Resources

- [Prettier Documentation](https://prettier.io/docs/en/)
- [ESLint Documentation](https://eslint.org/docs/latest/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Rustfmt Options](https://rust-lang.github.io/rustfmt/)

## Summary

**Before every commit:**
```bash
npm run precommit
```

**Before every PR:**
```bash
npm run validate
```

**If CI fails:**
```bash
npm run validate  # Should catch the same issues
```

This ensures your local environment is as strict as the GitHub Actions workflows! 🚀

