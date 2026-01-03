# Developer Quickstart: Manual Break Start

**Feature**: Manual Break Start  
**Branch**: `004-manual-break-start`  
**Target Audience**: Developers implementing this feature

## What This Feature Does

Changes timer behavior so work/break sessions don't auto-start. When a work session completes, a chime plays and the timer displays "5:00" for break time, but the countdown doesn't start until the user presses Start. Same for break completion returning to work.

**User Impact**: Gives users control over when they actually begin breaks and work sessions, instead of forcing immediate transitions.

## Prerequisites

- Rust 1.92+ installed (`rustup` recommended)
- Tauri CLI 2.9+ (`cargo install tauri-cli@2.9`)
- Node.js/npm (for frontend)
- Working knowledge of Rust and TypeScript

## Quick Setup

```bash
# Clone and checkout feature branch
git checkout 004-manual-break-start

# Install dependencies (if not already done)
cd src-tauri
cargo build

# Run in development mode
cargo tauri dev
```

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│ Frontend (TypeScript)                                   │
│ - src/main.ts                                           │
│ - Polls backend every 1 second                          │
│ - Renders timer display and button states              │
│ - Plays chime on completionFlag                         │
└─────────────────────┬───────────────────────────────────┘
                      │ Tauri IPC
                      │ (invoke commands)
┌─────────────────────▼───────────────────────────────────┐
│ Backend (Rust)                                          │
│ - src-tauri/src/timer.rs (timer logic)                 │
│ - src-tauri/src/lib.rs (IPC commands)                  │
│ - Manages state machine                                 │
│ - Calculates countdown                                  │
└─────────────────────────────────────────────────────────┘
```

## Key Files to Modify

### 1. `src-tauri/src/timer.rs` (Core Logic)

**What to change**:
- Expand `Status` enum to include `WorkReady` and `BreakReady`
- Modify `handle_completion()` to transition to Ready states (not auto-start)
- Update `start()` to be phase-aware (start work or break based on current state)
- Update state labels for clarity

**Current behavior** (line ~98-115):
```rust
fn handle_completion(&mut self) {
    match self.phase {
        Phase::Work => {
            // Currently auto-starts break - REMOVE THIS
            self.phase = Phase::Break;
            self.status = Status::Running;  // ← Change to BreakReady
            // ...
        }
        // ...
    }
}
```

**New behavior**:
```rust
fn handle_completion(&mut self) {
    self.completion_flag = true;
    match self.phase {
        Phase::Work => {
            self.phase = Phase::Break;
            self.status = Status::BreakReady;  // Don't auto-start
            self.remaining_secs = BREAK_DURATION_SECS;
            self.state_label = "Break ready - press Start".to_string();
            // Don't set started_instant
        }
        Phase::Break => {
            self.phase = Phase::Work;
            self.status = Status::WorkReady;
            self.remaining_secs = WORK_DURATION_SECS;
            self.state_label = "Work ready - press Start".to_string();
        }
    }
}
```

### 2. `src-tauri/src/lib.rs` (IPC Commands)

**What to verify**: Commands should handle new status values properly. The `start_timer` command may need updates if it currently assumes only work starts.

### 3. `src/main.ts` (Frontend)

**What to change**:
- Update `TimerState` interface with new status values
- Update button enable/disable logic for Ready states
- Simplify chime detection (remove work→break auto-start check)

**Current interface**:
```typescript
interface TimerState {
  status: 'idle' | 'running' | 'paused' | 'complete';
  // ...
}
```

**New interface**:
```typescript
interface TimerState {
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  // ...
}
```

**Button states**:
```typescript
// Current: Start disabled when running
startBtn.disabled = state.status === 'running';

