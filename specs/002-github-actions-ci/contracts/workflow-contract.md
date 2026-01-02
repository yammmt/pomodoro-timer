# CI Workflow Contract

**Type**: GitHub Actions Workflow Configuration  
**Format**: YAML  
**Location**: `.github/workflows/ci.yml`

## Overview

This document defines the expected structure and behavior of the GitHub Actions CI workflow. While not a traditional API contract, it serves as a specification for the workflow configuration.

## Workflow Definition

### Metadata

```yaml
name: CI
```

### Triggers

```yaml
on:
  push:
    branches: ['**']
  pull_request:
    branches: ['**']
```

**Behavior**:
- Activates on any push to any branch
- Activates on any pull request to any branch
- Multiple concurrent runs possible (one per push/PR)

## Jobs Contract

### Job: `test`

**Purpose**: Run automated tests on multiple platforms

**Configuration**:
```yaml
runs-on: ${{ matrix.os }}
timeout-minutes: 10
strategy:
  fail-fast: false
  matrix:
    os: [ubuntu-latest, macOS-latest]
```

**Steps**:
1. Checkout code (actions/checkout@v4)
2. Setup Rust toolchain (actions-rust-lang/setup-rust-toolchain@v1)
3. Run tests: `cargo test` in `src-tauri/`

**Success Criteria**: Exit code 0 from cargo test on BOTH OS platforms

**Failure Conditions**:
- Any test fails
- Compilation fails
- Timeout exceeded (10 minutes)

---

### Job: `format`

**Purpose**: Verify code formatting consistency

**Configuration**:
```yaml
runs-on: ubuntu-latest
timeout-minutes: 10
```

**Steps**:
1. Checkout code
2. Setup Rust toolchain
3. Check formatting: `cargo fmt --check` in `src-tauri/`

**Success Criteria**: Exit code 0 (no formatting changes needed)

**Failure Conditions**:
- Code requires formatting (cargo fmt would make changes)
- Timeout exceeded (10 minutes)

---

### Job: `clippy`

**Purpose**: Enforce Rust linting and best practices

**Configuration**:
```yaml
runs-on: ubuntu-latest
timeout-minutes: 10
```

**Steps**:
1. Checkout code
2. Setup Rust toolchain
3. Run linter: `cargo clippy --all-targets --all-features` in `src-tauri/`

**Success Criteria**: Exit code 0 (no clippy warnings/errors)

**Failure Conditions**:
- Clippy warnings or errors detected
- Compilation fails
- Timeout exceeded (10 minutes)

## Status Reporting

### GitHub Checks API

Each job reports status via GitHub Checks API:

**Status Values**:
- `queued`: Job is waiting to run
- `in_progress`: Job is currently executing
- `completed`: Job finished (check conclusion for result)

**Conclusion Values** (when status = completed):
- `success`: Job passed all checks
- `failure`: Job encountered errors
- `cancelled`: Job was cancelled
- `timed_out`: Job exceeded timeout limit

### Commit/PR Status

GitHub UI displays:
- ✓ Green checkmark: All jobs succeeded
- ✗ Red X: One or more jobs failed
- ⚠ Yellow warning: Jobs cancelled or skipped
- 🔵 Blue circle: Jobs in progress

## Required Permissions

**Repository Permissions**:
- `contents: read` - Read source code
- `checks: write` - Report status checks (automatic for GitHub Actions)

**No Secrets Required**: All checks run on public source code without authentication.

## External Dependencies

1. **GitHub Actions Infrastructure**
   - Hosted runners (ubuntu-latest, macOS-latest)
   - Minimum runner specs: 2-core CPU, 7GB RAM, 14GB SSD

2. **Rust Ecosystem**
   - crates.io (dependency download)
   - Rust toolchain distribution (via rustup)

3. **Project Files**
   - `rust-toolchain.toml` - Rust version specification
   - `src-tauri/Cargo.toml` - Dependency manifest
   - `src-tauri/Cargo.lock` - Locked dependency versions

## Backward Compatibility

**Breaking Changes**:
- Changing Rust version in rust-toolchain.toml may cause CI failures
- Adding new dependencies may increase build time
- Changing clippy lint levels may cause new failures

**Non-Breaking Changes**:
- Adding new tests (must pass)
- Refactoring code (must pass all checks)
- Updating dependencies (if tests still pass)

## Performance SLA

Based on research (see [research.md](../research.md)):

| Metric | Target | Maximum |
|--------|--------|---------|
| Format check | 30 seconds | 2 minutes |
| Clippy (per OS) | 4 minutes | 10 minutes |
| Tests (per OS) | 4 minutes | 10 minutes |
| Total workflow | 5 minutes | 10 minutes |

**Note**: Jobs run in parallel, so total time ≈ slowest job time.

## Error Codes

Cargo standard exit codes:
- `0`: Success
- `101`: Compilation or test failure
- Other non-zero: Various error conditions

GitHub Actions interprets any non-zero exit code as job failure.
