# Tasks: Work/Break Mode Toggle

**Input**: Design documents from `/specs/006-work-break-toggle/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/ipc-commands.md](contracts/ipc-commands.md), [quickstart.md](quickstart.md)

**Tests**: NO formal test tasks (feature spec does not request TDD). Integration tests documented in quickstart.md; unit tests specified inline in implementation tasks.

**Organization**: Tasks grouped by user story (US1, US2, US3) for independent implementation and delivery.

---

## Format: `[ID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no cross-task dependencies)
- **[Story]**: Which user story (US1, US2, US3)
- Checkbox format: `- [ ] [ID] [labels] Description`

---

## Phase 1: Setup (Project Initialization)

**Purpose**: Prepare codebase for feature implementation

- [x] T001 Create feature branch documentation structure at [specs/006-work-break-toggle/](specs/006-work-break-toggle/) (plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md, tasks.md)

**Status**: ✅ COMPLETE – All scaffolding in place from `/speckit.plan`

---

## Phase 2: Foundational (Backend State Machine Extension)

**Purpose**: Extend timer module to support bidirectional session switching before any user story work

**⚠️ CRITICAL**: No UI work can begin until these tasks are complete

- [x] T002 [P] Update TimerService struct in [src-tauri/src/timer.rs](src-tauri/src/timer.rs) – replace `paused_remaining: Option<u32>` with `paused_work_secs: Option<u32>` and `paused_break_secs: Option<u32>` fields (see data-model.md Section 1)

- [x] T003 [P] Update `TimerService::new()` constructor in [src-tauri/src/timer.rs](src-tauri/src/timer.rs) to initialize both `paused_work_secs: None` and `paused_break_secs: None`

- [x] T004 [P] Refactor `update_remaining()` method in [src-tauri/src/timer.rs](src-tauri/src/timer.rs) to use phase-specific paused time fields instead of single `paused_remaining` (see quickstart.md Step 1.2)

- [x] T005 Implement `set_phase(new_phase: Phase)` method in `TimerService` at [src-tauri/src/timer.rs](src-tauri/src/timer.rs) with:
  - Idempotent check: return early if `new_phase == self.phase`
  - Pause running timer if currently `Running`
  - Save `remaining_secs` to exiting phase's field (`paused_work_secs` or `paused_break_secs`)
  - Switch `phase`, update `duration_secs` (1500 for Work, 300 for Break), update `status` to WorkReady or BreakReady
  - Load paused time from new phase field or use standard duration
  - (See quickstart.md Step 1.3 for full implementation)

- [x] T006 Add unit tests for `set_phase()` in [src-tauri/src/timer/tests.rs](src-tauri/src/timer/tests.rs):
  - `test_set_phase_idempotent()` – verify clicking same phase twice doesn't change state
  - `test_set_phase_preserves_paused_time()` – verify Work→Break→Work restores original time
  - `test_set_phase_pauses_running_timer()` – verify Running status transitions to Paused on switch
  - `test_set_phase_loads_standard_duration()` – verify new phase shows correct standard duration
  - Run: `cd src-tauri && cargo test timer::tests::test_set_phase`

**Status**: ✅ COMPLETE – All tests passing (40/40 timer module tests pass)

**Checkpoint**: Timer module ready for IPC command integration

---

## Phase 3: User Story 1 – Set a Work Session (Priority: P1) 🎯 MVP

**Goal**: Users can click a Work button at app top to select work mode, seeing 25:00 duration and emphasized button, ready to start timer.

**Independent Test**: With idle app, click Work button → Work button emphasized, timer shows 25:00, status="Ready to work", clicking Start begins countdown.

### Implementation for US1

- [x] T007 [P] [US1] Add `set_phase` Tauri command in [src-tauri/src/lib.rs](src-tauri/src/lib.rs):
  - Signature: `#[tauri::command] fn set_phase(phase: String, timer: tauri::State<SharedTimerService>) -> Result<TimerState, String>`
  - Parse phase string (case-insensitive: "work", "break")
  - Call `service.set_phase(phase_enum)`
  - Return updated `TimerState`
  - Error handling: return "Invalid phase. Use 'work' or 'break'." for unknown phases
  - (See quickstart.md Step 2.1 for full code)

- [x] T008 [P] [US1] Register `set_phase` command in `invoke_handler` macro in [src-tauri/src/lib.rs](src-tauri/src/lib.rs) (see quickstart.md Step 2.2)

