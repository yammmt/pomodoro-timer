# API Contracts: Timer State Extension for Overtime Display

**Feature**: 009-show-elapsed-time  
**Date**: 2026-01-31  
**Type**: Internal IPC (Tauri Command Interface)

## Overview

This contract defines the extension to the existing TimerState data structure returned by the `get_state` Tauri command. The extension adds overtime tracking capability without breaking existing frontend code.

---

## Extended TimerState Interface

### TypeScript Interface (Frontend)

```typescript
interface TimerState {
  phase: 'work' | 'break';
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  remainingSecs: number;
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
  startedAt?: string;  // Optional, legacy field
  pausedAt?: string;   // Optional, legacy field

  // ⭐ NEW: Overtime tracking
  overtimeSecs?: number;  // Present only when status === 'complete' and overtime is active
}
```

### Rust Struct (Backend)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerState {
    pub phase: Phase,
    pub status: Status,
    pub remaining_secs: u32,
    pub duration_secs: u32,
    pub completion_flag: bool,
    pub state_label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_at: Option<String>,

    // ⭐ NEW: Overtime field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_secs: Option<u32>,
}
```

---

## Field Specifications

### `overtimeSecs` / `overtime_secs`

**Type**: `number | undefined` (TypeScript) / `Option<u32>` (Rust)

**Presence Condition**:

- Present (`Some(value)`) when `status === Complete` AND overtime tracking is active
- Absent (`None`) in all other states (WorkReady, BreakReady, Running, Paused, or Complete without overtime)

**Value Range**: `0` to `3599` (0 seconds to 59 minutes 59 seconds)

**Semantics**: Number of seconds elapsed since the timer completed, capped at maximum display limit of 59:59.

**Calculation**:

```rust
if status == Status::Complete && completed_at.is_some() {
    let elapsed = completed_at.unwrap().elapsed().as_secs() as u32;
    overtime_secs = Some(min(elapsed, 3599));
} else {
    overtime_secs = None;
}
```

**Serialization**:

- When `None`: Field omitted from JSON (via `skip_serializing_if`)
- When `Some(n)`: Field included as `"overtimeSecs": n`

**Example JSON**:

```json
{
  "phase": "work",
  "status": "complete",
  "remainingSecs": 0,
  "durationSecs": 1500,
  "completionFlag": true,
  "stateLabel": "Work completed",
  "overtimeSecs": 133
}
```

---

## Existing Commands (Unchanged)

All existing Tauri commands remain unchanged:

### `get_state`

```typescript
invoke<TimerState>('get_state')
```

Returns: `TimerState` (now with optional `overtimeSecs` field)

### `start_timer`

```typescript
invoke('start_timer')
```

Effect: Clears overtime state if present

### `pause_timer`

```typescript
invoke('pause_timer')
```

Effect: No impact on overtime (only applies when Running)

### `resume_timer`

```typescript
invoke('resume_timer')
```

Effect: Clears overtime state if present

### `clear_timer`

```typescript
invoke('clear_timer')
```

Effect: Clears overtime state, resets to Ready

### `set_phase`

```typescript
invoke('set_phase', { phase: 'work' | 'break' })
```

Effect: Clears overtime state, switches phase

---

## Contract Guarantees

### Backend Guarantees

1. **Presence Consistency**: `overtime_secs` is `Some` ⟺ (`status == Complete` ∧ `completed_at.is_some()`)

2. **Value Bounds**: When present, `0 ≤ overtime_secs ≤ 3599`

3. **Monotonicity**: Within a single overtime session, `overtime_secs` is non-decreasing (until capped)

4. **Cleanup**: `overtime_secs` becomes `None` when:
   - `start_timer` called
   - `resume_timer` called
   - `clear_timer` called
   - `set_phase` called

5. **Serialization**: JSON field name is `overtimeSecs` (camelCase)

### Frontend Guarantees

1. **Optional Handling**: Frontend MUST handle `overtimeSecs` being undefined

2. **Display Logic**:
   - IF `overtimeSecs` is present THEN display `-MM:SS` in red
   - ELSE display `remainingSecs` normally

3. **Type Safety**: Check `state.overtimeSecs !== undefined` before accessing value

---

## Backward Compatibility

**Fully backward compatible.**

- Existing frontend code that doesn't check `overtimeSecs` will continue to work
- Field is optional and omitted when not relevant
- No changes to existing fields or their semantics
- Frontend can be updated independently to support overtime display

---

## Example Usage

### Frontend Display Logic

```typescript
async function updateUI() {
  const state = await invoke<TimerState>('get_state');

  // Check for overtime mode
  if (state.overtimeSecs !== undefined) {
    // Display overtime with red color and minus prefix
    timerDisplay.textContent = `-${formatTime(state.overtimeSecs)}`;
    timerDisplay.classList.add('overtime');
  } else {
    // Normal countdown display
    timerDisplay.textContent = formatTime(state.remainingSecs);
    timerDisplay.classList.remove('overtime');
  }

  // ... rest of UI update logic
}
```

### Backend State Calculation

```rust
pub fn get_state(&mut self) -> TimerState {
    self.update_remaining();

    // Calculate overtime if applicable
    let overtime_secs = if self.status == Status::Complete {
        self.completed_at.map(|completed| {
            let elapsed = completed.elapsed().as_secs() as u32;
            std::cmp::min(elapsed, 3599) // Cap at 59:59
        })
    } else {
        None
    };

    TimerState {
        phase: self.phase,
        status: self.status,
        remaining_secs: self.remaining_secs,
        duration_secs: self.duration_secs,
        completion_flag: self.completion_flag,
        started_at: None,
        paused_at: None,
        state_label: self.state_label.clone(),
        overtime_secs,  // ⭐ NEW field
    }
}
```

---

## Testing Contract Compliance

### Test Cases

1. **Overtime Presence**
   - GIVEN status is Complete with completed_at set
   - WHEN get_state called
   - THEN overtimeSecs is present in response

2. **Overtime Absence**
   - GIVEN status is NOT Complete
   - WHEN get_state called
   - THEN overtimeSecs is absent from response

3. **Value Capping**
   - GIVEN elapsed time is 3700 seconds
   - WHEN get_state called
   - THEN overtimeSecs equals 3599 (not 3700)

4. **Clear on Start**
   - GIVEN overtimeSecs is present
   - WHEN start_timer called
   - THEN next get_state has overtimeSecs absent

5. **JSON Serialization**
   - GIVEN TimerState with overtimeSecs = Some(125)
   - WHEN serialized to JSON
   - THEN JSON contains `"overtimeSecs": 125`

6. **JSON Omission**
   - GIVEN TimerState with overtimeSecs = None
   - WHEN serialized to JSON
   - THEN JSON does NOT contain `overtimeSecs` key

---

## Migration Notes

**No migration required.**

This is a pure extension (adding optional field). Existing code continues to work without changes.

Frontend can optionally upgrade to support overtime display by checking for presence of `overtimeSecs` field.
