# Tasks: Show Elapsed Time After Session Completion

**Feature**: 009-show-elapsed-time  
**Branch**: `009-show-elapsed-time`  
**Input**: Design documents from `/specs/009-show-elapsed-time/`

**Organization**: Tasks organized by user story (US1, US2) to enable independent implementation and testing.

## Format: `- [ ] [ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1=Work Session Overtime, US2=Break Session Overtime)
- Exact file paths included in descriptions

---

## Phase 1: Setup

**Purpose**: Prepare for implementation

- [ ] T001 Review existing timer state machine in `src-tauri/src/timer.rs`
- [ ] T002 [P] Review frontend timer display logic in `src/main.ts`
- [ ] T003 [P] Verify test framework setup with `cargo test`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core overtime infrastructure needed for BOTH user stories

**⚠️ CRITICAL**: Complete this phase before implementing any user story

- [ ] T004 Add `completed_at: Option<Instant>` field to `TimerService` in `src-tauri/src/timer.rs`
- [ ] T005 Add `overtime_secs: Option<u32>` field to `TimerState` struct in `src-tauri/src/timer.rs`
- [ ] T006 Update `TimerService::new()` constructor to initialize `completed_at = None` in `src-tauri/src/timer.rs`
- [ ] T007 Add `overtime_secs` field to `TimerState` TypeScript interface in `src/main.ts`
- [ ] T008 Add CSS `.overtime { color: #dc2626; }` style rule in `src/index.html` or separate CSS file

**Checkpoint**: Foundation ready - user story implementation can begin

---

## Phase 3: User Story 1 - Work Session Overtime Display (Priority: P1) 🎯 MVP

**Goal**: Display elapsed overtime when work session (25:00) completes, showing "-MM:SS" in red, updating every second, capped at 59:59

**Independent Test**: Complete work session, wait 30 seconds without action, verify display shows "-00:30" in red

### Implementation for User Story 1

- [ ] T009 [US1] Modify `handle_completion()` to set `completed_at = Some(Instant::now())` in `src-tauri/src/timer.rs`
- [ ] T010 [US1] Modify `get_state()` to calculate `overtime_secs` from `completed_at` when `status == Complete` in `src-tauri/src/timer.rs`
- [ ] T011 [US1] Add overtime cap logic: `min(elapsed_secs, 3599)` in `get_state()` in `src-tauri/src/timer.rs`
- [ ] T012 [US1] Update `start()` to clear `completed_at = None` in `src-tauri/src/timer.rs`
- [ ] T013 [US1] Update `resume()` to clear `completed_at = None` in `src-tauri/src/timer.rs`
- [ ] T014 [US1] Update `clear()` to clear `completed_at = None` in `src-tauri/src/timer.rs`
- [ ] T015 [US1] Update `set_phase()` to clear `completed_at = None` in `src-tauri/src/timer.rs`
- [ ] T016 [US1] Modify `updateUI()` to check for `state.overtimeSecs` presence in `src/main.ts`
- [ ] T017 [US1] Add overtime display logic: set `textContent` to `` `-${formatTime(overtimeSecs)}` `` in `src/main.ts`
- [ ] T018 [US1] Add CSS class toggle: `classList.add('overtime')` when overtime active in `src/main.ts`
- [ ] T019 [US1] Remove CSS class: `classList.remove('overtime')` when overtime cleared in `src/main.ts`

### Testing for User Story 1

- [ ] T020 [P] [US1] Add unit test: `test_overtime_displayed_after_work_completion` in `src-tauri/src/timer/tests.rs`
- [ ] T021 [P] [US1] Add unit test: `test_overtime_capped_at_59_59` in `src-tauri/src/timer/tests.rs`
- [ ] T022 [P] [US1] Add unit test: `test_overtime_cleared_on_start` in `src-tauri/src/timer/tests.rs`
- [ ] T023 [P] [US1] Add unit test: `test_overtime_cleared_on_resume` in `src-tauri/src/timer/tests.rs`
- [ ] T024 [P] [US1] Add unit test: `test_overtime_cleared_on_clear` in `src-tauri/src/timer/tests.rs`
- [ ] T025 [P] [US1] Add unit test: `test_overtime_cleared_on_phase_switch` in `src-tauri/src/timer/tests.rs`
- [ ] T026 [US1] Manual test: Start work timer, let it complete, verify "-00:01" appears in red
- [ ] T027 [US1] Manual test: Let overtime reach "-01:00", verify display continues updating
- [ ] T028 [US1] Manual test: Click Start during overtime, verify display returns to "25:00" without red
- [ ] T029 [US1] Manual test: Click Clear during overtime, verify display returns to "25:00"

**Checkpoint**: Work session overtime fully functional and tested

---

## Phase 4: User Story 2 - Break Session Overtime Display (Priority: P2)

**Goal**: Display elapsed overtime when break session (05:00) completes, providing consistency with work session behavior

**Independent Test**: Complete break session, wait 1 minute without action, verify display shows "-01:00" in red

### Implementation for User Story 2

**Note**: Most implementation already complete from US1. This phase verifies break session support.

