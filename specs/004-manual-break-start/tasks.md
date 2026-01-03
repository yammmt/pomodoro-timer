---
description: "Task list for Manual Break Start feature implementation"
---

# Tasks: Manual Break Start

**Input**: Design documents from `/specs/004-manual-break-start/`
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/ipc-commands.md](contracts/ipc-commands.md)

**Tests**: Unit tests for timer state transitions and behavior

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Single Tauri desktop project:
- Backend: `src-tauri/src/`
- Frontend: `src/`
- Tests: `src-tauri/src/timer/tests.rs`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure development environment is ready

- [X] T001 Verify Rust 1.92+ and Tauri CLI 2.9+ installed
- [X] T002 Checkout feature branch 004-manual-break-start
- [X] T003 [P] Run cargo build to ensure clean baseline

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core timer state infrastructure changes that MUST be complete before ANY user story implementation

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Update Status enum in src-tauri/src/timer.rs to add WorkReady and BreakReady variants
- [X] T005 [P] Update serde serialization for Status enum to use camelCase (workReady, breakReady)
- [X] T006 Update TimerService::new() to initialize with Status::WorkReady instead of Status::Idle
- [X] T007 Update clear() method to return Status::WorkReady in src-tauri/src/timer.rs
- [X] T008 Update TimerState interface in src/main.ts to include new status values

**Checkpoint**: Foundation ready - Status enum expanded, serialization works, baseline state updated

---

## Phase 3: User Story 1 - Manual break initiation after work session (Priority: P1) 🎯 MVP

**Goal**: Work completion transitions to break-ready state without auto-starting countdown. User must manually start break.

**Independent Test**: Start work session, wait for completion, verify chime plays and 5:00 displays without countdown starting. Press Start to begin break countdown.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T009 [P] [US1] Add test for work completion → BreakReady transition in src-tauri/src/timer/tests.rs
- [X] T010 [P] [US1] Add test for completion_flag set on work completion in src-tauri/src/timer/tests.rs
- [X] T011 [P] [US1] Add test for start from BreakReady begins break countdown in src-tauri/src/timer/tests.rs

### Implementation for User Story 1

- [X] T012 [US1] Modify handle_completion() for Work phase: transition to BreakReady instead of auto-starting in src-tauri/src/timer.rs
- [X] T013 [US1] Set completion_flag=true and state_label in handle_completion() for Work phase in src-tauri/src/timer.rs
- [X] T014 [US1] Update start() method to detect BreakReady status and start break countdown in src-tauri/src/timer.rs
- [X] T015 [US1] Ensure start() clears completion_flag when starting from BreakReady in src-tauri/src/timer.rs
- [X] T016 [US1] Update frontend button logic to enable Start when status is breakReady in src/main.ts
- [X] T017 [US1] Verify chime triggers on completionFlag detection in src/main.ts (already implemented, just verify)

**Checkpoint**: Work session completion now transitions to BreakReady, manual Start required, chime plays

---

## Phase 4: User Story 2 - Pause and resume break session (Priority: P2)

**Goal**: Break sessions support pause and resume functionality, consistent with work sessions.

**Independent Test**: Start break countdown, press Pause, verify countdown stops. Press Resume, verify countdown continues from paused time.

### Tests for User Story 2

- [X] T018 [P] [US2] Add test for pause during break countdown in src-tauri/src/timer/tests.rs
- [X] T019 [P] [US2] Add test for resume from paused break in src-tauri/src/timer/tests.rs
- [X] T020 [P] [US2] Add test for pause preserving remaining time in break in src-tauri/src/timer/tests.rs

### Implementation for User Story 2

- [X] T021 [US2] Verify pause() method works correctly when phase=Break in src-tauri/src/timer.rs
- [X] T022 [US2] Verify resume() method works correctly when phase=Break in src-tauri/src/timer.rs
- [X] T023 [US2] Update state_label for paused break state in src-tauri/src/timer.rs
- [X] T024 [US2] Verify frontend Pause/Resume buttons work with break status in src/main.ts

**Checkpoint**: Pause and Resume controls work identically for both work and break sessions

---

## Phase 5: User Story 3 - Break completion and cycle restart (Priority: P2)

**Goal**: Break completion transitions to work-ready state without auto-starting. Chime plays, 25:00 displays, user starts next work session manually.

