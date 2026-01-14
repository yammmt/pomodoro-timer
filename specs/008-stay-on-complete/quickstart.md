# Quickstart: Stay on Completed Session

**Date**: 2026-01-14  
**Feature**: [spec.md](spec.md)  
**Data Model**: [data-model.md](data-model.md)  
**Research**: [research.md](research.md)

---

## Overview

This guide provides step-by-step instructions to implement the stay-on-complete feature. The implementation modifies four methods in `timer.rs` to prevent automatic phase switching when sessions complete.

---

## Part 1: Modify handle_completion() Method

### Step 1.1: Locate Current Implementation

**File**: `src-tauri/src/timer.rs`  
**Current Lines**: 98-124

The current `handle_completion()` automatically switches phases. We need to replace this with simpler logic that stays in the current phase.

### Step 1.2: Replace handle_completion() Implementation

Replace the entire method with:

```rust
pub(crate) fn handle_completion(&mut self) {
    self.completion_flag = true;
    self.remaining_secs = 0;
    self.status = Status::Complete;
    self.started_instant = None;

    // Stay in current phase, update label
    self.state_label = match self.phase {
        Phase::Work => "Work completed".to_string(),
        Phase::Break => "Break completed".to_string(),
    };

    // Note: Do NOT clear paused_work_secs or paused_break_secs
    // Note: Do NOT change self.phase or self.duration_secs
}
```

**Key Changes**:

- ❌ **REMOVED**: All `self.phase = ...` assignments
- ❌ **REMOVED**: All `self.duration_secs = ...` assignments  
- ❌ **REMOVED**: All `paused_work_secs` and `paused_break_secs` clearing
- ✅ **ADDED**: `self.status = Status::Complete` (persistent completion state)
- ✅ **KEPT**: `self.completion_flag = true` (for UI indication)
- ✅ **ADDED**: Phase-specific state labels

---

## Part 2: Modify start() Method

### Step 2.1: Locate Status::Complete Handling

**File**: `src-tauri/src/timer.rs`  
**Current Lines**: 127-142 (inside the `start()` method)

Current code handles `Status::Complete` by always starting a work session. We need to make this phase-aware.

### Step 2.2: Update Status::Complete Branch

Find this section in the `start()` method:

```rust
Status::WorkReady | Status::Complete => {
    // Start work session
    self.phase = Phase::Work;
    self.status = Status::Running;
    self.duration_secs = WORK_DURATION_SECS;
    self.remaining_secs = WORK_DURATION_SECS;
    self.completion_flag = false;
    self.state_label = "Working".to_string();
    self.started_instant = Some(Instant::now());
    self.paused_work_secs = None;
    // Preserve paused_break_secs for switching back to break later
}
```

Replace with:

```rust
Status::WorkReady => {
    // Start work session
    self.phase = Phase::Work;
    self.status = Status::Running;
    self.duration_secs = WORK_DURATION_SECS;
    self.remaining_secs = WORK_DURATION_SECS;
    self.completion_flag = false;
    self.state_label = "Working".to_string();
    self.started_instant = Some(Instant::now());
    self.paused_work_secs = None;
    // Preserve paused_break_secs for switching back to break later
}
Status::Complete => {
    // Restart the current phase (stay in work or break)
    match self.phase {
        Phase::Work => {
            self.status = Status::Running;
            self.duration_secs = WORK_DURATION_SECS;
            self.remaining_secs = WORK_DURATION_SECS;
            self.completion_flag = false;
            self.state_label = "Working".to_string();
            self.started_instant = Some(Instant::now());
            self.paused_work_secs = None;
        }
        Phase::Break => {
            self.status = Status::Running;
            self.duration_secs = BREAK_DURATION_SECS;
            self.remaining_secs = BREAK_DURATION_SECS;
            self.completion_flag = false;
            self.state_label = "Break time".to_string();
            self.started_instant = Some(Instant::now());
            self.paused_break_secs = None;
        }
    }
}
```

**Key Changes**:

- ✅ **SPLIT**: `WorkReady | Complete` pattern into separate branches
- ✅ **ADDED**: Phase check in `Status::Complete` branch
- ✅ **BEHAVIOR**: Start restarts the *current* phase, not always work

---

## Part 3: Modify clear() Method

### Step 3.1: Locate Current Implementation

