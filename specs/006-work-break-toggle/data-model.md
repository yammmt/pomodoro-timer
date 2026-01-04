# Data Model: Work/Break Mode Toggle

**Date**: 2026-01-04  
**Feature**: [specs/006-work-break-toggle/spec.md](specs/006-work-break-toggle/spec.md)  
**Research**: [research.md](research.md)

## Overview

This document defines the data structures and state transitions required for work/break mode switching feature.

---

## Entities

### 1. TimerService (Rust Backend)

**Location**: `src-tauri/src/timer.rs`

**Updated Fields**:

```rust
pub struct TimerService {
    pub(crate) phase: Phase,                    // Current phase: Work or Break
    pub(crate) status: Status,                  // Current status: Ready, Running, Paused, Complete
    pub(crate) remaining_secs: u32,             // Remaining seconds for current phase
    duration_secs: u32,                         // Total duration for current phase
    completion_flag: bool,                      // Flag when phase completes
    pub(crate) started_instant: Option<Instant>, // When current run started
    pub(crate) paused_work_secs: Option<u32>,   // **NEW**: Paused remaining time for Work phase
    pub(crate) paused_break_secs: Option<u32>,  // **NEW**: Paused remaining time for Break phase
    state_label: String,                        // Human-readable status
}
```

**Key Changes**:
- Replaced single `paused_remaining: Option<u32>` with `paused_work_secs` and `paused_break_secs` to track both phases independently.
- Allows switching back and forth while preserving each phase's paused time.

---

### 2. Phase Enum

**Location**: `src-tauri/src/timer.rs`

**Existing Definition** (no changes):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Work,
    Break,
}
```

**Usage**: Represents the current session mode selected by the user.

---

### 3. Status Enum

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
    Complete,     // Phase completed
}
```

**Usage**: Transitions on mode switch:
- Running → Paused (via `set_phase`)
- `WorkReady`/`BreakReady` → opposite phase's Ready state (via `set_phase`)

---

### 4. ModeButton (Frontend UI Component)

**Location**: `src/index.html` + `src/main.ts`

**Conceptual Structure**:

```html
<div id="mode-selector">
    <button id="work-btn" class="mode-btn active">Work</button>
    <button id="break-btn" class="mode-btn">Break</button>
</div>
```

**CSS Classes**:
- `.mode-btn`: Base button styling
- `.active`: Emphasized styling for the currently selected phase

**State Binding**:
- Class `.active` added to button matching `TimerState.phase`
- Event listeners invoke `set_phase` command when clicked

---

## State Transitions

### Scenario 1: Initialize & Select Work (Default)

```
Initial State: phase=Work, status=WorkReady, remaining_secs=1500
User clicks Work button (already active): No change, idempotent
Frontend display: Work button emphasized, timer shows 25:00
```

### Scenario 2: Switch from Work (paused) to Break

```
Before Switch:
  phase=Work, status=Paused, remaining_secs=1200
  paused_work_secs=1200, paused_break_secs=None

User clicks Break button:
1. Backend set_phase(Break) called
2. Save paused_work_secs = remaining_secs (1200)
3. Switch phase to Break
4. Load paused_break_secs (None → use standard 300 secs)
5. Update status to BreakReady
6. Return new state

After Switch:
  phase=Break, status=BreakReady, remaining_secs=300
  paused_work_secs=1200, paused_break_secs=None

Frontend display: Break button emphasized, timer shows 05:00
```

### Scenario 3: Switch Back to Work (Preserve Paused Time)

```
Before Switch:
  phase=Break, status=Paused, remaining_secs=180
  paused_work_secs=1200, paused_break_secs=180

User clicks Work button:
1. Backend set_phase(Work) called
2. Save paused_break_secs = remaining_secs (180)
3. Switch phase to Work
4. Load paused_work_secs (1200 secs)
5. Update status to WorkReady
6. Return new state

After Switch:
  phase=Work, status=WorkReady, remaining_secs=1200
  paused_work_secs=1200, paused_break_secs=180

Frontend display: Work button emphasized, timer shows 20:00 (1200 secs)
```

### Scenario 4: Running Timer, Mid-Switch

```
Before Switch:
  phase=Work, status=Running, remaining_secs=1100
  started_instant=Some(T-400s), paused_remaining=None

User clicks Break button:
1. Backend pauses running timer (copies remaining_secs to paused_work_secs)
2. Save paused_work_secs = 1100
3. Switch phase to Break
4. Clear started_instant, load paused_break_secs
5. Update status to BreakReady
6. Return new state

After Switch:
  phase=Break, status=BreakReady, remaining_secs=300
  paused_work_secs=1100, paused_break_secs=None
```

---

## Validation Rules

1. **Idempotency**: If `new_phase == self.phase`, return without modification.
2. **Time Preservation**: When switching, save current `remaining_secs` to the exiting phase's field before changing phases.
3. **Status Updates**: Always transition to appropriate `-Ready` status after `set_phase`.
4. **Duration Reset**: When switching to a new phase, reset `duration_secs` to match phase (Work=1500, Break=300).

---

## Testing Considerations

- Unit tests for `set_phase()` covering:
  - Idempotent calls (click same button twice)
  - Running → switch → preserved time
  - Paused → switch → back (preserves both directions)
  - Status transitions on switch
  - Duration updates match phase

---

## Related Files

- [contracts/ipc-commands.md](contracts/ipc-commands.md) – `set_phase` command interface
- [quickstart.md](quickstart.md) – Frontend setup steps
