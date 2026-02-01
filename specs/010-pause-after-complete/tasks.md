# Tasks: Pause/Resume After Completion

**Input**: Design documents from `/specs/010-pause-after-complete/`
**Prerequisites**: plan.md (required), spec.md, research.md, data-model.md, contracts/

**Automated Tests**: Not requested. Manual verification is described in the checkpoints below.

## Phase 1: Setup (Shared Infrastructure)

- [ ] T001 Review current timer pause/resume flow in src-tauri/src/timer.rs and UI bindings in src/main.ts

---

## Phase 2: Foundational (Blocking Prerequisites)

- [ ] T002 Update `TimerState` to add `overtimePaused` status and `overtimePausedSecs` in src-tauri/src/timer.rs
- [ ] T003 [P] Update `TimerState` type to include `overtimePaused` and `overtimePausedSecs` in src/main.ts

**Checkpoint**: Shared state model updated across backend and UI.

---

## Phase 3: User Story 1 - Pause overdue time (Priority: P1) 🎯 MVP

**Goal**: Allow pausing and resuming overdue time after completion.

**Manual Verification**: Complete a session, pause at negative time, verify the display freezes, then resume and verify it continues from the frozen value.

### Implementation for User Story 1

- [ ] T004 [US1] Implement pause/resume transitions for completed overtime in src-tauri/src/timer.rs
- [ ] T005 [US1] Update overtime calculation and state label for `overtimePaused` in src-tauri/src/timer.rs
- [ ] T006 [P] [US1] Render frozen overtime using `overtimePausedSecs` in src/main.ts

**Checkpoint**: Overdue time can be paused and resumed after completion.

---

## Phase 4: User Story 2 - Keep controls available after completion (Priority: P2)

**Goal**: Keep Pause/Resume controls available after completion for work and break sessions.

**Manual Verification**: Complete a work session and a break session; confirm Pause/Resume remains enabled in both cases.

### Implementation for User Story 2

- [ ] T007 [US2] Enable Pause when status is `complete` and Resume when status is `overtimePaused` in src/main.ts

**Checkpoint**: Controls remain available after completion across both phases.

---

## Phase 5: Polish & Cross-Cutting Concerns

- [ ] T008 [P] Validate quickstart steps in specs/010-pause-after-complete/quickstart.md

---

## Dependencies & Execution Order

- **Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5**
- **US1** depends on Phase 2 completion.
- **US2** depends on Phase 2 completion and does not depend on US1.

## Parallel Execution Examples

- **After Phase 2**: T004 (timer.rs) and T006 (main.ts) can run in parallel.
- **Phase 2**: T002 (timer.rs) and T003 (main.ts) can run in parallel.

## Implementation Strategy

- **MVP**: Complete Phases 1–3 (US1) and validate the independent test.
- **Incremental**: Add US2 control enablement, then validate quickstart.
