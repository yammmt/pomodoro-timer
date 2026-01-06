# Tasks: Show Elapsed Time After Session Completion

**Input**: Design documents from `/specs/007-show-elapsed-time/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/ipc-commands.md, quickstart.md

**Organization**: Tasks grouped by user story (P1 core display, P2 pause/resume, P1 clear) to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare backend and frontend foundation for elapsed time feature

- [ ] T001 Create feature branch and verify project structure (already on `007-show-elapsed-time`)
- [ ] T002 Review existing timer state and IPC commands in `src-tauri/src/timer.rs` and `src-tauri/src/lib.rs`
- [ ] T003 Verify test framework (`cargo test`) is working and understand existing timer tests in `src-tauri/src/timer/tests.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core backend elapsed time infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No UI work can begin until this phase is complete

- [ ] T004 Extend `TimerService` struct in `src-tauri/src/timer.rs` with elapsed tracking fields: `last_completed_phase: Option<Phase>`, `elapsed_started_instant: Option<Instant>`, `elapsed_paused_secs: u32`, `elapsed_running: bool`
- [ ] T005 Update `TimerService::new()` constructor to initialize elapsed fields to default/None values
- [ ] T006 Modify `handle_completion()` in `src-tauri/src/timer.rs` to transition to `Status::Complete`, start elapsed clock (`elapsed_running=true`, `elapsed_started_instant=now`), and record `last_completed_phase`
- [ ] T007 Add elapsed computation logic in `get_state()` to calculate `elapsed_secs` based on elapsed running/paused state
- [ ] T008 Update `TimerState` struct in `src-tauri/src/timer.rs` with new fields: `elapsedSecs: Option<u32>`, `elapsedRunning: bool`, `lastCompletedPhase: Option<Phase>`
- [ ] T009 Modify `start()` method to reject calls when `status == Status::Complete` with appropriate error message
- [ ] T010 Update `clear()` method to reset all elapsed fields when `status == Status::Complete`
- [ ] T011 Verify Tauri serde serialization exports new fields in IPC responses via `get_state` command

**Checkpoint**: Backend elapsed time infrastructure complete; ready for user story implementations

---

## Phase 3: User Story 1 - View Elapsed Time After Session Ends (Priority: P1) 🎯 MVP

**Goal**: Display elapsed time in "-MM:SS" format with bright red color after a session completes

**Independent Test**: Complete a work or break session; verify display transitions to "-00:01" in red, increments each second, and continues until paused or cleared

### Backend Implementation for User Story 1

- [ ] T012 Implement elapsed time calculation in `get_state()` when `status == Status::Complete`, computing real elapsed from `elapsed_paused_secs + (now - elapsed_started_instant)` when running
- [ ] T013 Add Rust unit test in `src-tauri/src/timer/tests.rs` for completion transition: verify `status=Complete`, `elapsedSecs` starts at ~1s, `completionFlag=true`, `elapsedRunning=true`
- [ ] T014 Add Rust unit test for elapsed incrementing: verify `get_state()` called at 1s intervals shows monotonic increase (0s → 1s → 2s)
- [ ] T015 Add Rust unit test for elapsed cap at 99:59: verify elapsed never displays beyond max display time

### Frontend Implementation for User Story 1

- [ ] T016 [P] [US1] Add CSS styling for elapsed red text in `src/index.html` using `#ef4444` color with bold weight
- [ ] T017 [P] [US1] Add CSS class for elapsed display state (e.g., `.elapsed-running`) in `src/index.html`
- [ ] T018 [US1] Update `updateUI()` function in `src/main.ts` to detect `status === 'complete'` and render `elapsedSecs` as `-MM:SS` in red instead of countdown `remainingSecs`
- [ ] T019 [US1] Update timer display format function in `src/main.ts` to prepend negative sign for elapsed display
- [ ] T020 [US1] Update state label in `src/main.ts` to show completion message (e.g., "Work session completed") using `lastCompletedPhase`
- [ ] T021 [US1] Disable Start button when `status === 'complete'` in `updateUI()`
- [ ] T022 [P] [US1] Manual UI test: Start work session, wait for completion, verify `-00:01` appears in red, increments continuously

**Checkpoint**: User Story 1 complete and independently testable; elapsed display works end-to-end

---

## Phase 4: User Story 2 - Pause Elapsed Time (Priority: P2)

**Goal**: Allow users to pause the elapsed counter to freeze the display

**Independent Test**: Pause elapsed; verify counter stops incrementing for 10+ seconds; Resume shows counter resumes

### Backend Implementation for User Story 2

- [ ] T023 Modify `pause()` method in `src-tauri/src/timer.rs` to handle `status == Status::Complete`: move elapsed delta into `elapsed_paused_secs`, clear `elapsed_started_instant`, set `elapsed_running = false`, update `state_label` to "Paused (elapsed)"
- [ ] T024 Add Rust unit test for pause of elapsed: verify `elapsedRunning=false`, `elapsedStartedInstant=None`, elapsed accumulator holds correct paused value

