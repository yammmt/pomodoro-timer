---
description: "Task list for GitHub Actions CI implementation"
---

# Tasks: GitHub Actions CI

**Input**: Design documents from `/specs/002-github-actions-ci/`
**Prerequisites**: plan.md, spec.md, research.md, contracts/workflow-contract.md, quickstart.md

**Tests**: No test tasks included - this is infrastructure configuration

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup

**Purpose**: Create directory structure for GitHub Actions workflow

- [X] T001 Create `.github/workflows/` directory in repository root

---

## Phase 2: Foundational

**Purpose**: Core workflow configuration that enables all three user stories

**⚠️ CRITICAL**: This phase creates the workflow file structure. All user stories depend on this foundation.

- [X] T002 Create `.github/workflows/ci.yml` with workflow name and triggers (push, pull_request on all branches)

**Checkpoint**: Workflow file exists and triggers are configured - ready for job definitions

---

## Phase 3: User Story 1 - Automated Testing on Code Changes (Priority: P1) 🎯 MVP

**Goal**: Run automated tests on every code change to catch bugs early. Tests run on both Ubuntu and macOS for cross-platform validation.

**Independent Test**: Push a commit to any branch and verify that test job runs automatically on both platforms, reporting pass/fail status clearly.

### Implementation for User Story 1

- [X] T003 [US1] Add `test` job configuration in .github/workflows/ci.yml with matrix strategy for ubuntu-latest and macOS-latest
- [X] T004 [US1] Add checkout step (actions/checkout@v4) to test job in .github/workflows/ci.yml
- [X] T005 [US1] Add Rust toolchain setup step (actions-rust-lang/setup-rust-toolchain@v1) to test job in .github/workflows/ci.yml
- [X] T006 [US1] Add cargo test execution step with working-directory: src-tauri in .github/workflows/ci.yml
- [X] T007 [US1] Add timeout-minutes: 10 to test job configuration in .github/workflows/ci.yml
- [X] T008 [US1] Add fail-fast: false to test job matrix strategy in .github/workflows/ci.yml

**Checkpoint**: Test job is complete. Push a commit to verify tests run on both Ubuntu and macOS, and results appear on GitHub commit/PR UI.

---

## Phase 4: User Story 2 - Code Formatting Verification (Priority: P2)

**Goal**: Automatically check code formatting on every push to maintain consistent style without manual reviews.

**Independent Test**: Push code with formatting issues (run `cargo fmt` locally first to see differences, then revert) and verify CI fails with clear formatting error messages.

### Implementation for User Story 2

- [X] T009 [P] [US2] Add `format` job configuration in .github/workflows/ci.yml with runs-on: ubuntu-latest
- [X] T010 [P] [US2] Add checkout step (actions/checkout@v4) to format job in .github/workflows/ci.yml
- [X] T011 [P] [US2] Add Rust toolchain setup step (actions-rust-lang/setup-rust-toolchain@v1) to format job in .github/workflows/ci.yml
- [X] T012 [P] [US2] Add cargo fmt --check execution step with working-directory: src-tauri in .github/workflows/ci.yml
- [X] T013 [P] [US2] Add timeout-minutes: 10 to format job configuration in .github/workflows/ci.yml

**Checkpoint**: Format job is complete. Test by introducing formatting issues and verifying CI catches them.

---

## Phase 5: User Story 3 - Linting and Best Practices Enforcement (Priority: P3)

**Goal**: Automatically check for code quality issues and enforce Rust best practices on every push.

**Independent Test**: Push code with clippy warnings (e.g., unused variables, unnecessary clones) and verify CI reports the issues clearly.

### Implementation for User Story 3

- [X] T014 [P] [US3] Add `clippy` job configuration in .github/workflows/ci.yml with runs-on: ubuntu-latest
- [X] T015 [P] [US3] Add checkout step (actions/checkout@v4) to clippy job in .github/workflows/ci.yml
- [X] T016 [P] [US3] Add Rust toolchain setup step (actions-rust-lang/setup-rust-toolchain@v1) to clippy job in .github/workflows/ci.yml
- [X] T017 [P] [US3] Add cargo clippy --all-targets --all-features execution step with working-directory: src-tauri in .github/workflows/ci.yml
- [X] T018 [P] [US3] Add timeout-minutes: 10 to clippy job configuration in .github/workflows/ci.yml

**Checkpoint**: Clippy job is complete. All three user stories (test, format, clippy) should now work independently and in parallel.

---

## Phase 6: Polish & Validation

**Purpose**: Final verification and documentation

