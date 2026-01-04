# Tasks: Confirm Dialog for Clear Action

**Input**: Design documents from `/specs/005-confirm-clear-dialog/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/, quickstart.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Understand current Clear handling and timer state before changes

- [x] T001 [P] Review current Clear button handling and timer update flow in src/main.ts
- [x] T002 [P] Inspect timer service state and clear implementation in src-tauri/src/timer.rs

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Validate backend clear behavior before UI changes

- [x] T003 Add unit test covering clear behavior (idle reset, mode preserved) in src-tauri/src/timer/tests.rs
- [x] T004 Verify clear_timer command returns cleared state consistently in src-tauri/src/lib.rs (no contract changes)

## Phase 3: User Story 1 - Confirm before clearing active timer (Priority: P1) 🎯 MVP

**Goal**: Prevent accidental loss of running/paused sessions by confirming Clear
**Independent Test**: Start session, click Clear → dialog shows; Confirm resets to idle; Cancel keeps state

- [x] T005 [P] [US1] Add confirmation dialog markup to src/index.html (overlay, dialog, confirm/cancel buttons)
- [x] T006 [P] [US1] Add modal styling to src/assets/styles.css (overlay, centered dialog, buttons, z-index)
- [x] T007 [US1] Implement Clear click logic in src/main.ts: show dialog when remaining_time>0 via get_state; bypass dialog when idle; wire confirm to invoke clear_timer and refresh display
- [x] T008 [US1] Add keyboard handling/focus in src/main.ts: focus confirm on open, Enter activates focused button

## Phase 4: User Story 2 - Understand impact of clearing (Priority: P2)

**Goal**: Make the consequence of clearing explicit in the dialog text and labels
**Independent Test**: Trigger Clear with progress → dialog text states it will remove current time/status; options are clear/cancel

- [x] T009 [P] [US2] Set dialog message and button labels in src/index.html to explain clearing removes current time/status
- [x] T010 [US2] Ensure dialog layout fits small window without scrolling in src/assets/styles.css (spacing, max-width)

## Phase 5: User Story 3 - Dismiss safely and continue (Priority: P3)

**Goal**: Let users cancel/dismiss and continue session unchanged; keep Clear disabled when idle
**Independent Test**: Open dialog, press Cancel/Escape/overlay → dialog closes and timer state/time unchanged; Clear disabled when idle

- [x] T011 [P] [US3] Implement cancel/dismiss handlers in src/main.ts (Cancel button, Escape key, overlay click) to hide dialog without state change
- [x] T012 [US3] Keep Clear button disabled when idle/no progress by updating state-driven UI logic in src/main.ts

## Final Phase: Polish & Cross-Cutting

**Purpose**: Docs and verification

- [x] T013 [P] Update specs/005-confirm-clear-dialog/quickstart.md with final steps/commands and manual test checklist
- [x] T014 Run fmt, clippy, tests in src-tauri/Cargo.toml workspace and perform manual dialog scenarios (confirm, cancel, idle clear)

---

## Dependencies & Execution Order

- Phase 1 → Phase 2 → Phase 3 (US1, P1) → Phase 4 (US2, P2) → Phase 5 (US3, P3) → Final Phase
- User stories are independent after Foundational; US1 delivers MVP. US2/US3 build UX clarity/safety.

## Parallel Opportunities

- T001 and T002 can run in parallel (different files)
- T003 and T004 can run in parallel (tests vs command review)
- Within US1, T005/T006 can run in parallel; T007 depends on markup/styles; T008 follows T007
- US2 (T009/T010) can proceed after US1 dialog exists
- US3 (T011/T012) can proceed after US1 base dialog logic; T011/T012 can run in parallel if coordinated
- T013 can start after stories planned; T014 last before delivery

## Implementation Strategy

- Build MVP with US1 first (T005–T008) after Foundational
- Layer messaging (US2) then safe dismissal/idle disable (US3)
- Finish with polish (docs, fmt/clippy/test, manual scenarios)
