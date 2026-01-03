# API Contract: Tauri IPC Commands

**Feature**: Manual Break Start  
**Version**: 1.0.0  
**Protocol**: Tauri IPC (JSON-RPC style)  
**Date**: 2026-01-03

## Overview

This contract defines the IPC commands between the Tauri frontend (TypeScript) and backend (Rust) for the Pomodoro timer with manual break start functionality.

## Commands

### get_state

**Description**: Retrieves the current timer state

**Direction**: Frontend → Backend

**Parameters**: None

**Returns**: `TimerState` object

**Success Response**:
```json
{
  "phase": "work" | "break",
  "status": "workReady" | "breakReady" | "running" | "paused" | "complete",
  "remainingSecs": number,
  "durationSecs": number,
  "completionFlag": boolean,
  "stateLabel": string
}
```

**Error Response**: N/A (always succeeds, returns current state)

**Example**:
```typescript
const state = await invoke<TimerState>('get_state');
// Returns: { phase: 'work', status: 'workReady', remainingSecs: 1500, ... }
```

**Side Effects**: Updates `remaining_secs` if timer is running (recalculates based on elapsed time)

---

### start_timer

**Description**: Starts a timer countdown based on current phase

**Direction**: Frontend → Backend

**Parameters**: None

**Returns**: `TimerState` object (updated state after start)

**Success Response**:
```json
{
  "phase": "work" | "break",
  "status": "running",
  "remainingSecs": number,
  "durationSecs": number,
  "completionFlag": false,
  "stateLabel": "Working" | "Break time"
}
```

**Error Response**:
```json
{
  "error": "Timer already running"
}
```

**Behavior**:
- If status is `workReady`: Starts work countdown (1500 seconds)
- If status is `breakReady`: Starts break countdown (300 seconds)
- If status is `running`: Returns error
- If status is `paused`: Returns error (use `resume_timer` instead)
- If status is `complete`: Treats as `workReady` and starts work

**Example**:
```typescript
// From workReady state
await invoke('start_timer');
// Returns: { phase: 'work', status: 'running', remainingSecs: 1500, ... }

// From breakReady state
await invoke('start_timer');
// Returns: { phase: 'break', status: 'running', remainingSecs: 300, ... }
```

**Side Effects**: 
- Sets `completion_flag` to `false`
- Records start time for countdown calculations
- Clears any paused state

---

### pause_timer

**Description**: Pauses the currently running timer

**Direction**: Frontend → Backend

**Parameters**: None

**Returns**: `TimerState` object (updated state after pause)

**Success Response**:
```json
{
  "phase": "work" | "break",
  "status": "paused",
  "remainingSecs": number,
  "durationSecs": number,
  "completionFlag": false,
  "stateLabel": "Paused (work)" | "Paused (break)"
}
```

**Error Response**:
```json
{
  "error": "No running timer to pause"
}
```

**Behavior**:
- If status is `running`: Pauses and preserves `remaining_secs`
- If status is not `running`: Returns error

**Example**:
```typescript
// During running work session
await invoke('pause_timer');
// Returns: { phase: 'work', status: 'paused', remainingSecs: 1432, ... }
```

**Side Effects**: 
- Saves current `remaining_secs` to resume from later
- Clears start time tracking

---

### resume_timer

**Description**: Resumes a paused timer from where it was paused

**Direction**: Frontend → Backend

**Parameters**: None

**Returns**: `TimerState` object (updated state after resume)

**Success Response**:
```json
{
  "phase": "work" | "break",
  "status": "running",
  "remainingSecs": number,
  "durationSecs": number,
  "completionFlag": false,
  "stateLabel": "Working" | "Break time"
}
```

**Error Response**:
```json
{
  "error": "No paused timer to resume"
}
```

**Behavior**:
- If status is `paused`: Resumes countdown from saved `remaining_secs`
- If status is not `paused`: Returns error

**Example**:
```typescript
// From paused state
await invoke('resume_timer');
// Returns: { phase: 'work', status: 'running', remainingSecs: 1432, ... }
```

**Side Effects**: 
- Restarts countdown from paused remaining time
- Records new start time for calculations

---

### clear_timer

**Description**: Resets timer to initial work-ready state

**Direction**: Frontend → Backend

**Parameters**: None

**Returns**: `TimerState` object (reset state)

**Success Response**:
```json
{
  "phase": "work",
  "status": "workReady",
  "remainingSecs": 1500,
  "durationSecs": 1500,
  "completionFlag": false,
  "stateLabel": "Ready to work"
}
```

