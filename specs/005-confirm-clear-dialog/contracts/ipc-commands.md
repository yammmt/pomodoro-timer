# API Contract: Clear Timer Command

**Scope**: Tauri IPC command for clearing the timer when user confirms the dialog.

## Command Signature

```typescript
// Frontend (TypeScript) invocation
const response = await invoke<TimerStateResponse>('clear_timer', {});

// Backend (Rust) handler
#[tauri::command]
fn clear_timer(state: tauri::State<'_, AppState>) -> TimerStateResponse { ... }
```

## Request

**Command Name**: `clear_timer`

**Parameters**: None (empty object)

**Preconditions**:
- Timer state is initialized (always true in running app)
- User has confirmed the dialog or skipped it (implicit; command only called after dialog interaction)

**Permissions**: Standard command capability (no elevated privileges needed)

---

## Response

**Type**: `TimerStateResponse`

```typescript
interface TimerStateResponse {
  success: boolean;           // true if clear succeeded, false if validation failed
  status: "idle" | "running" | "paused";  // New timer status after clear
  remaining_time: number;     // Remaining time in milliseconds (0 if cleared)
  mode: "work" | "break";     // Work or break mode (unchanged by clear)
  error?: string;             // Optional error message if success = false
}
```

**Example Success**:
```json
{
  "success": true,
  "status": "idle",
  "remaining_time": 0,
  "mode": "work"
}
```

**Example Error** (hypothetical validation failure):
```json
{
  "success": false,
  "status": "paused",
  "remaining_time": 45000,
  "mode": "work",
  "error": "Timer is not in a clearable state"
}
```

---

## Semantics

### On Success (success = true)

- Timer state has been reset to idle (remaining_time = 0, status = "idle")
- Mode and initial duration are preserved (not cleared)
- Frontend should update display to show idle state
- Dialog is hidden by frontend

### On Failure (success = false)

- Timer state is unchanged (returned state = current state before attempt)
- Error message explains why clear failed
- Frontend can log error or show user-facing message (optional; should rarely occur)

---

## Error Cases

| Scenario | Likelihood | Handling |
|----------|-----------|----------|
| Timer state corrupted (should never happen) | Extremely rare | Return success=false with error; frontend logs and recovers to current state |
| Command called while UI not initialized | Nearly impossible in normal flow | Tauri framework prevents this; command only registered after app init |
| User network/IPC failure | Very rare in Tauri (local IPC) | Tauri invoke() rejects promise; frontend can retry or show error |

---

## Implementation Notes

### Rust Backend (src-tauri/src/lib.rs)

```rust
#[tauri::command]
fn clear_timer(state: tauri::State<'_, AppState>) -> TimerStateResponse {
    let mut timer = state.timer.lock().unwrap();
    
    // Validate (optional; clear is idempotent)
    if timer.remaining_time > 0 && timer.status == TimerStatus::Idle {
        // Unusual but valid: idle timer with time (shouldn't happen in normal flow)
        // Still allow clear; it's a no-op
    }
    
    // Clear
    timer.clear();
    
    // Return new state
    TimerStateResponse {
        success: true,
        status: format!("{:?}", timer.status),
        remaining_time: timer.remaining_time,
        mode: format!("{:?}", timer.mode),
        error: None,
    }
}
```

### TypeScript Frontend (src/main.ts)

```typescript
async function handleConfirmClear() {
    try {
        const response = await invoke<TimerStateResponse>('clear_timer', {});
        if (response.success) {
            updateTimerDisplay(response);
            hideConfirmDialog();
        } else {
            console.error('Clear failed:', response.error);
            // Optionally: show user message or retry
        }
    } catch (error) {
        console.error('IPC error:', error);
        // Tauri invoke() rejected; unlikely in normal flow
    }
}
```

---

## Backward Compatibility

- **New command**: Does not break existing commands (start, pause, get_state).
- **Existing timers**: Clear operation is idempotent and non-destructive to mode/duration.
- **Future extensions**: Response object can add new fields without breaking existing code (TypeScript optional properties).

---

## Testing

### Unit Test (Rust)

```rust
#[test]
fn test_clear_timer_command() {
    let mut timer = Timer::new();
    timer.mode = TimerMode::Work;
    timer.remaining_time = 25 * 60 * 1000;  // 25 minutes
    timer.status = TimerStatus::Running;
    
    timer.clear();
    
    assert_eq!(timer.remaining_time, 0);
    assert_eq!(timer.status, TimerStatus::Idle);
    assert_eq!(timer.mode, TimerMode::Work);  // Mode preserved
}
```

### Integration Test (Manual)

1. Start a 25-minute work session (remaining_time = 25*60*1000, status = running)
2. Click Clear button
3. Confirm in dialog
4. Verify response: success=true, remaining_time=0, status=idle
5. Verify UI: timer displays 00:00, no playback indicator
