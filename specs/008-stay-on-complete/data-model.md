# Data Model: Stay on Completed Session

**Date**: 2026-01-14  
**Feature**: [spec.md](spec.md)  
**Research**: [research.md](research.md)

## Overview

This document defines the data structures and state transitions for the stay-on-complete feature. This feature removes automatic phase switching when sessions complete, requiring only behavioral changes to existing data structures.

---

## Entities

### 1. TimerService (Rust Backend)

**Location**: `src-tauri/src/timer.rs`

**Existing Fields** (no new fields required):

```rust
pub struct TimerService {
    pub(crate) phase: Phase,                    // Current phase: Work or Break
    pub(crate) status: Status,                  // Current status
    pub(crate) remaining_secs: u32,             // Remaining seconds for current phase
    duration_secs: u32,                         // Total duration for current phase
    completion_flag: bool,                      // Flag when phase completes
    pub(crate) started_instant: Option<Instant>, // When current run started
    pub(crate) paused_work_secs: Option<u32>,   // Paused remaining time for Work phase
    pub(crate) paused_break_secs: Option<u32>,  // Paused remaining time for Break phase
    state_label: String,                        // Human-readable status
}
```

**Key Behavioral Changes**:

- `phase` now persists through completion (no auto-switch)
- `status` will be set to `Status::Complete` when session finishes
- `remaining_secs` will be 0 at completion
- `completion_flag` remains true until user takes action (start, clear, or phase switch)

---

## State Transitions

### Current Behavior (Before This Feature)

```text
Work Running (25:00 → 00:00)
    ↓ [automatic on completion]
Break Ready (5:00)
    ↓ [user clicks Start]
Break Running (5:00 → 00:00)
    ↓ [automatic on completion]
Work Ready (25:00)
```

### New Behavior (After This Feature)

```text
Work Running (25:00 → 00:00)
    ↓ [stays in work phase]
Work Complete (00:00)
    ↓ [user clicks Start]
Work Running (25:00)
    OR
    ↓ [user clicks Break button]
Break Ready (5:00)
    ↓ [user clicks Start]
Break Running (5:00 → 00:00)
    ↓ [stays in break phase]
Break Complete (00:00)
    ↓ [user clicks Start]
Break Running (5:00)
    OR
    ↓ [user clicks Work button]
Work Ready (25:00)
```

---

## Status Enum Usage

**Location**: `src-tauri/src/timer.rs`

