# Quickstart: Work/Break Mode Toggle

**Date**: 2026-01-04  
**Feature**: [specs/006-work-break-toggle/spec.md](specs/006-work-break-toggle/spec.md)  
**Data Model**: [data-model.md](data-model.md)  
**IPC Contracts**: [contracts/ipc-commands.md](contracts/ipc-commands.md)

---

## Overview

This guide provides step-by-step instructions to implement the work/break mode toggle feature.

---

## Part 1: Rust Backend (Timer Service)

### Step 1.1: Update TimerService Structure

**File**: `src-tauri/src/timer.rs`

Replace the single `paused_remaining` field with two phase-specific fields:

```rust
// OLD
pub(crate) paused_remaining: Option<u32>,

// NEW
pub(crate) paused_work_secs: Option<u32>,
pub(crate) paused_break_secs: Option<u32>,
```

Update the `new()` constructor:

```rust
impl TimerService {
    pub fn new() -> Self {
        Self {
            phase: Phase::Work,
            status: Status::WorkReady,
            remaining_secs: WORK_DURATION_SECS,
            duration_secs: WORK_DURATION_SECS,
            completion_flag: false,
            started_instant: None,
            paused_work_secs: None,      // NEW
            paused_break_secs: None,      // NEW
            state_label: "Ready to work".to_string(),
        }
    }
}
```

### Step 1.2: Update Existing Methods to Use New Fields

In methods that reference `paused_remaining`, update to use the phase-specific field:

**In `update_remaining()`**:

```rust
pub(crate) fn update_remaining(&mut self) {
    if self.status == Status::Running {
        if let Some(start) = self.started_instant {
            let elapsed = start.elapsed().as_secs() as u32;

            // Use phase-specific paused time
            let initial = match self.phase {
                Phase::Work => self.paused_work_secs.unwrap_or(self.duration_secs),
                Phase::Break => self.paused_break_secs.unwrap_or(self.duration_secs),
            };

            if elapsed >= initial {
                self.remaining_secs = 0;
                self.handle_completion();
            } else {
                self.remaining_secs = initial - elapsed;
            }
        }
    }
}
```

### Step 1.3: Implement set_phase() Method

Add this new method to `TimerService`:

```rust
pub fn set_phase(&mut self, new_phase: Phase) {
    // Idempotent: no-op if already on requested phase
    if new_phase == self.phase {
        return;
    }

    // If currently running, pause and save remaining time
    if self.status == Status::Running {
        self.status = Status::Paused;
        // Store current remaining in exiting phase's field
        match self.phase {
            Phase::Work => self.paused_work_secs = Some(self.remaining_secs),
            Phase::Break => self.paused_break_secs = Some(self.remaining_secs),
        }
        self.started_instant = None;
    } else if self.status == Status::Paused {
        // Already paused; save current remaining time to exiting phase
        match self.phase {
            Phase::Work => self.paused_work_secs = Some(self.remaining_secs),
            Phase::Break => self.paused_break_secs = Some(self.remaining_secs),
        }
    }

    // Switch to new phase
    self.phase = new_phase;

    // Load paused time from new phase, or use standard duration
    match new_phase {
        Phase::Work => {
            self.duration_secs = WORK_DURATION_SECS;
            self.remaining_secs = self.paused_work_secs.unwrap_or(WORK_DURATION_SECS);
            self.status = Status::WorkReady;
            self.state_label = "Ready to work".to_string();
        }
        Phase::Break => {
            self.duration_secs = BREAK_DURATION_SECS;
            self.remaining_secs = self.paused_break_secs.unwrap_or(BREAK_DURATION_SECS);
            self.status = Status::BreakReady;
            self.state_label = "Ready to break".to_string();
        }
    }

    self.completion_flag = false;
}
```

### Step 1.4: Add Unit Tests

In `src-tauri/src/timer/tests.rs`, add tests:

