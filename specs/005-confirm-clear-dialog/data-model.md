# Data Model: Confirm Dialog for Clear Action

**Feature**: [spec.md](spec.md) | **Research**: [research.md](research.md)

## Domain Model

### Timer Session Entity

**Represents**: Current work or break segment with mutable state.

**Attributes**:

- `mode`: enum \`"work" | "break"\` — Which phase of the pomodoro cycle is active
- `remaining_time`: number (milliseconds) — Countdown value; 0 = idle, >0 = active session
- `status`: enum \`"idle" | "running" | "paused"\` — Current playback state
- `initial_duration`: number (milliseconds) — Original duration of session (e.g., 25 min for work, 5 min for break)

**Relationships**:

- No external entities; timer state is self-contained within the app.

**State Transitions**:

```
idle ←─→ running  (user taps Play/Pause)
      ↓
      paused  (user taps Pause while running)
      ↑
      └─ running (user taps Play while paused)

[Any state] → idle  (user taps Clear and confirms dialog)
```

**Validation Rules**:

- `remaining_time >= 0` (never negative)
- `remaining_time <= initial_duration` (never exceeds session length)
- If `remaining_time == 0`, status must be `"idle"`
- If `remaining_time > 0`, status can be any of `"idle"`, `"running"`, `"paused"` (transient states; normally running or paused)

---

### Dialog State (Frontend Only)

**Represents**: Confirmation dialog visibility and user interaction.

**Attributes**:

- `dialog_visible`: boolean — Whether confirm dialog is displayed
- `dialog_mode`: enum \`"confirm_clear" | null\` — Which action triggered the dialog (currently only clear; extensible for future destructive actions)
- `pending_action`: function or command name — Callback to execute if user confirms

**Lifecycle**:

1. User taps Clear button on timer with remaining_time > 0
2. Frontend sets dialog_visible = true, dialog_mode = "confirm_clear"
3. Dialog displays with message and Confirm/Cancel buttons
4. User selects Confirm → execute pending_action (call clear_timer command)
5. User selects Cancel or Escape → set dialog_visible = false, dialog_mode = null, pending_action = null

**Validation Rules**:

- If `dialog_visible == true`, dialog_mode must be set (never null)
- If `dialog_visible == false`, dialog_mode and pending_action can be null
- Dialog modal state is client-side ephemeral; no persistence

---

## Clear Operation

**Effect**: Reset timer session to initial idle state.

**Precondition**:

- Timer is in any state (idle, running, or paused)

**Actions**:

1. Set `status = "idle"`
2. Set `remaining_time = 0`
3. Preserve `mode` (work/break stays the same) and `initial_duration` (for potential future replay)
4. Close dialog (set `dialog_visible = false`)

**Postcondition**:

- Timer displays idle state (time = 00:00, status = stopped)
- Dialog is no longer visible
- Next user action: tap Start to begin a new session or change mode

**Idempotency**: Clearing an already-idle timer is a no-op (state unchanged). No error.

---

## Data Flow Diagram

```
┌─────────────────────────────────────────────────────────┐
│ Frontend (HTML/CSS/TypeScript in src/)                  │
├─────────────────────────────────────────────────────────┤
│ 1. Display timer: remaining_time, status                │
│ 2. User clicks Clear button                             │
│    └─ Check: remaining_time > 0?                        │
│       ├─ YES: Show dialog, dialog_visible = true        │
│       └─ NO: Skip dialog, call clear directly           │
│ 3. User responds to dialog:                             │
│    ├─ Confirm → call clear_timer command               │
│    └─ Cancel → hide dialog, keep state                  │
└─────────────────────────────────────────────────────────┘
         ↓ Tauri IPC ↑
┌─────────────────────────────────────────────────────────┐
│ Backend (Rust in src-tauri/src/)                        │
├─────────────────────────────────────────────────────────┤
│ clear_timer command:                                    │
│   1. Get current timer state from timer.rs module       │
│   2. Validate: remaining_time > 0 (if dialog shown)     │
│   3. Call timer.clear() → reset state to idle           │
│   4. Return new state (status, remaining_time) to UI    │
└─────────────────────────────────────────────────────────┘
         ↑ Return ↓
┌─────────────────────────────────────────────────────────┐
│ Frontend updates display:                               │
│ - remaining_time = 0                                    │
│ - status = "idle"                                       │
│ - Dialog hidden                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Impact on Existing Entities

### Timer (src-tauri/src/timer.rs)

**New method**:

```rust
pub fn clear(&mut self) {
    self.status = TimerStatus::Idle;
    self.remaining_time = 0;
    // mode and initial_duration preserved
}
```

**Existing methods** (no change needed):

- `start()`, `pause()`, `resume()`, `tick()` — unaffected

### Commands (src-tauri/src/lib.rs)

**Existing command (reused)**:

```rust
#[tauri::command]
fn clear_timer(timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.clear()
}
```

**Other commands** (unchanged):

- `get_state`, `start_timer`, `pause_timer`, `resume_timer`

### UI (src/index.html, src/main.ts)

**New DOM elements**:

- `<div id="clear-confirm-dialog">` — modal container
- Button `<button id="confirm-clear">Clear</button>`
- Button `<button id="cancel-clear">Cancel</button>`

**New event handlers** (main.ts):

- `clearButton.click()` → show dialog or call clear directly
- `confirmButton.click()` → invoke clear_timer command
- `cancelButton.click()` or `dialog.keydown(Escape)` → hide dialog

**Existing elements** (no change needed):

- Clear button label stays the same
- Timer display (remaining_time, status) unaffected

---

## No New Persistence or External Dependencies

- **Storage**: Dialog visibility is ephemeral; no localStorage or database writes.
- **APIs**: Only tauri IPC and native browser APIs (DOM, events, flexbox layout).
- **Databases**: None.
- **External services**: None.

**Simplicity**: Single modal dialog, synchronous command, no async side effects.