**Independent Test**: Start and complete break session, verify chime plays and 25:00 displays without countdown starting. Press Start to begin new work session.

### Tests for User Story 3

- [X] T025 [P] [US3] Add test for break completion → WorkReady transition in src-tauri/src/timer/tests.rs
- [X] T026 [P] [US3] Add test for completion_flag set on break completion in src-tauri/src/timer/tests.rs
- [X] T027 [P] [US3] Add test for start from WorkReady after break begins work countdown in src-tauri/src/timer/tests.rs

### Implementation for User Story 3

- [X] T028 [US3] Modify handle_completion() for Break phase: transition to WorkReady instead of Complete in src-tauri/src/timer.rs
- [X] T029 [US3] Set completion_flag=true and state_label in handle_completion() for Break phase in src-tauri/src/timer.rs
- [X] T030 [US3] Update start() method to handle starting from WorkReady after break completion in src-tauri/src/timer.rs
- [X] T031 [US3] Update frontend button logic to enable Start when status is workReady in src/main.ts
- [X] T032 [US3] Verify chime triggers on break completion via completionFlag in src/main.ts

**Checkpoint**: Full work-break-work cycle complete, all transitions require manual Start press

---

## Phase 6: User Story 4 - Clear during break ready state (Priority: P3)

**Goal**: Clear button skips break and returns to work-ready state from break-ready state.

**Independent Test**: Complete work session to reach BreakReady, press Clear, verify display shows 25:00 work mode.

### Tests for User Story 4

- [X] T033 [P] [US4] Add test for clear from BreakReady returns to WorkReady in src-tauri/src/timer/tests.rs
- [X] T034 [P] [US4] Add test for clear from running break returns to WorkReady in src-tauri/src/timer/tests.rs
- [X] T035 [P] [US4] Add test for clear from paused break returns to WorkReady in src-tauri/src/timer/tests.rs

### Implementation for User Story 4

- [X] T036 [US4] Verify clear() method works correctly from BreakReady status in src-tauri/src/timer.rs
- [X] T037 [US4] Verify clear() method works correctly from Break/Running status in src-tauri/src/timer.rs
- [X] T038 [US4] Verify clear() method works correctly from Break/Paused status in src-tauri/src/timer.rs
- [X] T039 [US4] Verify frontend Clear button works from all break-related states in src/main.ts

**Checkpoint**: Clear provides flexibility to skip breaks or reset from any state

---

## Phase 7: Edge Cases & Polish

**Purpose**: Handle edge cases and improve user experience

- [X] T040 [P] Add test for Start pressed while Running returns error in src-tauri/src/timer/tests.rs
- [X] T041 [P] Verify pause near session end (e.g., at 1 second) doesn't skip auto-transition in src-tauri/src/timer/tests.rs
- [X] T042 [P] Add test for Ready states maintained indefinitely without timeout in src-tauri/src/timer/tests.rs
- [X] T043 Update state_label strings for clarity ("Break ready - press Start") in src-tauri/src/timer.rs
- [X] T044 Remove old work→break auto-start chime detection (line ~87-89) in src/main.ts
- [X] T045 [P] Run cargo fmt to format Rust code
- [X] T046 [P] Run cargo clippy to check for issues
- [X] T047 Run all tests with cargo test and verify 100% pass
- [X] T048 Manual testing: Complete full work→break→work cycle
- [X] T049 Manual testing: Test pause/resume during break
- [X] T050 Manual testing: Test Clear from various states
- [X] T051 Manual testing: Verify chimes play at correct times
- [X] T052 Update any outdated comments or documentation in code

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational (Phase 2) completion
  - User stories can then proceed in priority order (P1 → P2 → P2 → P3)
  - Or P2 stories can be worked in parallel if desired
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - MVP baseline
- **User Story 2 (P2)**: Can start after US1 (needs break countdown to exist) - Could run parallel with US3
- **User Story 3 (P2)**: Can start after US1 (needs break state transitions) - Could run parallel with US2
- **User Story 4 (P3)**: Can start after US1 (needs BreakReady state to exist)

### Within Each User Story

1. Tests MUST be written FIRST and FAIL before implementation
2. Backend state machine changes before frontend updates
3. Core implementation before edge case handling
4. Story complete and tested before moving to next priority

### Parallel Opportunities