### Frontend Implementation for User Story 2

- [ ] T025 [US2] Update `pauseBtn` event listener in `src/main.ts` to enable pause when elapsed is running (`status === 'complete' && elapsedRunning`)
- [ ] T026 [P] [US2] Add visual indicator for paused elapsed state in `updateUI()` (e.g., button state or display styling)
- [ ] T027 [US2] Manual UI test: Pause elapsed counter, wait 10s, verify time stays frozen; Resume, verify time resumes from paused value

**Checkpoint**: User Story 2 complete; pause functionality works independently

---

## Phase 5: User Story 3 - Resume Elapsed Time Counter (Priority: P2)

**Goal**: Resume counting from where elapsed was paused

**Independent Test**: Pause at "-02:15"; Resume; verify counter continues from "-02:15"

### Backend Implementation for User Story 3

- [ ] T028 Modify `resume()` method in `src-tauri/src/timer.rs` to handle `status == Status::Complete`: set `elapsed_started_instant = Some(Instant::now())`, set `elapsed_running = true`, update `state_label` to "Elapsed running"
- [ ] T029 Add Rust unit test for resume of elapsed: verify `elapsedRunning=true`, `elapsedStartedInstant=Some(now)`, elapsed continues from paused value without jumps

### Frontend Implementation for User Story 3

- [ ] T030 [US3] Update `resumeBtn` event listener in `src/main.ts` to enable resume when elapsed is paused (`status === 'complete' && !elapsedRunning`)
- [ ] T031 [P] [US3] Update visual indicator to clear paused state on resume
- [ ] T032 [US3] Manual UI test: Pause at "-02:15", Resume, wait 30s, verify display shows "-02:45"

**Checkpoint**: User Story 3 complete; pause/resume cycle works end-to-end

---

## Phase 6: User Story 4 - Clear Elapsed Time (Priority: P1)

**Goal**: Clear elapsed time and reset timer to ready state for next session

**Independent Test**: Display elapsed "-01:23"; Click Clear; verify elapsed disappears, timer shows next phase ready (05:00 if transitioning to break), Start enabled

### Backend Implementation for User Story 4

- [ ] T033 Ensure `clear()` method in `src-tauri/src/timer.rs` resets elapsed fields: `elapsed_started_instant = None`, `elapsed_paused_secs = 0`, `elapsed_running = false`, clears `completion_flag`
- [ ] T034 Verify `clear()` transitions to next phase ready state (phase already set to next from completion; set status to appropriate Ready, reset `remaining_secs` and `duration_secs`)
- [ ] T035 Add Rust unit test for clear after completion: verify elapsed fields cleared, status transitions to next phase ready (work/break), Start enabled

### Frontend Implementation for User Story 4

- [ ] T036 [P] [US4] Keep existing Clear button behavior; it already calls `invoke('clear_timer')`
- [ ] T037 [P] [US4] Update `shouldConfirmClear()` in `src/main.ts` to trigger confirmation dialog when `status === 'complete'` with elapsed
- [ ] T038 [US4] Verify Clear button remains enabled and clears `elapsedSecs` display in `updateUI()`
- [ ] T039 [US4] Verify after clear, display transitions to next phase countdown (e.g., "-01:23" → "05:00"), Start enabled
- [ ] T040 [P] [US4] Manual UI test: Pause elapsed at "-05:00", Click Clear, verify timer resets to ready state for next phase, Start enabled; start new session without errors

**Checkpoint**: User Story 4 complete; clear workflow matches existing pattern

---

## Phase 7: Integration & Validation

**Purpose**: Cross-story validation and edge case testing

- [ ] T041 [P] Full end-to-end test: Work 25min → complete → pause elapsed → resume → clear → Break → complete (verify next phase works)
- [ ] T042 [P] Test app minimization/focus scenarios: start elapsed, minimize app, return; verify elapsed continues counting with monotonic clock
- [ ] T043 [P] Test max elapsed display cap: wait for elapsed to exceed 99:59, verify display clamps and doesn't overflow
- [ ] T044 Test button state consistency across all elapsed substates (running, paused, clearing)
- [ ] T045 Verify `completionFlag` edge trigger only fires once per completion (for chime)
- [ ] T046 Test keyboard accessibility: Escape closes any dialogs; Tab navigates buttons in elapsed state

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final refinement, documentation, and code quality