- [ ] T030 [US2] Verify `handle_completion()` captures `completed_at` for both work AND break phases in `src-tauri/src/timer.rs`
- [ ] T031 [US2] Verify `get_state()` calculates `overtime_secs` correctly when `phase = Break` in `src-tauri/src/timer.rs`
- [ ] T032 [US2] Verify frontend overtime display works for break phase (no phase-specific logic needed) in `src/main.ts`

### Testing for User Story 2

- [ ] T033 [P] [US2] Add unit test: `test_overtime_displayed_after_break_completion` in `src-tauri/src/timer/tests.rs`
- [ ] T034 [P] [US2] Add unit test: `test_overtime_break_cleared_on_start` in `src-tauri/src/timer/tests.rs`
- [ ] T035 [US2] Manual test: Start break timer, let it complete, verify "-00:01" appears in red
- [ ] T036 [US2] Manual test: Let break overtime reach "-02:30", verify display continues updating
- [ ] T037 [US2] Manual test: Click Start during break overtime, verify new work session starts with "25:00"
- [ ] T038 [US2] Manual test: Switch to Work mode during break overtime, verify display shows "25:00" without red

**Checkpoint**: Both work and break session overtime fully functional

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and edge case handling

- [ ] T039 Run `cargo fmt` to format Rust code in `src-tauri/`
- [ ] T040 Run `cargo clippy` for linting checks in `src-tauri/`
- [ ] T041 Run `cargo test` to verify all tests pass in `src-tauri/`
- [ ] T042 [P] Manual test: Minimize app during overtime, restore, verify display shows correct elapsed time
- [ ] T043 [P] Manual test: Let overtime run to `"-59:59"`, verify it stays capped (doesn't go to `"-60:00"`)
- [ ] T044 Review `quickstart.md` verification checklist and confirm all items complete
- [ ] T045 Update `implementation-report.md` with completion status

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Can start in parallel with US1 after Foundational, but benefits from US1 completion for verification
- **Polish (Phase 5)**: Depends on US1 and US2 completion

### User Story Dependencies

- **US1 (Work Overtime)**: Independent - can implement standalone after Foundational
- **US2 (Break Overtime)**: Mostly inherits from US1 - primarily verification tasks

### Task Dependencies Within Phases

**Phase 2 (Foundational)**:

- T004-T006 can be done together (backend struct changes)
- T007-T008 can be done in parallel (frontend setup)

**Phase 3 (US1)**:

- T009-T011 are sequential (backend overtime tracking logic)
- T012-T015 can be done together (clear overtime logic)
- T016-T019 are sequential (frontend display logic)
- T020-T025 can all run in parallel (unit tests)
- T026-T029 are sequential manual tests

**Phase 4 (US2)**:

- T030-T032 are verification tasks (can be quick)
- T033-T034 can run in parallel (unit tests)
- T035-T038 are sequential manual tests

**Phase 5 (Polish)**:

- T039-T041 should run sequentially
- T042-T043 can run in parallel
- T044-T045 are final documentation tasks

### Parallel Opportunities

**Within Foundational Phase**:

```bash
# Frontend and backend setup can happen simultaneously
Terminal 1: cd src-tauri && # work on T004-T006
Terminal 2: # work on T007-T008 in `src/`
```

**Within US1 Testing**:

```bash
# All unit tests can run in parallel
Terminal 1: cd src-tauri && cargo test test_overtime_displayed_after_work_completion
Terminal 2: cd src-tauri && cargo test test_overtime_capped_at_59_59
Terminal 3: cd src-tauri && cargo test test_overtime_cleared_on_start
# etc.
```

**US1 vs US2**:

```bash
# If team has capacity, US2 verification can start while US1 testing is in progress
Terminal 1: # Working on US1 manual tests (T026-T029)
Terminal 2: # Working on US2 verification (T030-T032)
```

---

## Implementation Strategy

### Recommended Approach: Sequential MVP-First

1. **Phase 1-2**: Setup and Foundational (1-2 hours)
2. **Phase 3**: Complete US1 including all tests (3-4 hours) - **This is MVP**
3. **Phase 4**: Verify US2 (1 hour) - Extension for consistency
4. **Phase 5**: Polish and final checks (1 hour)

**Total Estimated Time**: 6-8 hours for complete implementation

### MVP Delivery Point

After **Phase 3 (US1)** completion:

- Work session overtime display fully functional
- All unit tests passing
- Manual verification complete
- Feature delivers core value (immediate feedback on work session completion)

Phase 4 (US2) adds consistency for break sessions but is not critical for MVP.

---

## Task Count Summary

- **Setup**: 3 tasks
- **Foundational**: 5 tasks (BLOCKING)
- **User Story 1 (P1)**: 21 tasks (11 implementation + 10 testing)
- **User Story 2 (P2)**: 9 tasks (3 implementation + 6 testing)
- **Polish**: 7 tasks

**Total**: 45 tasks

**Parallel Opportunities**: 12 tasks marked [P] can run in parallel with other tasks in same phase

**Independent Test Criteria**:

- US1: Complete work session → wait 30s → see "-00:30" in red
- US2: Complete break session → wait 1 min → see "-01:00" in red