Within Phase 2 (Foundational):
- T005 (serialization) can run parallel with T004 (enum update) once Status enum defined
- T008 (frontend interface) can run parallel with backend tasks

Within Each User Story:
- All test tasks marked [P] can run in parallel
- Frontend updates can run parallel with backend verification tasks

Between User Stories:
- US2 and US3 (both P2) can be worked in parallel by different developers
- Edge case tests (Phase 7) marked [P] can all run in parallel

---

## Parallel Example: User Story 1

```bash
# Write all tests in parallel
git checkout -b us1-tests
# Developer A writes T009
# Developer B writes T010  
# Developer C writes T011
cargo test  # All should FAIL

# Then implement serially (state machine changes)
git checkout -b us1-impl
# T012: Modify handle_completion (core change)
# T013: Set flags (builds on T012)
# T014: Update start() (builds on T012)
# T015: Clear flag (builds on T014)

# Then frontend in parallel
# T016 and T017 can run in parallel
cargo test  # All should PASS
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

For fastest value delivery, implement only:
- Phase 1 (Setup): T001-T003
- Phase 2 (Foundational): T004-T008
- Phase 3 (User Story 1): T009-T017

This delivers the core feature: manual break initiation after work completion.

### Full Feature (All Stories)

Implement all phases sequentially:
1. Setup → Foundational (blocking)
2. US1 (P1) - Core manual break feature
3. US2 (P2) - Pause/resume consistency
4. US3 (P2) - Break completion behavior
5. US4 (P3) - Clear flexibility
6. Polish - Edge cases and cleanup

### Recommended Task Order

```
1. T001-T003 (Setup)
2. T004-T008 (Foundation) - MUST complete before user stories
3. T009-T011 (US1 Tests) → T012-T017 (US1 Implementation)
4. T018-T020 (US2 Tests) → T021-T024 (US2 Implementation)
5. T025-T027 (US3 Tests) → T028-T032 (US3 Implementation)
6. T033-T035 (US4 Tests) → T036-T039 (US4 Implementation)
7. T040-T052 (Polish)
```

### Testing Strategy

- Write tests first (TDD approach)
- Run `cargo test` after each task group
- Manual testing after each user story phase
- Full integration testing in Phase 7

---

## Task Summary

**Total Tasks**: 52
- Setup: 3 tasks
- Foundational: 5 tasks (BLOCKING)
- User Story 1 (P1): 9 tasks (3 tests + 6 implementation)
- User Story 2 (P2): 7 tasks (3 tests + 4 implementation)
- User Story 3 (P2): 8 tasks (3 tests + 5 implementation)
- User Story 4 (P3): 7 tasks (3 tests + 4 implementation)
- Polish: 13 tasks

**Parallel Opportunities**: 15 tasks marked [P] can run in parallel within their phases

**Independent Test Criteria**:
- US1: Can be fully tested by starting work, completing, and manually starting break
- US2: Can be tested by pausing/resuming break countdown independently
- US3: Can be tested by completing break and manually starting work
- US4: Can be tested by clearing from break-ready state

**Suggested MVP Scope**: Phase 1-3 (18 tasks) delivers core manual break feature

---

## Validation Checklist

Before marking feature complete:

- [ ] All cargo tests pass (100% pass rate)
- [ ] Manual test: Work completion → chime + 5:00 display (no auto-start)
- [ ] Manual test: Manual break start → countdown begins
- [ ] Manual test: Break pause/resume works correctly
- [ ] Manual test: Break completion → chime + 25:00 display (no auto-start)
- [ ] Manual test: Manual work start after break → countdown begins
- [ ] Manual test: Clear from BreakReady → returns to WorkReady
- [ ] Manual test: Clear from running break → returns to WorkReady
- [ ] Manual test: Full cycle work→break→work without issues
- [ ] Code formatted with cargo fmt
- [ ] No clippy warnings with cargo clippy
- [ ] State labels clear and informative
- [ ] Chimes play at correct times (work complete, break complete)
- [ ] No auto-start transitions anywhere in the cycle

---

## References

- Feature Specification: [spec.md](spec.md)
- Implementation Plan: [plan.md](plan.md)
- Research & Decisions: [research.md](research.md)
- Data Model & State Machine: [data-model.md](data-model.md)
- API Contract: [contracts/ipc-commands.md](contracts/ipc-commands.md)
- Developer Guide: [quickstart.md](quickstart.md)