// New: Start enabled when in Ready states
startBtn.disabled = !(state.status === 'workReady' || state.status === 'breakReady');
```

### 4. `src-tauri/src/timer/tests.rs` (Tests)

**What to add**:
- Test work completion → BreakReady transition
- Test start from BreakReady → Break/Running
- Test break completion → WorkReady transition
- Test completion_flag behavior
- Test clear from Ready states

## Implementation Checklist

- [ ] Update `Status` enum in timer.rs (add WorkReady, BreakReady)
- [ ] Modify `handle_completion()` to not auto-start
- [ ] Update `start()` to handle starting from BreakReady
- [ ] Update `TimerService::new()` to use WorkReady instead of Idle
- [ ] Update `clear()` to return to WorkReady
- [ ] Update state_label messages for clarity
- [ ] Update frontend TimerState interface
- [ ] Update frontend button logic
- [ ] Remove work→break auto-start chime detection in frontend
- [ ] Write/update unit tests for new behavior
- [ ] Test full cycle manually: work → break → work
- [ ] Test pause/resume during break
- [ ] Test clear from various states
- [ ] Update any affected integration tests

## Testing

### Run Unit Tests
```bash
cd src-tauri
cargo test
```

### Manual Test Scenarios

**Scenario 1: Work Completion**
1. Start app, press Start
2. Wait for work timer to reach 0:00
3. Verify: Chime plays, display shows 5:00, countdown does NOT start
4. Press Start
5. Verify: Break countdown begins

**Scenario 2: Break Completion**
1. Complete work session, start break
2. Wait for break to complete
3. Verify: Chime plays, display shows 25:00, countdown does NOT start
4. Press Start
5. Verify: Work countdown begins

**Scenario 3: Skip Break**
1. Complete work session (reach BreakReady state)
2. Press Clear instead of Start
3. Verify: Display shows 25:00, ready for work

**Scenario 4: Pause During Break**
1. Start break countdown
2. Press Pause
3. Verify: Countdown stops
4. Press Resume
5. Verify: Countdown continues from paused time

## State Machine Reference

See [data-model.md](data-model.md#state-machine) for complete state machine diagram.

**Key states**:
- `WorkReady`: Initial state, ready to start work (replaces old "Idle")
- `BreakReady`: After work completes, ready to start break (NEW)
- `Running`: Active countdown (work or break)
- `Paused`: Countdown paused (work or break)

**Key transitions**:
- Work/Running → `remaining=0` → BreakReady (not auto-start)
- Break/Running → `remaining=0` → WorkReady (not auto-start)
- Ready states → `start()` → Running

## Common Pitfalls

1. **Don't auto-start in handle_completion()**: The key change is setting status to Ready, not Running
2. **Update both phase and status together**: BreakReady must have phase=Break
3. **Clear completion_flag on actions**: Flag should clear when user starts next session
4. **Test serialization**: New status values must serialize correctly to frontend
5. **Update all state_label strings**: Users need clear indication of what to do

## API Contract

Full IPC command documentation: [contracts/ipc-commands.md](contracts/ipc-commands.md)

**Key commands**:
- `get_state()` - Get current timer state (polls every second)
- `start_timer()` - Start work or break based on current phase
- `pause_timer()` - Pause running countdown
- `resume_timer()` - Resume paused countdown
- `clear_timer()` - Reset to WorkReady state

## Dependencies

**No new dependencies needed**. All changes use existing:
- `serde` for serialization
- `std::time::Instant` for timing
- Tauri IPC (existing pattern)

## Performance Considerations

- No performance impact - same polling frequency (1 Hz)
- No additional memory usage
- Chime already implemented, just triggered at different times

## Rolling Back

If issues arise:
```bash
git checkout main
cargo tauri dev
```

All changes are in the feature branch, easy to isolate.

## Questions?

- Architecture decisions: See [research.md](research.md)
- Data structures: See [data-model.md](data-model.md)
- API details: See [contracts/ipc-commands.md](contracts/ipc-commands.md)
- Requirements: See [spec.md](spec.md)

## Next Steps

After implementing:
1. Run all tests: `cargo test`
2. Manual test all scenarios above
3. Verify no regressions in existing functionality (pause, resume, clear)
4. Consider adding visual indicator in UI for "ready" vs "running" states
5. Move to `/speckit.tasks` for detailed task breakdown if needed