- [X] T019 [P] Verify workflow syntax is valid (push to branch and check Actions tab, or use `actionlint` if available)
- [X] T020 [P] Test complete workflow by pushing a clean commit - verify all three jobs pass
- [ ] T021 [P] Test failure scenarios: introduce test failure, verify CI fails appropriately (requires push to GitHub)
- [ ] T022 [P] Test failure scenarios: introduce formatting issue, verify format job fails (requires push to GitHub)
- [ ] T023 [P] Test failure scenarios: introduce clippy warning, verify clippy job fails (requires push to GitHub)
- [X] T024 Update quickstart.md if any actual implementation differs from design
- [X] T025 Verify all success criteria from spec.md are met (SC-001 through SC-004)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) - creates base workflow file
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) - adds test job to workflow
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) - adds format job to workflow
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2) - adds clippy job to workflow
- **Polish (Phase 6)**: Depends on all user stories (Phases 3-5) being complete

### User Story Dependencies

- **User Story 1 (P1 - Test)**: Can start immediately after Foundational phase - No dependencies on other stories
- **User Story 2 (P2 - Format)**: Can start immediately after Foundational phase - Independent of US1
- **User Story 3 (P3 - Clippy)**: Can start immediately after Foundational phase - Independent of US1 and US2

**Key Insight**: Once the foundational workflow file exists (Phase 2), all three user stories can be implemented in parallel since they add independent jobs to the same file. However, to minimize merge conflicts when editing the same file, implement sequentially in priority order: P1 → P2 → P3.

### Within Each User Story

Each user story follows this pattern:

1. Add job configuration (name, runs-on, timeout, strategy if applicable)
2. Add checkout step
3. Add Rust setup step
4. Add cargo command execution step

All tasks within a user story should be completed in sequence to build a complete, working job.

### Parallel Opportunities

**Within Same File** (Limited):

- If multiple developers, Phase 4 (US2) and Phase 5 (US3) CAN be done in parallel by carefully coordinating to avoid merge conflicts in ci.yml
- More practically: implement in sequence (P1 → P2 → P3) to avoid conflicts

**Different Files** (Full Parallelism):

- Polish tasks (T019-T025) marked [P] can all run in parallel once user stories are complete
- Testing different failure scenarios can happen concurrently

**Recommended Approach**:

- Implement user stories sequentially (T003-T008, then T009-T013, then T014-T018) to avoid merge conflicts
- Run all polish/validation tasks in parallel at the end

---

## Parallel Example: After Foundational Phase

If you have multiple team members and want to parallelize:

```bash
# Developer 1: User Story 1 (Test Job)
# Implements T003-T008 in their branch

# Developer 2: User Story 2 (Format Job)  
# Implements T009-T013 in their branch

# Developer 3: User Story 3 (Clippy Job)
# Implements T014-T018 in their branch

# Merge order: US1 → US2 → US3 (to minimize conflicts)
```

However, for a single developer or to avoid merge conflicts, sequential execution is recommended:

1. Complete Phase 3 (US1) entirely
2. Complete Phase 4 (US2) entirely
3. Complete Phase 5 (US3) entirely
4. Run all Phase 6 tasks in parallel

---

## Implementation Strategy

### MVP First (Phase 3 Only)

The minimum viable product is **User Story 1 (Test Job)** only:

- Provides immediate value: catching test failures automatically
- Can be deployed and used independently
- Phases 1, 2, and 3 = ~8 tasks for working test automation

### Incremental Delivery

After MVP:

- **Phase 4 (US2)**: Add format checking - 5 tasks
- **Phase 5 (US3)**: Add clippy linting - 5 tasks
- Each phase adds independent value without breaking existing functionality

### Validation Checkpoints

After each user story phase:

1. Push a commit to trigger the workflow
2. Check GitHub Actions tab to see job results
3. Verify status appears on commits/PRs
4. Test failure scenarios to ensure errors are caught

---

## Task Summary

- **Total Tasks**: 25
- **Phase 1 (Setup)**: 1 task
- **Phase 2 (Foundational)**: 1 task
- **Phase 3 (US1 - Test)**: 6 tasks - MVP
- **Phase 4 (US2 - Format)**: 5 tasks
- **Phase 5 (US3 - Clippy)**: 5 tasks
- **Phase 6 (Polish)**: 7 tasks

**Parallel Opportunities**:

- Phases 4 and 5 can run in parallel (with coordination)
- Phase 6 tasks are mostly independent (5 tasks can run in parallel)

**Estimated MVP Completion**: Phases 1-3 (8 tasks) delivers working test automation

**Format Validation**: ✅ All tasks follow checklist format with:

- Checkbox: `- [ ]`
- Task ID: T001-T025 (sequential)
- [P] marker: Used for truly parallelizable tasks
- [Story] label: US1, US2, US3 for user story phases
- File paths: All tasks include exact file paths