- [ ] T047 [P] Add inline code comments in `src-tauri/src/timer.rs` for elapsed field usage and state transitions
- [ ] T048 [P] Update `src/main.ts` comments to document elapsed display and button state logic
- [ ] T049 Run `cargo fmt` and `cargo clippy` in `src-tauri/` to verify code style compliance
- [ ] T050 Verify all elapsed-related unit tests pass with `cargo test` in `src-tauri/`
- [ ] T051 Run quickstart.md validation: follow all steps in [quickstart.md](quickstart.md) to verify implementation matches documented flow
- [ ] T052 [P] Review CSS contrast ratio for elapsed red text (`#ef4444` on `#1a1a1a`) meets WCAG AA standards
- [ ] T053 Update feature branch PR description and commit messages with conventional commit format (feat: elapsed display)
- [ ] T054 Final smoke test: complete two sessions (work → break), verify full lifecycle works without regressions

**Checkpoint**: Feature complete, tested, documented, and ready for merge

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 Setup**: No dependencies - start immediately
- **Phase 2 Foundational**: Depends on Phase 1 - BLOCKS all user stories
- **Phases 3-6 User Stories**: All depend on Phase 2 Foundational completion
  - **US1 (P1)** and **US4 (P1)**: Can proceed in parallel after Foundational; together form MVP
  - **US2 (P2)** and **US3 (P2)**: Can proceed in parallel after US1, or after Foundational if staffed
  - **US1 + US4** = MVP (display + clear); **US2 + US3** = enhancement (pause/resume)
- **Phase 7 Integration**: Depends on all desired user stories (at minimum US1 + US4)
- **Phase 8 Polish**: Depends on integration validation passing

### Within Phase 2 (Foundational)

Tasks T004-T011 must complete sequentially (each builds on previous state structure):

1. T004: Add fields
2. T005: Initialize in constructor
3. T006: Start elapsed on completion
4. T007: Compute elapsed in get_state
5. T008: Update TimerState struct
6. T009: Reject start in Complete state
7. T010: Clear elapsed
8. T011: Verify IPC serialization

### Parallel Opportunities

**Within Phase 2**: Tasks T004-T011 are sequential (one struct evolves)

**Within Phase 3 (US1)**:

- Backend implementation (T012-T015) can precede frontend (T016-T022)
- Frontend CSS (T016-T017) can start in parallel with backend

**Within Phase 4 (US2)** and **Phase 5 (US3)**:

- Backend (T023-T024) and (T028-T029) can run in parallel with each other
- Frontend (T025-T027) and (T030-T032) can run in parallel with each other
- Frontend must wait for backend Phase 2 completion before testing

**Phase 3 + Phase 6** (US1 + US4):

- Can be done together as one task sequence (both core to MVP)
- Frontend CSS (T016-T017) and Clear button logic (T036-T040) can run in parallel

**Phase 7 Integration**:

- All test tasks (T041-T046) marked [P] can run in parallel on different scenarios

**Phase 8 Polish**:

- Formatting and linting (T047-T049) marked [P] can run in parallel
- Tests and validation (T050-T054) must run sequentially

### Parallel Example: MVP Fast Track (US1 + US4 together)

```
Phase 1 Setup (T001-T003) → Complete
Phase 2 Foundational (T004-T011) → Complete sequentially
Phase 3 + 6 MVP (US1 + US4):
  - Backend: T012-T015 (US1) + T033-T035 (US4) → Complete sequentially
  - Frontend Parallel:
    - T016-T017 (CSS) + T025-T027 (US2 pause UI state) → Start
    - T018-T022 (US1 display) + T036-T040 (US4 clear) → Start after Phase 2
  - Manual tests: T022 + T040 → Validate MVP end-to-end
→ MVP Complete: Display elapsed + Clear to next phase
```

---

## Implementation Strategy

### MVP First (US1 + US4 = Display + Clear)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (backend elapsed infrastructure)
3. Complete Phase 3 (US1) + Phase 6 (US4): Display elapsed and clear
4. **STOP and VALIDATE**: Test US1 + US4 independently
5. Deploy/demo MVP: elapsed display with clear workflow

### Incremental Enhancement

1. MVP deployed (US1 + US4)
2. Add Phase 4 (US2): Pause elapsed
3. Add Phase 5 (US3): Resume elapsed
4. Phase 7-8: Integration and polish
5. Deploy full feature

### Parallel Team Strategy (if multiple devs)

With team capacity:

1. All: Complete Phase 1 + Phase 2 together
2. Once Foundational done:
   - **Dev A**: US1 (P1) backend + frontend display → MVP core
   - **Dev B**: US4 (P1) clear logic (backend + frontend) → MVP completion
   - **Dev C**: US2 + US3 (P2) pause/resume → enhancement
3. Dev A + Dev B merge → MVP ready
4. Dev C merges → Full feature
5. All: Phase 7-8 validation and polish together

---

## Notes

- [P] tasks = same story, different components (backend/frontend); can parallelize if capacity exists
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- **Commit after each phase or milestone** to preserve history
- Manual UI tests (marked "Manual") are acceptance validation; document results
- Phase 2 Foundational is sequential and blocking; all others can parallelize after it
- MVP = Phase 1 + Phase 2 + Phase 3 (US1) + Phase 6 (US4)
- Full feature = All phases through Phase 8
