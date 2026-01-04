# IPC Commands: Work/Break Mode Toggle

**Date**: 2026-01-04  
**Feature**: [specs/006-work-break-toggle/spec.md](specs/006-work-break-toggle/spec.md)

---

## Command: set_phase

### Purpose
Switch the active session mode between Work and Break while preserving the paused remaining time of each phase.

### IPC Signature

```rust
#[tauri::command]
fn set_phase(phase: String, timer: tauri::State<SharedTimerService>) -> Result<TimerState, String>
```

### Request

**Method**: `invoke('set_phase', { phase })`

**Parameters**:

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `phase` | String | Yes | Target phase: `"work"` or `"break"` (case-insensitive). |

**Example Request** (TypeScript):

```typescript
const newState = await invoke<TimerState>('set_phase', { phase: 'break' });
```

### Response

**Type**: `Result<TimerState, String>`

**Success Response**:

Returns `TimerState` JSON object with updated fields:

```typescript
interface TimerState {
  phase: 'work' | 'break';           // New active phase
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  remainingSecs: number;              // Remaining seconds for new phase
  durationSecs: number;               // Total duration of new phase
  completionFlag: boolean;            // Whether phase completed
  stateLabel: string;                 // Human-readable status
}
```

**Example Success Response**:

```json
{
  "phase": "break",
  "status": "breakReady",
  "remainingSecs": 300,
  "durationSecs": 300,
  "completionFlag": false,
  "stateLabel": "Ready to break"
}
```

**Error Response**:

Returns error string if phase is invalid or backend fails:

```
"Invalid phase"
"Failed to lock timer service"
```

---

## Behavior Specification

### Semantics

1. **Idempotent**: If the requested `phase` matches the current phase, the command returns without modifying state.
   
   ```
   Current: phase=Work, remaining_secs=1200, status=Paused
   Request: set_phase(work)
   Response: Same state, no change
   ```

2. **Pause on Switch**: If the current phase is Running, it is paused before switching.
   
   ```
   Current: phase=Work, status=Running, remaining_secs=1100
   Request: set_phase(break)
   After: phase=Break, status=BreakReady, paused_work_secs=1100
   ```

3. **Preserve Paused Time**: The remaining time of the paused phase is stored and restored when switching back.
   
   ```
   Sequence:
   1. Work, Paused, 1200 secs
   2. set_phase(break)
   3. Break, BreakReady, 300 secs (standard)
   4. set_phase(work)
   5. Work, WorkReady, 1200 secs (restored from storage)
   ```

4. **Status Transition**: New phase always enters its corresponding Ready state.
   - `set_phase(work)` → status = WorkReady
   - `set_phase(break)` → status = BreakReady

5. **Duration Reset**: `durationSecs` always reflects the full duration of the new phase.
   - Work phase: 1500 seconds
   - Break phase: 300 seconds

---

## Usage in Frontend

### Invoking the Command

```typescript
// Switch to break when user clicks break button
breakBtn.addEventListener('click', async () => {
  try {
    const newState = await invoke<TimerState>('set_phase', { phase: 'break' });
    updateUIFromState(newState);
    // Toggle active class
    workBtn.classList.remove('active');
    breakBtn.classList.add('active');
  } catch (error) {
    console.error('Failed to set phase:', error);
  }
});
```

### UI Update Flow

1. User clicks Work or Break button
2. Frontend calls `invoke('set_phase', { phase: '...' })`
3. Backend updates state and returns new `TimerState`
4. Frontend updates display (timer, buttons, labels)
5. Add `.active` class to corresponding button

---

## Error Handling

| Condition | Error | Recovery |
|-----------|-------|----------|
| Invalid phase string (not "work" or "break") | `"Invalid phase"` | Log and retry with correct phase |
| Timer service lock fails | `"Failed to lock timer service"` | Retry after short delay |
| Network/IPC failure | Tauri error | User sees no change; retry on next button click |

---

## Backward Compatibility

This command is new and does not affect existing commands:
- `get_state()` – unaffected
- `start_timer()` – still starts current phase
- `pause_timer()` – still pauses current timer
- `resume_timer()` – still resumes current timer
- `clear_timer()` – still clears current phase

The `set_phase` command is purely additive.

---

## Testing Scenarios

### Test 1: Simple Mode Switch
- Initial: phase=Work, status=WorkReady
- Call: `set_phase('break')`
- Expected: phase=Break, status=BreakReady, remainingSecs=300

### Test 2: Preserve Paused Time (Work → Break → Work)
- Initial: phase=Work, status=Paused, remainingSecs=1200
- Call: `set_phase('break')`
- Result: phase=Break, remainingSecs=300
- Call: `set_phase('work')`
- Expected: phase=Work, remainingSecs=1200 (restored)

### Test 3: Running Timer Pause on Switch
- Initial: phase=Work, status=Running, elapsed=400s
- Call: `set_phase('break')`
- Expected: phase=Break, status=BreakReady, remainingSecs=300
- Verify: Work paused time stored as ~1100s

### Test 4: Idempotent Call
- Initial: phase=Work, status=Paused, remainingSecs=1200
- Call: `set_phase('work')`
- Expected: No change, same state returned

### Test 5: Case Insensitivity
- Call: `set_phase('WORK')` or `set_phase('Break')`
- Expected: Both parsed correctly and execute

---

## Related Files

- [data-model.md](../data-model.md) – TimerService structure and state machine
- [quickstart.md](../quickstart.md) – Frontend implementation steps