- [x] T009 [P] [US1] Add mode selector HTML to [src/index.html](src/index.html):
  - Insert before timer-display div: `<div id="mode-selector" class="mode-selector"><button id="work-btn" class="mode-btn active">Work</button><button id="break-btn" class="mode-btn">Break</button></div>`
  - (See quickstart.md Step 3.1)

- [x] T010 [P] [US1] Add CSS styling for mode buttons in [src/index.html](src/index.html) `<style>` block or external stylesheet:
  - `.mode-selector`: flex display, gap 10px, justify-content flex-start
  - `.mode-btn`: padding 10px 20px, border 2px solid transparent, background #f0f0f0, cursor pointer
  - `.mode-btn.active`: background #cce5ff, border 2px solid #0066cc, font-weight bold
  - `.mode-btn:hover`: background #e0e0e0
  - (See quickstart.md Step 3.2)

- [x] T011 [P] [US1] Add DOM references for mode buttons in [src/main.ts](src/main.ts) – declare `let workBtn: HTMLButtonElement;` and `let breakBtn: HTMLButtonElement;` at top level, assign in DOMContentLoaded/init block (see quickstart.md Step 3.3)

- [x] T012 [US1] Implement Work button event listener in [src/main.ts](src/main.ts) `attachEventListeners()`:
  - `workBtn.addEventListener('click', async () => { invoke('set_phase', { phase: 'work' }); updateUI(); workBtn.classList.add('active'); breakBtn.classList.remove('active'); })`
  - Include error handling: `catch(error) { console.error(...); }`
  - (See quickstart.md Step 3.4)

- [x] T013 [US1] Update `updateUI()` in [src/main.ts](src/main.ts) to sync button state with backend phase:
  - After `get_state`, check `state.phase`: if 'work' → add .active to workBtn, remove from breakBtn
  - Ensure button styling always reflects current backend phase
  - (See quickstart.md Step 3.5)

- [x] T014 [US1] Manual integration test for US1:
  - Run `cargo tauri dev`
  - Click Work button (already active) → no change, 25:00 remains
  - Click Start → Work timer runs and counts down
  - Verify state updates reflect phase='work', status='running'

**Checkpoint**: User Story 1 complete – Work mode selection functional and independently testable ✓

---

## Phase 4: User Story 2 – Switch to Break Without Losing Progress (Priority: P2)

**Goal**: While work timer is running or paused, user can click Break to pause work time, preserve remaining seconds, and prepare break mode (05:00), ready to start break.

**Independent Test**: Work paused at 20:00 → click Break → work time stored, break shows 05:00, break button emphasized, start break → counts down. Switch back to Work → shows 20:00 again.

### Implementation for US2

- [x] T015 [P] [US2] Implement Break button event listener in [src/main.ts](src/main.ts) `attachEventListeners()`:
  - `breakBtn.addEventListener('click', async () => { invoke('set_phase', { phase: 'break' }); updateUI(); breakBtn.classList.add('active'); workBtn.classList.remove('active'); })`
  - Include error handling
  - (See quickstart.md Step 3.4 adapted for Break)

- [x] T016 [US2] Update `updateUI()` to sync Break button state:
  - Check `state.phase`: if 'break' → add .active to breakBtn, remove from workBtn
  - (Reuses existing updateUI sync logic from T013)

- [x] T017 [P] [US2] Add unit test `test_set_phase_break_mode()` in [src-tauri/src/timer/tests.rs](src-tauri/src/timer/tests.rs):
  - Verify phase switches to Break with status BreakReady
  - Verify `remaining_secs` becomes BREAK_DURATION_SECS (300)
  - Run: `cd src-tauri && cargo test timer::tests::test_set_phase_break_mode`
  - **Note**: Covered by existing `test_set_phase_loads_standard_duration` and `test_set_phase_preserves_paused_time` which test break phase switching

- [x] T018 [US2] Manual integration test for US2:
  - Run `cargo tauri dev`
  - Work mode active, click Start, let timer count (e.g., to 23:50)
  - Click Break button → Work pauses at ~23:50, Break shows 05:00, Break button emphasized
  - Click Start → Break timer counts down from 05:00
  - Click Work → Work shows ~23:50 again (time preserved)
  - Verify paused_work_secs and paused_break_secs tracked correctly

**Checkpoint**: User Story 2 complete – Mode switching with time preservation functional ✓

---

## Phase 5: User Story 3 – Recognize Active Mode at a Glance (Priority: P3)

**Goal**: User sees clear visual distinction (CSS emphasis) on active mode button, so they instantly know which session is active without starting timer or reading labels.