```rust
#[test]
fn test_set_phase_idempotent() {
    let mut service = TimerService::new();
    let initial_remaining = service.remaining_secs;
    service.set_phase(Phase::Work);
    assert_eq!(service.remaining_secs, initial_remaining);
    assert_eq!(service.phase, Phase::Work);
}

#[test]
fn test_set_phase_preserves_paused_time() {
    let mut service = TimerService::new();
    service.status = Status::Paused;
    service.remaining_secs = 1200;

    service.set_phase(Phase::Break);
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.remaining_secs, BREAK_DURATION_SECS);
    assert_eq!(service.paused_work_secs, Some(1200));

    service.set_phase(Phase::Work);
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.remaining_secs, 1200); // Restored
}

#[test]
fn test_set_phase_pauses_running_timer() {
    let mut service = TimerService::new();
    service.status = Status::Running;
    service.started_instant = Some(std::time::Instant::now() - std::time::Duration::from_secs(400));
    service.duration_secs = WORK_DURATION_SECS;
    service.update_remaining();
    let paused_secs = service.remaining_secs;

    service.set_phase(Phase::Break);
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.status, Status::BreakReady);
    assert_eq!(service.paused_work_secs, Some(paused_secs));
}
```

---

## Part 2: Tauri Command Registration

### Step 2.1: Add set_phase Command

**File**: `src-tauri/src/lib.rs`

Add the new command:

```rust
#[tauri::command]
fn set_phase(phase: String, timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let phase_enum = match phase.to_lowercase().as_str() {
        "work" => Phase::Work,
        "break" => Phase::Break,
        _ => return Err("Invalid phase. Use 'work' or 'break'.".to_string()),
    };

    let mut service = timer.lock().map_err(|e| format!("Failed to lock timer: {}", e))?;
    service.set_phase(phase_enum);
    Ok(service.get_state())
}
```

### Step 2.2: Register Command in generate_handler

**File**: `src-tauri/src/lib.rs`

Update the `invoke_handler`:

```rust
.invoke_handler(tauri::generate_handler![
    get_state,
    start_timer,
    pause_timer,
    resume_timer,
    clear_timer,
    set_phase  // NEW
])
```

---

## Part 3: Frontend UI (TypeScript/HTML)

### Step 3.1: Add Mode Selector HTML

**File**: `src/index.html`

Add the mode selector before the timer display:

```html
<body>
  <div id="app">
    <!-- NEW: Mode selector buttons -->
    <div id="mode-selector" class="mode-selector">
      <button id="work-btn" class="mode-btn active">Work</button>
      <button id="break-btn" class="mode-btn">Break</button>
    </div>

    <!-- Existing timer display -->
    <div id="timer-display">25:00</div>
    <div id="state-label">Ready to work</div>

    <!-- Existing control buttons -->
    <div id="button-group">
      <!-- ... existing buttons ... -->
    </div>
  </div>
</body>
```

### Step 3.2: Add CSS for Mode Buttons

**File**: `src/index.html` or external stylesheet

Add styling:

```css
#mode-selector {
  display: flex;
  gap: 10px;
  justify-content: flex-start;
  margin-bottom: 20px;
}

.mode-btn {
  padding: 10px 20px;
  font-size: 14px;
  border: 2px solid transparent;
  border-radius: 4px;
  background-color: #f0f0f0;
  color: #333;
  cursor: pointer;
  transition: all 0.2s ease;
}

.mode-btn:hover {
  background-color: #e0e0e0;
}

.mode-btn.active {
  background-color: #cce5ff;  /* Light blue */
  border: 2px solid #0066cc;  /* Blue border */
  font-weight: bold;
}

.mode-btn:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}
```

### Step 3.3: Get References to Mode Buttons

**File**: `src/main.ts`

Add at the top with other DOM references:

```typescript
let workBtn: HTMLButtonElement;
let breakBtn: HTMLButtonElement;

// In the initialization/DOMContentLoaded block:
workBtn = document.getElementById('work-btn') as HTMLButtonElement;
breakBtn = document.getElementById('break-btn') as HTMLButtonElement;
```

### Step 3.4: Implement Mode Button Event Listeners

**File**: `src/main.ts`

Add in `attachEventListeners()`:

```typescript
workBtn.addEventListener('click', async () => {
  try {
    const state = await invoke<TimerState>('set_phase', { phase: 'work' });
    await updateUI();
    // Update active styling
    workBtn.classList.add('active');
    breakBtn.classList.remove('active');
  } catch (error) {
    console.error('Failed to set work phase:', error);
  }
});

breakBtn.addEventListener('click', async () => {
  try {
    const state = await invoke<TimerState>('set_phase', { phase: 'break' });
    await updateUI();
    // Update active styling
    breakBtn.classList.add('active');
    workBtn.classList.remove('active');
  } catch (error) {
    console.error('Failed to set break phase:', error);
  }
});
```

### Step 3.5: Update UI State Sync

**File**: `src/main.ts`

In `updateUI()`, add code to sync button state with backend phase:

```typescript
async function updateUI() {
  try {
    const state = await invoke<TimerState>('get_state');

    timerDisplay.textContent = formatTime(state.remainingSecs);
    stateLabel.textContent = state.stateLabel;

    // Sync mode button styling with backend state
    if (state.phase === 'work') {
      workBtn.classList.add('active');
      breakBtn.classList.remove('active');
    } else {
      breakBtn.classList.add('active');
      workBtn.classList.remove('active');
    }

    // Update button states
    startBtn.disabled = !(state.status === 'workReady' || state.status === 'breakReady');
    pauseBtn.disabled = state.status !== 'running';
    resumeBtn.disabled = state.status !== 'paused';
    clearBtn.disabled = state.status === 'workReady' || state.status === 'breakReady';

    // ... rest of existing updateUI logic ...
  } catch (error) {
    console.error('Failed to get state:', error);
  }
}
```

---

## Part 4: Testing

### Unit Tests

Run Rust unit tests:

```bash
cd src-tauri
cargo test timer::tests
```

Verify:

- ✓ Idempotent calls don't change state
- ✓ Paused time preserved across switches
- ✓ Running timer pauses on switch
- ✓ Status transitions correctly

### Integration Testing

Start the app in dev mode:

```bash
cargo tauri dev
```

Manual tests:

1. **Test 1: Simple Switch**
   - Click Work button (already active) → No change
   - Click Break button → Timer resets to 05:00, Break button emphasized
   - Click Work button → Timer restores previous paused time, Work button emphasized

2. **Test 2: Preserve Time**
   - Start work timer (click Start)
   - Wait 10 seconds (should show ~24:50)
   - Click Break button → Work paused at ~24:50
   - Break shows 05:00, Break button emphasized
   - Click Work button → Work shows ~24:50 again

3. **Test 3: Multiple Switches**
   - Rapid clicking between Work and Break
   - Each remaining time should preserve

4. **Test 4: Clear and Mode Switch**
   - Work paused at 20:00
   - Click Clear (confirm) → Reset to 25:00
   - Click Break → Break at 05:00

---

## Part 5: Cargo Build & Format

### Check Code Quality

```bash
cd src-tauri
cargo fmt
cargo clippy
cargo test
```

### Build Release

```bash
cd src-tauri
cargo tauri build
```

---

## Completion Checklist

- [ ] TimerService fields updated (paused_work_secs, paused_break_secs)
- [ ] set_phase() method implemented and tested
- [ ] All existing methods updated to use phase-specific paused fields
- [ ] set_phase command added to lib.rs
- [ ] set_phase command registered in invoke_handler
- [ ] Mode selector HTML added to index.html
- [ ] CSS styling added for .mode-btn and .active classes
- [ ] JavaScript references and event listeners added
- [ ] updateUI() syncs button state with backend
- [ ] Unit tests pass (cargo test)
- [ ] Integration tests pass (manual or automation)
- [ ] Code formatted and linted (cargo fmt && cargo clippy)
- [ ] Feature branch commits clean and documented

---

## Related Documentation

- [data-model.md](data-model.md) – Detailed state model
- [contracts/ipc-commands.md](contracts/ipc-commands.md) – IPC command specifications
- [spec.md](spec.md) – Original feature specification
