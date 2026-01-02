---

description: "Tasks for Pomodoro Desktop Timer"
---

# Tasks: Pomodoro Desktop Timer

**Input**: Design documents from /specs/001-pomodoro-desktop-timer/
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Phase 1: Setup (Shared Infrastructure)

- [x] T001 Pin Rust toolchain to 1.92 in rust-toolchain.toml
- [x] T002 Initialize Tauri 2 app skeleton (src-tauri/, src/) via cargo tauri init so src-tauri/Cargo.toml and src/index.html exist
- [x] T003 Configure app metadata (name, window title, identifiers) in src-tauri/tauri.conf.json

---

## Phase 2: Foundational (Blocking Prerequisites)

- [x] T004 Create timer module skeleton with state enums, durations (25m/5m), and shared TimerState struct in src-tauri/src/timer.rs
- [x] T005 [P] Wire Tauri command plumbing (start, pause, resume, clear, get_state signatures) to timer module in src-tauri/src/main.rs
- [x] T006 [P] Scaffold frontend shell with layout and controls (time display, Start, Pause/Resume, Clear) in src/index.html
- [x] T007 [P] Add frontend IPC/polling helper to invoke timer commands and refresh state at 1 Hz in src/main.ts

---

## Phase 3: User Story 1 - Start and manage 25-minute work session (Priority: P1) 🎯 MVP

**Goal**: User can start a 25:00 work session, see countdown, pause/resume, and prevent overlapping starts.

**Independent Test**: From idle, Start begins 25:00 countdown; Pause/Resume freezes and continues without drift; Start stays disabled while running.

### Implementation

- [x] T008 [US1] Implement work timer state machine (start/pause/resume, overlap prevention, monotonic timing) in src-tauri/src/timer.rs
- [x] T009 [US1] Implement Tauri command handlers for start/pause/resume/get_state returning TimerState for work mode in src-tauri/src/main.rs
- [x] T010 [US1] Render work countdown and control states (start disabled while running, pause/resume toggle) in src/main.ts
- [x] T011 [US1] Display current work state/remaining time in UI layout in src/index.html

**Checkpoint**: User Story 1 independently testable (work session only).

---

## Phase 4: User Story 2 - Take 5-minute break after focus (Priority: P2)

**Goal**: Auto-transition to a 5:00 break after work completion with pause/resume and completion cues.

**Independent Test**: Let work reach 00:00 → break starts at 5:00; pause/resume works in break; completion cue fires at break end.

### Implementation

- [x] T012 [US2] Extend state machine for work→break transition, break countdown, and completion flag in src-tauri/src/timer.rs
- [x] T013 [US2] Add completion cue trigger (visual state and local chime asset) on work/break completion in src/main.ts and src/assets/chime.mp3
- [x] T014 [US2] Update UI logic to handle break state (labels, remaining time, pause/resume) in src/main.ts
- [x] T015 [US2] Update layout copy to reflect break mode indicators in src/index.html

**Checkpoint**: User Stories 1 and 2 independently testable (full work+break cycle).

---

## Phase 5: User Story 3 - Clear and prepare next cycle (Priority: P3)

**Goal**: Clear stops any timer and resets to ready 25:00 work state.

**Independent Test**: Press Clear in any state → timer stops, displays 25:00 work-ready, Start enabled.

### Implementation

- [x] T016 [US3] Implement clear/reset logic returning ready work state in src-tauri/src/timer.rs
- [x] T017 [US3] Wire clear command handler and ensure get_state reflects reset in src-tauri/src/main.rs
- [x] T018 [US3] Implement Clear button behavior and state reset (time display, control flags) in src/main.ts
- [x] T019 [US3] Reflect ready-state labels and defaults in layout after clear in src/index.html

**Checkpoint**: All user stories independently testable.

---

## Phase N: Polish & Cross-Cutting

- [x] T020 Refresh quickstart instructions to match final dev commands in specs/001-pomodoro-desktop-timer/quickstart.md
- [x] T021 Add accessibility pass (aria labels, focus order) for controls in src/index.html
- [x] T022 Document implemented IPC behaviors and any deviations in specs/001-pomodoro-desktop-timer/contracts/openapi.yaml

---

## Dependencies & Execution Order

- Setup (Phase 1) → Foundational (Phase 2) → US1 (P1) → US2 (P2) → US3 (P3) → Polish
- Foundational blocks all stories; each story is independently testable once its phase completes.
- Stories can proceed sequentially by priority; after Foundational, different owners could parallelize stories if needed (US1 prioritized for MVP).

## Parallel Examples

- Within Foundational: T005, T006, T007 can run in parallel (different files: src-tauri/src/main.rs, src/index.html, src/main.ts).
- Within US1: T008 can proceed while T011 designs layout; T009 and T010 can run in parallel once timer module exists.
- Across stories: After US1 complete, US2 and US3 can be developed in parallel (UI vs clear logic) if coordination on shared files is managed.

## Implementation Strategy

- MVP first: Finish Setup → Foundational → US1; validate countdown accuracy and controls before adding break/clear.
- Incremental: Add US2 (break flow and cues), then US3 (clear/reset), validating each independently.
- Keep timer logic centralized in src-tauri/src/timer.rs; UI should consume TimerState via Tauri commands to avoid duplicate state.
