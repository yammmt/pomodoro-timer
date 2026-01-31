# Data Model: Show Elapsed Time After Session Completion

**Feature**: 009-show-elapsed-time  
**Date**: 2026-01-31  
**Phase**: 1 - Design

## Entities

### TimerService (Backend - Extended)

**Purpose**: Core timer state management with overtime tracking capability.

**Fields**:

- `phase: Phase` - Current timer phase (Work or Break)
- `status: Status` - Current timer status (WorkReady, BreakReady, Running, Paused, Complete)
- `remaining_secs: u32` - Countdown time remaining (0 when complete)
- `duration_secs: u32` - Session duration (1500 for work, 300 for break)
- `completion_flag: bool` - Flag indicating session just completed
- `started_instant: Option<Instant>` - Timestamp when timer started running
- `paused_work_secs: Option<u32>` - Saved work time when paused
- `paused_break_secs: Option<u32>` - Saved break time when paused
- `state_label: String` - Human-readable status label
- **`completed_at: Option<Instant>`** - ⭐ NEW: Timestamp when session completed (for overtime calculation)

**Relationships**: None (single service, no external dependencies)

**State Transitions**:

```text
WorkReady/BreakReady → Running → Complete (with completed_at set)
                                    ↓
                                Overtime Mode (elapsed time displayed)
                                    ↓
                                Running (on start/resume, completed_at cleared)
```

**Validation Rules**:

- `completed_at` MUST be `Some` only when `status == Complete`
- `completed_at` MUST be cleared on transitions to Running, Paused, or Ready states
- Overtime calculation MUST cap at 3599 seconds (59:59)

---

### TimerState (Data Transfer Object)

**Purpose**: Serializable state snapshot sent from backend to frontend.

**Fields**:

- `phase: "work" | "break"` - Current phase
- `status: "workReady" | "breakReady" | "running" | "paused" | "complete"` - Current status
- `remaining_secs: number` - Countdown time
- `duration_secs: number` - Session duration
- `completion_flag: boolean` - Completion indicator
- `started_at: string | null` - Not currently used (legacy field)
- `paused_at: string | null` - Not currently used (legacy field)
- `state_label: string` - Status description
- **`overtime_secs: number | undefined`** - ⭐ NEW: Elapsed overtime in seconds (present only when in overtime mode)

**Relationships**: Frontend consumes this from backend via Tauri IPC

**Validation Rules**:

- `overtime_secs` present ⟹ `status === "complete"`
- `overtime_secs` range: 0 ≤ value ≤ 3599
- When `overtime_secs` is undefined/absent, display normal timer

---

### Overtime Display State (Frontend UI)

**Purpose**: Visual representation of overtime mode.

**Attributes**:

- Display text: `-MM:SS` format (e.g., "-02:13")
- Color: Red (#dc2626 / Tailwind red-600)
- CSS class: `.overtime` applied to timer display element

**Relationships**:

- Derives from `TimerState.overtime_secs`
- Applies to existing `#timer-display` DOM element

**Behavioral Rules**:

- Display updates every 1 second (existing polling interval)
- Minus prefix always shown when overtime active
- Red color applied via `.overtime` CSS class
- Removed when `overtime_secs` becomes undefined (user resumes timer)

---

## State Machine Extension

### Current State Machine (Unchanged)

```text
WorkReady ↔ Running → Paused → Running → Complete
    ↓                                          ↓
BreakReady ↔ Running → Paused → Running → Complete
```

### Overtime Sub-State (New)

```text
Complete (remaining = 0, completed_at = started_instant + duration)
    ↓
Overtime Mode (remaining = 0, overtime_secs calculated)
    ↓ (on start/resume)
Running (completed_at cleared, new session begins)
```

**Key Properties**:

- Overtime is not a new `Status` value
- Overtime is a condition: `status == Complete && completed_at.is_some()`
- Transitions out of overtime: `start()`, `resume()`, `clear()`, or phase switch

---

## Data Flow

### Overtime Entry

```text
Timer reaches 0:00
    ↓
handle_completion() called
    ↓
Set: status = Complete, remaining_secs = 0, completed_at = started_instant + duration
    ↓
Frontend polls get_state()
    ↓
Backend calculates: overtime_secs = now - completed_at (capped at 3599)
    ↓
Frontend receives TimerState with overtime_secs = Some(N)
    ↓
Display: "-MM:SS" in red
```

### Overtime Exit

```text
User clicks Start/Resume
    ↓
Backend: start() or resume() called
    ↓
Set: completed_at = None, status = Running
    ↓
Frontend polls get_state()
    ↓
Backend: overtime_secs = None (not in TimerState)
    ↓
Display: normal "MM:SS" (no red, no minus)
```

---

## Persistence

**No persistence required.**

- Overtime state is transient (resets on app restart)
- Consistent with existing timer behavior (no state persistence)
- `completed_at` timestamp is in-memory only

---

## Edge Cases

### Edge Case 1: Overtime Cap at 59:59

- **Condition**: `elapsed_secs > 3599`
- **Behavior**: `overtime_secs = min(elapsed_secs, 3599)`
- **Display**: Stays at "-59:59" even if more time passes

### Edge Case 2: Clear During Overtime

- **Condition**: User clicks Clear while in overtime mode
- **Behavior**: `clear()` resets to WorkReady/BreakReady, `completed_at = None`
- **Display**: Returns to "25:00" or "05:00" (depending on phase)

### Edge Case 3: Phase Switch During Overtime

- **Condition**: User clicks Work/Break button while in overtime
- **Behavior**: `set_phase()` clears `completed_at`, switches phase
- **Display**: Shows Ready state for new phase ("25:00" or "05:00")

### Edge Case 4: App Minimized During Overtime

- **Condition**: App in background, overtime continues
- **Behavior**: `completed_at` timestamp preserved, calculation accurate on next poll
- **Display**: Shows correct elapsed time when app returns to foreground

---

## Testing Considerations

### Unit Tests (Backend)

- Overtime calculation returns correct values (0s, 30s, 3599s)
- Overtime capped at 3599 (test with 3600s, 7200s)
- `completed_at` cleared on start/resume/clear
- `overtime_secs` absent when not in Complete status

### Integration Tests (Frontend-Backend)

- TimerState serialization includes `overtime_secs` when appropriate
- Frontend receives and parses overtime value correctly

### Manual UI Tests

- Red color applied correctly
- Minus prefix shown
- Updates every second
- Returns to normal display on resume

---

## Summary

The data model extends existing `TimerService` and `TimerState` with minimal additions:

1. Backend: `completed_at: Option<Instant>` for tracking
2. DTO: `overtime_secs: Option<u32>` for frontend communication
3. Frontend: Conditional formatting based on overtime presence

No breaking changes to existing fields or interfaces.
