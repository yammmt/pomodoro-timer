# Quick Start: GitHub Actions CI

**Feature**: GitHub Actions CI for Rust/Tauri Project  
**Branch**: `002-github-actions-ci`  
**Last Updated**: 2026-01-02

## Overview

This feature adds automated CI checks using GitHub Actions. Three quality checks run automatically on every code push and pull request:

1. **Test** - Validates code correctness (runs on Ubuntu and macOS)
2. **Format** - Ensures consistent code style (runs on Ubuntu only)
3. **Clippy** - Enforces Rust best practices (runs on Ubuntu only)

## What Gets Created

```
.github/
└── workflows/
    └── ci.yml          # GitHub Actions workflow configuration
```

## How It Works

### Automatic Execution

CI runs automatically when you:
- Push commits to any branch
- Open, update, or reopen a pull request

### Job Execution

Three jobs run in parallel:

**1. Test Job** (`cargo test`)
- Runs on: ubuntu-latest AND macOS-latest (matrix strategy)
- Timeout: 10 minutes per OS
- Working directory: `src-tauri/`
- Fails if any test fails on either OS

**2. Format Job** (`cargo fmt --check`)
- Runs on: ubuntu-latest only
- Timeout: 10 minutes
- Working directory: `src-tauri/`
- Fails if code needs formatting

**3. Clippy Job** (`cargo clippy --all-targets --all-features`)
- Runs on: ubuntu-latest only
- Timeout: 10 minutes
- Working directory: `src-tauri/`
- Fails if clippy warnings/errors exist

### Viewing Results

**In GitHub UI:**
1. Navigate to commit or pull request
2. See status checks at bottom of page
3. Green ✓ = passed, Red ✗ = failed
4. Click "Details" to view logs

**In Actions Tab:**
1. Go to repository → Actions tab
2. See all workflow runs listed
3. Click any run to see detailed logs for each job

## Local Testing Before Push

Run the same checks locally to catch issues before CI:

```bash
# Navigate to Rust project directory
cd src-tauri

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features
```

**Fix formatting issues:**
```bash
cd src-tauri
cargo fmt
```

## Expected Timing

- **Format check**: 15-30 seconds
- **Clippy**: 2-4 minutes
- **Tests (per OS)**: 2-4 minutes
- **Total CI time**: ~3-5 minutes (jobs run in parallel)

## Troubleshooting

### CI Takes Too Long (> 5 minutes)
- Check if timeout limits (10 min) are hit
- Review job logs for slow tests or compilation issues
- Consider if new dependencies increased build time

### Format Check Fails
```bash
# Fix locally
cd src-tauri
cargo fmt

# Verify
cargo fmt --check
```

### Clippy Warnings
```bash
# See all warnings locally
cd src-tauri
cargo clippy --all-targets --all-features

# Fix issues in code, then verify
cargo clippy --all-targets --all-features
```

### Tests Fail Only on macOS
- Platform-specific code may have issues
- Check logs for specific test failures
- Consider adding conditional compilation or platform-specific tests

### All Checks Pass Locally But Fail in CI
- Ensure rust-toolchain.toml specifies Rust 1.92
- Check for uncommitted changes
- Verify Cargo.toml dependencies are complete (no local paths)

## Success Criteria Check

✅ **SC-001**: CI feedback within 5 minutes  
→ Expected: 3-5 minutes, max timeout 10 minutes

✅ **SC-002**: 100% commit/PR status visibility  
→ GitHub shows status checks on all commits/PRs automatically

✅ **SC-003**: Catches issues before merge  
→ CI fails if any check fails, blocking merge if branch protection enabled

✅ **SC-004**: Identify failures within 30 seconds  
→ GitHub UI shows pass/fail status immediately, logs accessible with one click

## Related Documentation

- [Feature Specification](spec.md)
- [Implementation Plan](plan.md)
- [Research](research.md)