**File**: `src-tauri/src/timer.rs`  
**Current Lines**: 218-230

Current code always resets to work phase. We need to preserve the current phase.

### Step 3.2: Replace clear() Implementation

Replace the entire method with:

```rust
pub fn clear(&mut self) -> Result<TimerState, String> {
    // Preserve current phase, reset to ready state
    match self.phase {
        Phase::Work => {
            self.status = Status::WorkReady;
            self.remaining_secs = WORK_DURATION_SECS;
            self.duration_secs = WORK_DURATION_SECS;
            self.state_label = "Ready to work".to_string();
        }
        Phase::Break => {
            self.status = Status::BreakReady;
            self.remaining_secs = BREAK_DURATION_SECS;
            self.duration_secs = BREAK_DURATION_SECS;
            self.state_label = "Ready to break".to_string();
        }
    }

    self.completion_flag = false;
    self.started_instant = None;
    self.paused_work_secs = None;
    self.paused_break_secs = None;

    Ok(self.get_state())
}
```

**Key Changes**:

- ❌ **REMOVED**: Hardcoded `self.phase = Phase::Work`
- ✅ **ADDED**: Phase matching to set appropriate ready status
- ✅ **BEHAVIOR**: Clear resets *current* phase, not always work

---

## Part 4: Modify set_phase() Method

### Step 4.1: Locate Status Handling Section

**File**: `src-tauri/src/timer.rs`  
**Current Lines**: 238-260 (inside `set_phase()` method)

Current code handles `Running` and `Paused` statuses but not `Complete`.

### Step 4.2: Add Status::Complete Handling

Find this section in `set_phase()`:

```rust
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
```

Add a new branch before switching phase (after the existing if/else if block):

```rust
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
} else if self.status == Status::Complete {
    // Session completed; switching phase clears completion state
    // No need to save remaining time (already 0)
    // Completion flag will be cleared below
}
```

Then ensure `self.completion_flag = false;` is executed for all phase switches (it already is on line 277).

**Key Changes**:

- ✅ **ADDED**: Explicit `Status::Complete` handling
- ✅ **BEHAVIOR**: Phase switching from Complete state works seamlessly
- ✅ **DOCUMENTATION**: Comment explains Complete state transition

---

## Part 5: Add Unit Tests

### Step 5.1: Open Test File

**File**: `src-tauri/src/timer/tests.rs`

Add the following tests at the end of the file:

### Step 5.2: Test Work Completion Stays in Work Mode

```rust
#[test]
fn test_work_completion_stays_in_work_mode() {
    let mut service = TimerService::new();

    // Start work session
    service.start().unwrap();

    // Simulate time passing to completion
    service.remaining_secs = 0;
    service.handle_completion();

    // Should stay in work phase with Complete status
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.status, Status::Complete);
    assert_eq!(service.remaining_secs, 0);
    assert!(service.completion_flag);
}
```

### Step 5.3: Test Break Completion Stays in Break Mode

```rust
#[test]
fn test_break_completion_stays_in_break_mode() {
    let mut service = TimerService::new();

    // Switch to break and start
    service.set_phase(Phase::Break);
    service.start().unwrap();

    // Simulate time passing to completion
    service.remaining_secs = 0;
    service.handle_completion();

    // Should stay in break phase with Complete status
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.status, Status::Complete);
    assert_eq!(service.remaining_secs, 0);
    assert!(service.completion_flag);
}
```

### Step 5.4: Test Start After Work Completion

```rust
#[test]
fn test_start_after_work_completion_restarts_work() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    service.remaining_secs = 0;
    service.handle_completion();
    assert_eq!(service.status, Status::Complete);

    // Start again - should restart work, not switch to break
    service.start().unwrap();
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.status, Status::Running);
    assert_eq!(service.remaining_secs, 1500); // 25 minutes
}
```

### Step 5.5: Test Start After Break Completion

```rust
#[test]
fn test_start_after_break_completion_restarts_break() {
    let mut service = TimerService::new();

    // Switch to break and complete
    service.set_phase(Phase::Break);
    service.start().unwrap();
    service.remaining_secs = 0;
    service.handle_completion();
    assert_eq!(service.status, Status::Complete);

    // Start again - should restart break, not switch to work
    service.start().unwrap();
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.status, Status::Running);
    assert_eq!(service.remaining_secs, 300); // 5 minutes
}
```