**Error Response**: N/A (always succeeds)

**Behavior**:
- Works from any state
- Always resets to work-ready state
- Clears all timing state and flags

**Example**:
```typescript
// From any state
await invoke('clear_timer');
// Returns: { phase: 'work', status: 'workReady', remainingSecs: 1500, ... }
```

**Side Effects**: 
- Clears `completion_flag`
- Clears all internal timing state
- Resets phase to Work

---

## Data Types

### TimerState

```typescript
interface TimerState {
  phase: 'work' | 'break';
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  remainingSecs: number;
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
}
```

**Field Descriptions**:
- `phase`: Current timer phase (work or break)
- `status`: Current execution status (see Status enum in data-model.md)
- `remainingSecs`: Seconds remaining in current/next countdown
- `durationSecs`: Total duration for current phase (1500 or 300)
- `completionFlag`: True when a session just completed (triggers chime)
- `stateLabel`: Human-readable state for UI display

---

## State Transition Events

The backend automatically handles state transitions when countdown reaches zero:

### Work Completion
**Trigger**: `remaining_secs` reaches 0 while `status=running` and `phase=work`

**Transition**: 
```
{ phase: 'work', status: 'running' } 
  → 
{ phase: 'break', status: 'breakReady', completionFlag: true }
```

**No command needed** - automatic on countdown completion

---

### Break Completion
**Trigger**: `remaining_secs` reaches 0 while `status=running` and `phase=break`

**Transition**: 
```
{ phase: 'break', status: 'running' } 
  → 
{ phase: 'work', status: 'workReady', completionFlag: true }
```

**No command needed** - automatic on countdown completion

---

## Usage Patterns

### Pattern 1: Start Work Session
```typescript
// Initial state: workReady
const state = await invoke<TimerState>('get_state');
// state.status === 'workReady'

await invoke('start_timer');
// Now: status === 'running', phase === 'work'
```

### Pattern 2: Complete Work → Manual Break Start
```typescript
// Poll until work completes
const state = await invoke<TimerState>('get_state');
if (state.completionFlag && state.status === 'breakReady') {
  playChime(); // Frontend plays chime
}

// User decides when to start break
await invoke('start_timer');
// Now: status === 'running', phase === 'break'
```

### Pattern 3: Pause and Resume
```typescript
// During work
await invoke('pause_timer');
// status === 'paused'

// Later...
await invoke('resume_timer');
// status === 'running', continues from paused time
```

### Pattern 4: Skip Break
```typescript
// After work completes
const state = await invoke<TimerState>('get_state');
// state.status === 'breakReady'

// User wants to skip break
await invoke('clear_timer');
// Back to: status === 'workReady', ready to start new work session
```

### Pattern 5: Frontend Polling
```typescript
// Poll every second to update UI
setInterval(async () => {
  const state = await invoke<TimerState>('get_state');
  updateDisplay(state);
  
  // Detect completion and play chime
  if (state.completionFlag && !lastCompletionFlag) {
    playChime();
  }
  lastCompletionFlag = state.completionFlag;
}, 1000);
```

---

## Error Handling

All commands return `Result<TimerState, String>` in Rust, which translates to:
- **Success**: Resolves with `TimerState` object
- **Error**: Rejects with error message string

**Frontend Example**:
```typescript
try {
  await invoke('start_timer');
} catch (error) {
  console.error('Failed to start timer:', error);
  // error === "Timer already running"
}
```

---

## Backwards Compatibility

**Breaking Changes from Previous Version**:
- Status value `idle` renamed to `workReady`
- Status value `complete` may be deprecated (replaced by ready states)
- Work completion no longer auto-starts break (breaks existing behavior)

**Migration**: No API compatibility needed - internal desktop app

---

## Testing Contract

### Test Cases for Each Command

**get_state**:
- Returns valid TimerState at all times
- Updates remaining_secs when running

**start_timer**:
- From workReady: starts work countdown
- From breakReady: starts break countdown
- From running: returns error
- From paused: returns error

**pause_timer**:
- From running: pauses and preserves time
- From non-running: returns error

**resume_timer**:
- From paused: resumes countdown
- From non-paused: returns error

**clear_timer**:
- From any state: resets to workReady

**Auto-transitions**:
- Work completion → breakReady (not auto-start)
- Break completion → workReady (not auto-start)
- completion_flag set on transitions

---

## Version History

- **1.0.0** (2026-01-03): Initial contract for manual break start feature
  - Added `workReady` and `breakReady` status values
  - Changed auto-transition behavior (no auto-start)
  - Defined completion event handling
