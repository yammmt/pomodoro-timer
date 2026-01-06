# Data Model: Show Elapsed Time After Session Completion

**Date**: 2026-01-06  
**Feature**: [specs/007-show-elapsed-time/spec.md](specs/007-show-elapsed-time/spec.md)  
**Research**: [research.md](research.md)

## Overview

Add elapsed-time tracking after a session completes. Backend keeps an accumulating elapsed clock with pause/resume support; frontend renders elapsed display in red until the user clears. Start/Resume remain disabled while elapsed is active to encourage explicit clear before starting the next session.

---

## Entities

### 1) TimerService (Rust backend)

**Location**: `src-tauri/src/timer.rs`

**New/Updated Fields**:

```rust
pub struct TimerService {
    pub(crate) phase: Phase,                      // Active/next session phase (work or break)
    pub(crate) status: Status,                    // workReady | breakReady | running | paused | complete
    pub(crate) remaining_secs: u32,               // Countdown remaining for active phase
    duration_secs: u32,                           // Total duration for active phase
    completion_flag: bool,                        // Completion edge trigger for chime/UI
    pub(crate) started_instant: Option<Instant>,  // Countdown start/resume reference
    pub(crate) paused_work_secs: Option<u32>,     // Paused remaining for work
    pub(crate) paused_break_secs: Option<u32>,    // Paused remaining for break
    state_label: String,                          // User-facing label
    // NEW for elapsed tracking
    pub(crate) last_completed_phase: Option<Phase>, // Which phase just finished
    pub(crate) elapsed_started_instant: Option<Instant>, // When elapsed counting began/resumed
    pub(crate) elapsed_paused_secs: u32,           // Accumulated elapsed seconds when paused
    pub(crate) elapsed_running: bool,              // Whether elapsed clock is ticking
}
```

**Behavior Changes**:
- On completion, do **not** reset to ready immediately. Set `status = Status::Complete`, set `phase` to the *next* session (work→break, break→work), record `last_completed_phase`, and start elapsed clock (`elapsed_running = true`, `elapsed_started_instant = Some(now)`).
- `clear()` zeroes elapsed fields, resets `status` and timers to the upcoming phase (`phase` already points to next), and clears `completion_flag`.
- `pause()` when `status == Complete` pauses elapsed by moving delta into `elapsed_paused_secs` and clearing `elapsed_started_instant`.
- `resume()` when `status == Complete` restarts elapsed by setting `elapsed_started_instant = Some(now)` and `elapsed_running = true`.
- `start()` only allowed from ready states; reject `Status::Complete` to enforce explicit clear before starting next session.

### 2) TimerState (IPC payload)

**Location**: `src-tauri/src/timer.rs`

**Additions**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub phase: Phase,                 // Next/active phase
    pub status: Status,               // includes Complete
    pub remaining_secs: u32,
    pub duration_secs: u32,
    pub completion_flag: bool,
    pub state_label: String,
    pub elapsed_secs: Option<u32>,    // NEW: elapsed since completion (None if not in elapsed mode)
    pub elapsed_running: bool,        // NEW: true when elapsed clock ticking
    pub last_completed_phase: Option<Phase>, // NEW: for UI label/iconography
}
```

### 3) UI Display Model (frontend)

**Location**: `src/main.ts`, `src/index.html`

**Display Rules**:
- When `status === 'complete'` and `elapsed_secs` is present, show `-MM:SS` from `elapsed_secs` in red (`#ef4444`) with bold text. Ignore `remainingSecs` in this state.
- `stateLabel` should indicate completion (e.g., "Work session completed" or "Break completed").
- Buttons: Start disabled in `complete`; Pause toggles elapsed pause; Resume resumes elapsed; Clear remains enabled.

---

## State Transitions

### Completion → Elapsed

```
Before completion:
  phase=Work, status=Running, remaining_secs=0..n

On completion event:
  last_completed_phase=Work
  phase=Break (next phase)
  status=Complete
  elapsed_running=true
  elapsed_paused_secs=0
  elapsed_started_instant=now
  completion_flag=true (for chime edge)

Frontend:
  Display -00:01 in red, Start disabled, Pause enabled, Clear enabled
```

### Pause elapsed

```
Input: status=Complete, elapsed_running=true, elapsed_started_instant=Some(t0)
Action: pause()
Result:
  elapsed_paused_secs += now - t0
  elapsed_started_instant=None
  elapsed_running=false
  state_label="Paused (elapsed)"
```

### Resume elapsed

```
Input: status=Complete, elapsed_running=false
Action: resume()
Result:
  elapsed_started_instant=Some(now)
  elapsed_running=true
  state_label="Elapsed running"
```

### Clear after completion

```
Input: status=Complete, phase=Break (next), last_completed_phase=Work
Action: clear()
Result:
  elapsed_* reset to 0/None
  completion_flag=false
  phase=Break
  status=BreakReady
  remaining_secs=300, duration_secs=300
  state_label="Break ready - press Start"
```

---

## Validation Rules

1. `elapsed_secs` present **only** when status == Complete.
2. `start()` returns error when status == Complete; requires `clear()` first.
3. `completion_flag` stays true for the first `get_state` after completion (for chime), then resets.
4. `elapsed_secs` computed as `elapsed_paused_secs + (now - elapsed_started_instant)` when running, else `elapsed_paused_secs`.
5. Max display cap aligns with UI expectation: clamp to 99:59 for rendering; backend may keep counting without overflow.

---

## Testing Considerations

- Unit tests for completion → elapsed start, pause/resume of elapsed, clear after completion, and start rejection in Complete state.
- Ensure `get_state` returns consistent `elapsed_secs` values across consecutive polls (monotonic increase when running; stable when paused).
- Verify `completion_flag` set once and cleared on subsequent `get_state` calls.
- UI-level tests: status=Complete shows red elapsed text; Start disabled; Clear resets to Ready with correct next phase.