**Existing Definition** (no changes):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    WorkReady,    // Work phase ready to start
    BreakReady,   // Break phase ready to start
    Running,      // Timer is counting down
    Paused,       // Timer is paused with remaining time
    Complete,     // Phase completed (NEW USAGE: now persists until user action)
}
```

**Usage Change**:

- **Before**: `Complete` was a transient state immediately followed by auto-transition
- **After**: `Complete` is a persistent state until user presses Start, Clear, or switches phase

---

## Method Behavior Changes

### 1. handle_completion()

**Current Implementation** (lines 98-124):

```rust
pub(crate) fn handle_completion(&mut self) {
    self.completion_flag = true;
    match self.phase {
        Phase::Work => {
            // AUTO-SWITCHES to Break
            self.phase = Phase::Break;
            self.status = Status::BreakReady;
            self.duration_secs = BREAK_DURATION_SECS;
            self.remaining_secs = BREAK_DURATION_SECS;
            // ... clear timers
        }
        Phase::Break => {
            // AUTO-SWITCHES to Work
            self.phase = Phase::Work;
            self.status = Status::WorkReady;
            self.duration_secs = WORK_DURATION_SECS;
            self.remaining_secs = WORK_DURATION_SECS;
            // ... clear timers
        }
    }
}
```

**New Implementation**:

```rust
pub(crate) fn handle_completion(&mut self) {
    self.completion_flag = true;
    self.remaining_secs = 0;
    self.status = Status::Complete;
    self.started_instant = None;

    // Stay in current phase, update label
    self.state_label = match self.phase {
        Phase::Work => "Work completed".to_string(),
        Phase::Break => "Break completed".to_string(),
    };

    // Note: Do NOT clear paused_work_secs or paused_break_secs
    // Note: Do NOT change self.phase or self.duration_secs
}
```

**Key Differences**:

- Removes all phase-switching logic
- Sets `status = Status::Complete` (persistent state)
- Stays in current phase
- Simpler implementation (fewer state changes)

---

### 2. start()

**Current Behavior** (lines 127-168):

- Checks `Status::Complete` → starts work session

**New Behavior**:

- Check both `Status::Complete` AND `self.phase`
- If `Complete` + `Work`: start work session (25:00)
- If `Complete` + `Break`: start break session (5:00)

**State Transition**:

```text
(Work, Complete, 0) + start() → (Work, Running, 25:00)
(Break, Complete, 0) + start() → (Break, Running, 5:00)
```

---

### 3. clear()

**Current Behavior** (lines 218-230):

- Always resets to Work phase with WorkReady status

**New Behavior**:

- Preserve current phase when resetting
- If `phase == Work`: reset to WorkReady with 25:00
- If `phase == Break`: reset to BreakReady with 5:00

**State Transition**:

```text
(Work, Complete, 0) + clear() → (Work, WorkReady, 25:00)
(Break, Complete, 0) + clear() → (Break, BreakReady, 5:00)
```

---

### 4. set_phase()

**Current Behavior** (lines 232-278):

- Handles Running → Paused transition
- Handles Paused → new phase transition
- Sets WorkReady or BreakReady for new phase

**New Behavior**:

- Add handling for `Status::Complete`:
  - Treat like Ready state (no pausing needed)
  - Switch to new phase's ready status
  - Reset to new phase's standard duration
  - Clear completion_flag

**State Transition**:

```text
(Work, Complete, 0) + set_phase(Break) → (Break, BreakReady, 5:00)
(Break, Complete, 0) + set_phase(Work) → (Work, WorkReady, 25:00)
(Work, Complete, 0) + set_phase(Work) → (Work, WorkReady, 25:00) [same phase = reset]
```

---

## Frontend Integration (No Changes Required)

The existing frontend already supports this feature through:

1. **Work/Break Buttons**: Call `set_phase()` IPC command (from feature 006)
2. **Start Button**: Calls `start_timer()` IPC command
3. **Clear Button**: Calls `clear_timer()` IPC command
4. **Completion Indication**: Checks `completion_flag` in `TimerState`

All required manual control mechanisms are already present in the UI.

---

## Edge Case Handling

### Edge Case 1: Pause Near Completion

**Scenario**: User pauses at 00:01, then resumes  
**Expected**: Timer continues to 00:00 and stays in current phase (not auto-switching)  
**Implementation**: `handle_completion()` is called from `update_remaining()` when time reaches 0, regardless of pause history

### Edge Case 2: Clear at 00:00

**Scenario**: User presses Clear when timer shows (Work, Complete, 00:00)  
**Expected**: Reset to (Work, WorkReady, 25:00) without switching to break  
**Implementation**: Modified `clear()` preserves current phase

### Edge Case 3: Start After Completion

**Scenario**: User presses Start when timer shows (Break, Complete, 00:00)  
**Expected**: Start a new break session (Break, Running, 5:00), not a work session  
**Implementation**: Modified `start()` checks both status and phase

### Edge Case 4: Rapid Phase Switching

**Scenario**: User clicks Work → Break → Work buttons rapidly after completion  
**Expected**: Each click updates phase and resets to that phase's ready state  
**Implementation**: `set_phase()` handles Complete status, each call is idempotent

### Edge Case 5: Completion Flag Persistence

**Scenario**: Completion indication should remain visible until user takes action  
**Expected**: `completion_flag` stays true through Complete status, cleared on Start/Clear/set_phase  
**Implementation**: Flag cleared in `start()`, `clear()`, and `set_phase()` methods

---

## Testing Scenarios

### Test 1: Work Completion Stays in Work Mode

```rust
// Start work session, advance to completion
service.start();
advance_time_to_completion();
let state = service.get_state();
assert_eq!(state.phase, Phase::Work);
assert_eq!(state.status, Status::Complete);
assert_eq!(state.remaining_secs, 0);
assert!(state.completion_flag);
```

### Test 2: Break Completion Stays in Break Mode

```rust
// Switch to break, start, advance to completion
service.set_phase(Phase::Break);
service.start();
advance_time_to_completion();
let state = service.get_state();
assert_eq!(state.phase, Phase::Break);
assert_eq!(state.status, Status::Complete);
```

### Test 3: Start After Work Completion Restarts Work

```rust
// Complete work session, then start again
complete_work_session();
service.start();
let state = service.get_state();
assert_eq!(state.phase, Phase::Work);
assert_eq!(state.status, Status::Running);
assert_eq!(state.remaining_secs, 1500); // 25 minutes
```

### Test 4: Clear Preserves Current Phase

```rust
// Complete break session, then clear
complete_break_session();
service.clear();
let state = service.get_state();
assert_eq!(state.phase, Phase::Break);
assert_eq!(state.status, Status::BreakReady);
assert_eq!(state.remaining_secs, 300); // 5 minutes
```

### Test 5: Phase Switch From Complete Status

```rust
// Complete work, then switch to break
complete_work_session();
service.set_phase(Phase::Break);
let state = service.get_state();
assert_eq!(state.phase, Phase::Break);
assert_eq!(state.status, Status::BreakReady);
assert!(!state.completion_flag); // Flag cleared on phase switch
```

---

## Summary

This feature requires **zero new data fields**—only behavioral modifications to existing methods. The implementation leverages the existing `Status::Complete` enum value and `phase` field to represent "completed in current session." All manual control mechanisms (Work/Break buttons, Start, Clear) are already present in the codebase and just need updated logic to handle the Complete status appropriately.