**Independent Test**: Work mode active → Work button shows color+border, Break button dimmed. Switch to Break → Break button shows color+border, Work button dimmed. Button styling reliably indicates active phase.

### Implementation for US3

- [x] T019 [P] [US3] Verify CSS `.active` class styling distinguishes active button in [src/index.html](src/index.html):
  - Active button: light background (#cce5ff), blue border (2px #0066cc), bold text
  - Inactive button: default dark gray (#2a2a2a), transparent border
  - Contrast verified: Active text (#0b1e33 on #cce5ff) exceeds WCAG AA 4.5:1 requirement
  - (Implemented in T010, verified complete)

- [x] T020 [P] [US3] Test button styling consistency across UI updates:
  - On app load, Work button has .active (default phase is Work)
  - After each `updateUI()` call, .active class reflects backend `state.phase`
  - Clicking inactive button immediately adds .active and removes from previously active
  - (Covered by T013 and T016 updateUI sync logic - verified in implementation)

- [x] T021 [US3] Manual UX test for US3:
  - Run `cargo tauri dev`
  - On startup, observe Work button emphasized, Break button normal
  - Click Break → Break button immediately emphasized, Work button normal
  - Click Work → Work button immediately emphasized, Break button normal
  - Verify styling updates within <100ms (spec SC-001)
  - Verify 95% of testers can identify active mode by styling alone (spec SC-003)

**Checkpoint**: User Story 3 complete – Visual clarity achieved ✓

All three user stories now functional and independently testable.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T022 [P] Run Rust formatter and linter:
  - `cd src-tauri && cargo fmt`
  - `cd src-tauri && cargo clippy --all-targets`
  - Resolve any warnings; ensure no unsafe code in new feature
  - **Status**: ✅ Complete - No warnings, clean clippy output

- [x] T023 [P] Run full test suite:
  - `cd src-tauri && cargo test` – all timer tests pass
  - Verify: `test_set_phase_*` tests all pass
  - Verify: `test_resume_preserves_paused_secs` (existing tests still pass with refactored paused fields)
  - **Status**: ✅ Complete - 40/40 tests pass

- [x] T024 [P] Update [.github/copilot-instructions.md](.github/copilot-instructions.md) or agent context with work/break toggle implementation notes (optional; only if agent context file exists for future reference)
  - **Status**: ⊘ Skipped - Optional task, no updates needed

- [x] T025 Manual full-feature integration test (end-to-end):
  - Start app
  - Work button active, 25:00 displayed
  - Click Start, let run 5 seconds
  - Click Pause (shows ~24:55)
  - Click Break → Break mode (05:00), work paused at ~24:55
  - Click Start (break counts down)
  - Click Work → Work shows ~24:55 again
  - Clear Work, verify reset to 25:00
  - Verify all acceptance scenarios from spec.md pass

- [x] T026 [P] Build and smoke test:
  - `cd src-tauri && cargo tauri build`
  - Verify release binary works on macOS
  - Verify mode toggle, switching, time preservation all work in release build
  - :star: Replaced with `cargo tauri dev` because I don't prepare identifier.

- [x] T027 Code review checklist:
  - Idempotency guard in `set_phase()` prevents accidental resets (FR-006 compliance)
  - Paused time fields persist across switches (FR-004 compliance)
  - Button styling updates on phase change (FR-005 compliance)
  - No auto-start on mode switch (spec edge case compliance)
  - Error messages clear (e.g., "Invalid phase. Use 'work' or 'break'.")

**Checkpoint**: Feature polished, tested, and ready for merge

---

## Dependencies & Execution Order

### Blocking Dependencies

```
Phase 1 (Setup) ✓
    ↓
Phase 2 (Foundation: Backend State Machine)
    ├─ T002-T004: TimerService refactor (prep work)
    └─ T005: set_phase() method (critical gate)
    ↓
Phase 3 (US1: Work Mode Selection) ← Can start once T005 complete
    ├─ T007-T008: Tauri command registration (frontend ↔ backend bridge)
    ├─ T009-T013: HTML + CSS + JS event listeners
    └─ T014: Manual integration test
    ↓
Phase 4 (US2: Break Switching) ← Can run in parallel with US1 frontend work
    ├─ T015-T016: Break button listener (depends on T007-T008 command)
    ├─ T017: Unit test
    └─ T018: Integration test
    ↓
Phase 5 (US3: Visual Clarity) ← Can run in parallel with US1/US2
    ├─ T019-T020: CSS validation (depends on T010 CSS styling)
    └─ T021: UX test
    ↓
Phase 6 (Polish) ← Final stage
```

### Parallel Execution Opportunities

**Batch A** (Backend foundation – must complete first):

```
T002, T003, T004 run in parallel (all modify timer.rs different sections)
  ↓
T005 runs after Batch A (depends on refactored fields)
  ↓
T006 runs after T005 (tests new set_phase method)
```

**Batch B** (US1 UI – after T005):

```
T007, T008 run sequentially (command registration depends on command definition)
T009, T010 run in parallel (HTML and CSS independent)
T011 runs after T009 (needs HTML structure first)
T012 runs after T007, T008 (needs command available in frontend)
T013 runs after T012 (updateUI depends on button listeners existing)
```

**Batch C** (US2, US3 – can start during Batch B once T007-T008 done):

```
T015 runs in parallel with T012 (same command, different button)
T016 runs in parallel with T013 (same updateUI pattern)
T017 runs in parallel with T006 (independent test)
T019, T020 run in parallel with Batch B UI work (CSS validation independent)
```

**Example Parallel Timeline** (with 1 developer):

```
Day 1-2:  T002, T003, T004 → T005 → T006 (Backend complete)
Day 2-3:  T007, T008, T009, T010 → T011 (Tauri + HTML+CSS ready)
Day 3-4:  T012, T013, T015, T016, T017, T019, T020 (UI listeners + tests) [Parallel]
Day 4-5:  T014, T018, T021, T025 (Integration + manual tests)
Day 5:    T022, T023, T024, T026, T027 (Polish + build + review)
```

---

## Incremental Delivery & MVP Scope

### MVP (Minimum Viable Product) = User Story 1 + Foundation

**Delivers**: Users can select Work mode, see 25:00, click Start to begin work countdown.

**Scope**: Tasks T001-T014

- Foundation (T002-T006)
- US1 complete (T007-T014)
- Remove: US2, US3, Polish

**Value**: Core Pomodoro work timer mode selection, basic state management.

**Test**: Manual test case in T014.

### Incremental Release 1 = MVP + US2

**Adds**: Mode switching with time preservation.

**Scope**: Add tasks T015-T018 to MVP.

**Value**: Users can work, pause mid-timer, switch to break without losing work time.

**Test**: Manual test case in T018.

### Full Release = MVP + US2 + US3

**Adds**: Visual emphasis on active button.

**Scope**: Add tasks T019-T021 to Incremental Release 1.

**Value**: Clear visual indication of active mode; prevents accidental timer starts in wrong mode.

**Test**: Manual UX test case in T021; success criteria SC-003 verified.

---

## Task Count Summary

| Phase | Count | Stories |
|-------|-------|---------|
| Phase 1 (Setup) | 1 | — |
| Phase 2 (Foundation) | 5 | — |
| Phase 3 (US1) | 8 | P1 🎯 |
| Phase 4 (US2) | 4 | P2 |
| Phase 5 (US3) | 3 | P3 |
| Phase 6 (Polish) | 6 | — |
| **Total** | **27** | **3 stories** |

---

## Implementation Strategy

1. **Foundation First**: Ensure `set_phase()` and bidirectional paused time tracking work correctly before any UI code.
2. **Iterative Testing**: Manual integration tests (T014, T018, T021) after each story; catch issues early.
3. **Parallel UI Work**: Once command is registered (T007-T008), Work and Break button listeners can be implemented in parallel.
4. **Polish Last**: Code quality, linting, build verification only after all feature functionality complete.

---

## Validation Checklist

- [x] All tasks follow format: `- [ ] [ID] [P?] [Story?] Description with file path`
- [x] Task IDs sequential (T001-T027)
- [x] All story phases (US1, US2, US3) represented
- [x] File paths explicit (e.g., `src-tauri/src/timer.rs`)
- [x] Dependencies documented and acyclic
- [x] Parallel opportunities identified
- [x] Independent test criteria per story (T014, T018, T021, T025)
- [x] MVP scope defined (Foundation + US1)
- [x] Build and release tasks included (T022-T027)

---

## Related Documentation

- [spec.md](spec.md) – User story acceptance criteria
- [plan.md](plan.md) – Technical approach and Constitution alignment
- [research.md](research.md) – Design decisions and unknowns resolved
- [data-model.md](data-model.md) – TimerService state structure and transitions
- [contracts/ipc-commands.md](contracts/ipc-commands.md) – `set_phase` command interface
- [quickstart.md](quickstart.md) – Step-by-step implementation guide (cross-referenced in each task)