### Step 5.6: Test Clear Preserves Phase

```rust
#[test]
fn test_clear_preserves_current_phase() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    service.remaining_secs = 0;
    service.handle_completion();

    // Clear should reset to work ready (not switch to break)
    service.clear().unwrap();
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.status, Status::WorkReady);
    assert_eq!(service.remaining_secs, 1500);

    // Same for break
    service.set_phase(Phase::Break);
    service.start().unwrap();
    service.remaining_secs = 0;
    service.handle_completion();

    service.clear().unwrap();
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.status, Status::BreakReady);
    assert_eq!(service.remaining_secs, 300);
}
```

### Step 5.7: Test Phase Switch From Complete

```rust
#[test]
fn test_phase_switch_from_complete_status() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    service.remaining_secs = 0;
    service.handle_completion();
    assert!(service.completion_flag);

    // Switch to break - should clear completion flag
    service.set_phase(Phase::Break);
    assert_eq!(service.phase, Phase::Break);
    assert_eq!(service.status, Status::BreakReady);
    assert!(!service.completion_flag); // Completion cleared
    assert_eq!(service.remaining_secs, 300);
}
```

### Step 5.8: Test Same-Phase Switch Resets Timer

```rust
#[test]
fn test_same_phase_switch_after_complete_resets() {
    let mut service = TimerService::new();

    // Complete work session
    service.start().unwrap();
    service.remaining_secs = 0;
    service.handle_completion();

    // Switch to same phase (work) should reset
    service.set_phase(Phase::Work);
    assert_eq!(service.phase, Phase::Work);
    assert_eq!(service.status, Status::WorkReady);
    assert_eq!(service.remaining_secs, 1500);
    assert!(!service.completion_flag);
}
```

---

## Part 6: Verification

### Step 6.1: Run Tests

```bash
cd src-tauri
cargo test
```

All tests should pass, including the new ones.

### Step 6.2: Run the Application

```bash
cargo tauri dev
```

### Step 6.3: Manual Testing Checklist

1. **Work Completion Stays in Work**:
   - [ ] Start a work session
   - [ ] Wait for completion (or reduce duration for testing)
   - [ ] Verify timer shows 00:00 in work mode
   - [ ] Verify Work button remains emphasized
   - [ ] Verify completion indication appears

2. **Break Completion Stays in Break**:
   - [ ] Click Break button
   - [ ] Start a break session
   - [ ] Wait for completion
   - [ ] Verify timer shows 00:00 in break mode
   - [ ] Verify Break button remains emphasized

3. **Start After Completion**:
   - [ ] Complete a work session (00:00)
   - [ ] Click Start
   - [ ] Verify timer starts a new work session (25:00)
   - [ ] Complete a break session (00:00)
   - [ ] Click Start
   - [ ] Verify timer starts a new break session (5:00)

4. **Clear After Completion**:
   - [ ] Complete a work session
   - [ ] Click Clear
   - [ ] Verify timer resets to 25:00 in work mode
   - [ ] Complete a break session
   - [ ] Click Clear
   - [ ] Verify timer resets to 5:00 in break mode

5. **Manual Phase Switching After Completion**:
   - [ ] Complete a work session
   - [ ] Click Break button
   - [ ] Verify timer switches to break mode at 5:00
   - [ ] Complete a break session
   - [ ] Click Work button
   - [ ] Verify timer switches to work mode at 25:00

---

## Summary

This implementation requires changes to **4 methods** in `timer.rs`:

1. ✅ `handle_completion()` - Stay in current phase instead of auto-switching
2. ✅ `start()` - Phase-aware restart from Complete status
3. ✅ `clear()` - Preserve current phase when resetting
4. ✅ `set_phase()` - Handle Complete status transitions

And adds **7 unit tests** in `timer/tests.rs`:

1. ✅ Work completion stays in work mode
2. ✅ Break completion stays in break mode
3. ✅ Start after work completion restarts work
4. ✅ Start after break completion restarts break
5. ✅ Clear preserves current phase
6. ✅ Phase switch from Complete status works
7. ✅ Same-phase switch after complete resets timer

**No frontend changes required** - the existing UI already supports manual session control through the Work/Break buttons from feature 006.
