# Research: GitHub Actions CI for Rust/Tauri Project

**Phase**: 0 - Outline & Research  
**Date**: 2026-01-02  
**Status**: Complete

## Research Tasks

### Task 1: GitHub Actions for Rust Projects

**Question**: What are the best practices for setting up GitHub Actions CI for Rust projects?

**Findings**:

1. **Rust Toolchain Installation**
   - Use `actions-rust-lang/setup-rust-toolchain` action (recommended) or `dtolnay/rust-toolchain` action
   - Both actions automatically read `rust-toolchain.toml` file to install correct Rust version
   - `actions-rust-lang/setup-rust-toolchain` provides better caching out of the box
   - Toolchain components (rustfmt, clippy) are installed automatically with the toolchain

2. **Cargo Commands Structure**
   - `cargo test`: Runs all tests in the project
   - `cargo fmt --check`: Checks formatting without modifying files (exits with error if formatting needed)
   - `cargo clippy --all-targets --all-features`: Runs linter on all code (lib, bins, tests, examples, benches) with all feature flags enabled

3. **Working Directory Management**
   - Use `working-directory` key in workflow steps to run commands in specific directories
   - Alternative: Use `defaults.run.working-directory` at job level for all steps in a job

4. **Multi-OS Testing**
   - Use matrix strategy with `strategy.matrix.os` to run jobs on multiple operating systems
   - Common choices: `ubuntu-latest`, `macos-latest`, `windows-latest`
   - Each OS runs as a separate job in parallel

**Decision**: Use `actions-rust-lang/setup-rust-toolchain` action for Rust setup. Configure all cargo commands with `working-directory: src-tauri`. Use matrix strategy for multi-OS testing.

**Rationale**: This approach is standard in the Rust community, provides automatic caching, and respects the existing `rust-toolchain.toml` configuration.

---

### Task 2: GitHub Actions Workflow Triggers

**Question**: What triggers should be configured for CI to run automatically?

**Findings**:

1. **Common Triggers**
   - `push`: Runs on every push to any branch
   - `pull_request`: Runs when PR is opened, synchronized (new commits), or reopened
   - Can be restricted to specific branches using `branches` filter

2. **Best Practices for CI**
   - Run on both `push` and `pull_request` events
   - Optionally exclude certain paths (e.g., documentation) using `paths-ignore`
   - For this project: No path restrictions needed as all code changes should be validated

3. **Branch Patterns**
   - `branches: ['**']` - all branches
   - Can specify patterns like `main`, `develop/*`, etc.
   - Default behavior without filters: runs on all branches

**Decision**: Configure triggers for both `push` and `pull_request` events on all branches.

**Rationale**: Ensures every code change is validated regardless of branch, providing immediate feedback to developers (FR-005, FR-006).

---

### Task 3: Job Dependencies and Workflow Structure

**Question**: Should CI jobs run in parallel or sequentially? How to structure the workflow?

**Findings**:

1. **Parallel vs Sequential**
   - Parallel: Faster feedback, better resource utilization
   - Sequential: Can save compute time by stopping early on failures
   - For small projects with fast builds: Parallel is preferred

2. **Job Independence**
   - Test, format, and clippy checks are independent
   - All three need to build the project, so no time savings from sequencing
   - Each job can fail independently, providing clear feedback

3. **Fail-Fast Strategy**
   - `fail-fast: false` - Continue all jobs even if one fails (better for seeing all issues)
   - `fail-fast: true` (default) - Cancel remaining jobs on first failure
   - For CI validation: `fail-fast: false` is better for complete feedback

**Decision**: Run all three jobs (test, format, clippy) in parallel without dependencies. Use `fail-fast: false` for test matrix.

**Rationale**: Provides fastest feedback and shows all issues at once. Developers can see all failing checks simultaneously (FR-004, SC-004).

---

### Task 4: Timeout and Performance Optimization

**Question**: How to ensure CI completes within the 5-minute target (SC-001)?

**Findings**:

1. **Typical Timings for Rust Projects**
   - Small Tauri project compilation: 2-4 minutes on GitHub Actions runners
   - Tests: 10-60 seconds (depends on test count)
   - Format check: 5-15 seconds (fast, no compilation)
   - Clippy: Similar to build time (2-4 minutes)

2. **Optimization Strategies**
   - Cargo caching: Reuse dependencies between runs (provided by `setup-rust-toolchain`)
   - Run format check early: Fast failure if formatting issues
   - Parallel jobs: Multiple checks run simultaneously
   - Shared compilation: Not possible across jobs, each builds independently

3. **Timeout Configuration**
   - `timeout-minutes` can be set per job or per step
   - Default timeout: 360 minutes (6 hours) - way too long
   - For this project: 10 minutes per job is reasonable safety margin

**Decision**: Set `timeout-minutes: 10` for each job. Rely on built-in caching from `setup-rust-toolchain` action.

**Rationale**: 10-minute timeout prevents runaway jobs while allowing sufficient time for compilation and checks. Expected actual runtime: 3-5 minutes.

---

### Task 5: Error Handling and CI Failure Reporting

**Question**: How to ensure clear error messages when CI fails (FR-009)?

**Findings**:

1. **GitHub Actions Output**
   - Cargo commands automatically print errors to stdout/stderr
   - GitHub Actions captures and displays all output in job logs
   - Failed steps are highlighted in red in the UI

2. **Cargo Error Messages**
   - `cargo test`: Shows test failures with file/line numbers
   - `cargo fmt --check`: Lists files needing formatting
   - `cargo clippy`: Shows warnings/errors with code locations and suggestions

3. **Status Checks in GitHub UI**
   - Each job appears as separate status check on commits/PRs
   - Green checkmark for pass, red X for fail
   - Click to view detailed logs
   - Summary shows which checks failed

**Decision**: Use default cargo output and GitHub Actions logging. No additional error handling needed.

**Rationale**: Cargo and GitHub Actions provide excellent error reporting out of the box. Meets FR-009 and SC-004 requirements without additional complexity.

---

## Technology Stack Summary

| Component | Choice | Version/Details |
|-----------|--------|-----------------|
| CI Platform | GitHub Actions | Native to GitHub, no setup required |
| Rust Setup | `actions-rust-lang/setup-rust-toolchain` | Automatic toolchain detection from rust-toolchain.toml |
| Rust Version | 1.92 | From existing rust-toolchain.toml |
| OS - Testing | ubuntu-latest, macOS-latest | Multi-platform validation |
| OS - Format/Clippy | ubuntu-latest | Single platform sufficient |
| Caching | Built-in (via setup-rust-toolchain) | Automatic cargo dependency caching |

## Alternatives Considered

### Alternative 1: Third-Party CI (Travis, CircleCI, etc.)

**Rejected**: GitHub Actions is native, free for public repos, and requires no external setup. Simpler to maintain.

### Alternative 2: Self-Hosted Runners

**Rejected**: Unnecessary complexity for this project. GitHub-hosted runners are sufficient and require no maintenance.

### Alternative 3: Single OS Testing Only

**Rejected**: User requirement specifies multi-OS testing for the test job to catch platform-specific issues.

### Alternative 4: Separate Workflow Files per Check

**Rejected**: Single workflow file is simpler to maintain and provides unified status view.

## Open Questions / Risks

None remaining - all research complete.

## References

- GitHub Actions Rust Quickstart: <https://github.com/actions-rust-lang/setup-rust-toolchain>
- Cargo Book - Testing: <https://doc.rust-lang.org/cargo/guide/tests.html>
- GitHub Actions Workflow Syntax: <https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions>
