# Quickstart: Show Elapsed Time After Session Completion

**Feature**: 009-show-elapsed-time  
**Date**: 2026-01-31  
**For**: Developers implementing this feature

## Overview

Add overtime display to show elapsed time after a session completes. When work or break timer reaches 0:00, continue counting elapsed time in red with minus prefix (e.g., "-02:13"), capped at 59:59.

---

## Prerequisites

- Existing Pomodoro Timer app (Rust 1.92 + Tauri 2.9)
- Familiarity with timer state machine in `src-tauri/src/timer.rs`
- Basic understanding of Tauri IPC commands

---

## Implementation Steps

### Step 1: Backend - Add Overtime Tracking Field

**File**: `src-tauri/src/timer.rs`

Add `completed_at` field to `TimerService`:

```rust
pub struct TimerService {
    // ... existing fields ...
    pub(crate) started_instant: Option<Instant>,
    pub(crate) completed_at: Option<Instant>,  // ⭐ NEW
    // ... rest of fields ...
}
```

Update constructor:

```rust
impl TimerService {
    pub fn new() -> Self {
        Self {
            // ... existing fields ...
            started_instant: None,
            completed_at: None,  // ⭐ NEW
            // ... rest of fields ...
        }
    }
}
```

---

### Step 2: Backend - Capture Completion Timestamp

**File**: `src-tauri/src/timer.rs`

Modify `handle_completion()`:

```rust
pub(crate) fn handle_completion(&mut self) {
    self.completion_flag = true;
    self.remaining_secs = 0;
    self.status = Status::Complete;
    self.started_instant = None;
    self.completed_at = Some(Instant::now());  // ⭐ NEW

    // ... rest of method ...
}
```

---

### Step 3: Backend - Clear Overtime on State Transitions

**File**: `src-tauri/src/timer.rs`

Update `start()`, `resume()`, `clear()`, and `set_phase()`:

```rust
pub fn start(&mut self) -> Result<TimerState, String> {
    // ... existing logic ...
    self.completed_at = None;  // ⭐ NEW
    // ... rest of method ...
}

pub fn resume(&mut self) -> Result<TimerState, String> {
    // ... existing logic ...
    self.completed_at = None;  // ⭐ NEW
    // ... rest of method ...
}

pub fn clear(&mut self) -> Result<TimerState, String> {
    // ... existing logic ...
    self.completed_at = None;  // ⭐ NEW
    // ... rest of method ...
}

pub fn set_phase(&mut self, new_phase: Phase) {
    // ... existing logic ...
    self.completed_at = None;  // ⭐ NEW (add early in method)
    // ... rest of method ...
}
```

---

### Step 4: Backend - Add Overtime Field to TimerState

**File**: `src-tauri/src/timer.rs`

Extend `TimerState` struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerState {
    // ... existing fields ...
    pub state_label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_secs: Option<u32>,  // ⭐ NEW
}
```

---

### Step 5: Backend - Calculate Overtime in get_state()

**File**: `src-tauri/src/timer.rs`

Modify `get_state()`:

```rust
pub fn get_state(&mut self) -> TimerState {
    self.update_remaining();

    // ⭐ NEW: Calculate overtime
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
        overtime_secs,  // ⭐ NEW
    }
}
```

---

### Step 6: Frontend - Update TimerState Interface

**File**: `src/main.ts`

Extend `TimerState` interface:

```typescript
interface TimerState {
  phase: 'work' | 'break';
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  remainingSecs: number;
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
  overtimeSecs?: number;  // ⭐ NEW
}
```

---

### Step 7: Frontend - Add Overtime Display Logic

**File**: `src/main.ts`

Modify `updateUI()`:

```typescript
async function updateUI() {
  try {
    const state = await invoke<TimerState>('get_state');

    // ⭐ NEW: Handle overtime display
    if (state.overtimeSecs !== undefined) {
      timerDisplay.textContent = `-${formatTime(state.overtimeSecs)}`;
      timerDisplay.classList.add('overtime');
    } else {
      timerDisplay.textContent = formatTime(state.remainingSecs);
      timerDisplay.classList.remove('overtime');
    }

    stateLabel.textContent = state.stateLabel;

    // ... rest of existing UI update logic ...
  } catch (error) {
    console.error('Failed to get state:', error);
  }
}
```

---

### Step 8: Frontend - Add CSS Styling

**File**: `src/index.html` (or separate CSS file)

Add overtime style:

```css
.overtime {
  color: #dc2626; /* Red color (Tailwind red-600) */
}
```

---

### Step 9: Testing - Add Unit Tests

**File**: `src-tauri/src/timer/tests.rs`

Add overtime test cases:

```rust
#[test]
fn test_overtime_displayed_after_completion() {
    let mut timer = TimerService::new();
    timer.remaining_secs = 0;
    timer.handle_completion();

    std::thread::sleep(std::time::Duration::from_secs(2));

    let state = timer.get_state();
    assert_eq!(state.status, Status::Complete);
    assert!(state.overtime_secs.is_some());
    assert!(state.overtime_secs.unwrap() >= 2);
}

#[test]
fn test_overtime_capped_at_59_59() {
    let mut timer = TimerService::new();
    timer.status = Status::Complete;
    timer.completed_at = Some(Instant::now() - std::time::Duration::from_secs(3700));

    let state = timer.get_state();
    assert_eq!(state.overtime_secs, Some(3599));
}

#[test]
fn test_overtime_cleared_on_start() {
    let mut timer = TimerService::new();
    timer.status = Status::Complete;
    timer.completed_at = Some(Instant::now());

    timer.start().unwrap();

    assert!(timer.completed_at.is_none());
    let state = timer.get_state();
    assert!(state.overtime_secs.is_none());
}
```

---

### Step 10: Manual Testing

1. **Start work session**: Click Start, wait for 25:00 countdown
2. **Let it complete**: Timer reaches 0:00, plays chime
3. **Observe overtime**: Display should show "-00:01", "-00:02", etc. in red
4. **Check cap**: Let it run to "-59:59", verify it stops incrementing
5. **Resume from overtime**: Click Start, verify display returns to "25:00" in normal color
6. **Test break phase**: Repeat for break session (5:00)

---

## Verification Checklist

- [ ] Backend compiles without errors (`cargo build`)
- [ ] Unit tests pass (`cargo test`)
- [ ] Frontend TypeScript compiles (`npm run check` or equivalent)
- [ ] Overtime displays in red with minus prefix
- [ ] Overtime updates every second
- [ ] Overtime caps at -59:59
- [ ] Resuming from overtime clears display
- [ ] Both work and break sessions support overtime
- [ ] Clear button works during overtime

---

## Common Issues

### Issue: Overtime not showing

**Check**:

- `completed_at` set in `handle_completion()`?
- `overtime_secs` calculated in `get_state()`?
- Frontend checking `state.overtimeSecs !== undefined`?

### Issue: Overtime not clearing

**Check**:

- `completed_at = None` in `start()`, `resume()`, `clear()`, `set_phase()`?

### Issue: Red color not applying

**Check**:

- `.overtime` CSS class defined?
- Class added in `updateUI()` when `overtimeSecs` present?
- Class removed when `overtimeSecs` absent?

---

## Resources

- Feature spec: [spec.md](../spec.md)
- Data model: [data-model.md](../data-model.md)
- API contract: [contracts/timer-state.md](../contracts/timer-state.md)
- Research notes: [research.md](../research.md)

---

## Next Steps

After implementation:

1. Run `/speckit.tasks` to break down work into trackable tasks
2. Implement changes following tasks checklist
3. Run tests and manual verification
4. Submit PR for code review
