# Research: Manual Break Start

**Feature**: Manual Break Start  
**Date**: 2026-01-03  
**Purpose**: Document technical decisions and research findings for implementing manual break transitions

## Current Architecture Analysis

### Timer State Machine (Current)

**File**: `src-tauri/src/timer.rs`

**Current States**:
- `Status`: Idle, Running, Paused, Complete
- `Phase`: Work, Break

**Current Behavior**:
- Work completion automatically transitions to Break with Running status
- Break completion sets status to Complete
- Start command always resets to Work phase

**Key Finding**: The current architecture auto-starts breaks in `handle_completion()` method at line ~98-115. This is the core behavior that needs to change.

### Frontend Chime Implementation

**File**: `src/main.ts`

**Current Implementation**:
- Chime already implemented using Web Audio API (`playCompletionChime()`)
- Currently plays on `completionFlag` transitions
- Duration: 3 seconds, 880Hz (A5) sine wave

**Key Finding**: Chime functionality exists and works. Need to ensure it plays at the right state transitions (work→break_ready, break→work_ready).

## Design Decisions

### Decision 1: State Representation

**Problem**: How to distinguish "ready to start" from "actively running"?

**Options Considered**:
1. Add new Status values: `WorkReady`, `BreakReady`
2. Keep Status enum, use Phase + Status combinations
3. Add separate boolean flag `auto_started`

**Decision**: Add new Status values (`WorkReady`, `BreakReady`) while keeping `Running` and `Paused` for active countdowns.

**Rationale**:
- Clear semantic meaning in code
- Follows existing Status enum pattern
- Makes state machine logic explicit and testable
- Frontend can easily distinguish ready vs running states
- No ambiguous combinations to handle

**Alternatives Rejected**:
- Option 2: Combinations like `(Phase::Break, Status::Idle)` are confusing - what does "idle break" mean?
- Option 3: Boolean flags add hidden state complexity and are error-prone

### Decision 2: State Machine Transitions

**Current Transitions**:
```
Idle --[start]--> Work/Running --[complete]--> Break/Running --[complete]--> Complete
```

**New Transitions**:
```
Idle/WorkReady --[start]--> Work/Running --[complete]--> BreakReady
BreakReady --[start]--> Break/Running --[complete]--> WorkReady
WorkReady --[start]--> Work/Running
```

**Key Changes**:
- Completions transition to "Ready" states, not auto-start
- Start command behavior depends on current phase/status
- Clear always returns to WorkReady (existing behavior mostly preserved)

### Decision 3: Completion Flag Behavior

**Problem**: When should `completion_flag` be set?

**Decision**: Set `completion_flag` when transitioning to any "Ready" state (WorkReady or BreakReady) from a completed countdown.

**Rationale**:
- Frontend uses this to trigger chime
- Represents "session just completed" event
- Clear on next action (start/clear) to reset for next cycle

### Decision 4: Start Command Logic

**Problem**: Start command currently always begins Work phase. How to support starting Break?

**Decision**: Make start command phase-aware:
- If status is `WorkReady` → start Work/Running
- If status is `BreakReady` → start Break/Running
- If status is `Running` → return error (already running)
- If status is `Complete` → treat as WorkReady

**Rationale**:
- Single command interface (no separate start_work/start_break)
- Intuitive: "Start" means "start the countdown for whatever phase you're in"
- Backwards compatible for initial work start
- Frontend doesn't need phase-specific commands

## Technical Implementation Notes

### Backend Changes (Rust)

**File**: `src-tauri/src/timer.rs`

1. **Expand Status enum**:
   ```rust
   pub enum Status {
       WorkReady,
       BreakReady,
       Running,
       Paused,
       Complete,  // May be deprecated if Ready states replace it
   }
   ```

2. **Modify `handle_completion()`**:
   - Work completion → transition to `BreakReady`, not auto-start
   - Break completion → transition to `WorkReady`
   - Set `completion_flag` in both cases
   - Update `state_label` appropriately

3. **Modify `start()` command**:
   - Check current status
   - If `WorkReady` or initial → start work countdown
   - If `BreakReady` → start break countdown
   - Set status to `Running`, clear `completion_flag`

4. **Update `clear()` command**:
   - Reset to `WorkReady` status (instead of `Idle`)
   - Works from any state

### Frontend Changes (TypeScript)

**File**: `src/main.ts`

1. **Update TimerState interface**:
   ```typescript
   status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
   ```

2. **Update chime detection**:
   - Trigger on `completionFlag` transitions (already works)
   - Remove work→break auto-start detection (line ~87-89)

3. **Update button states**:
   - Start enabled: status is `workReady` or `breakReady`
   - Pause enabled: status is `running`
   - Resume enabled: status is `paused`
   - Clear always enabled

### Testing Strategy

**Unit Tests** (`src-tauri/src/timer/tests.rs`):
1. Work completion transitions to BreakReady (not auto-start)
2. Start from BreakReady begins break countdown
3. Break completion transitions to WorkReady
4. Start from WorkReady begins work countdown
5. Clear from BreakReady returns to WorkReady
6. Clear from Running break returns to WorkReady
7. Completion flag set/cleared appropriately

**Integration Tests**:
1. Full cycle: start work → complete → manual start break → complete → manual start work
2. Clear during various states
3. Pause/resume during break countdown

## Dependencies

**No new dependencies required**. All changes use existing:
- Rust standard library (std::time, serde)
- Tauri IPC (existing commands pattern)
- Web Audio API (already in use)

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Breaking existing tests | High | Update tests incrementally, ensure all pass |
| UI confusion (users expect auto-start) | Medium | Clear state labels ("Press Start for break") |
| Serialization issues with new Status values | Medium | Test serde serialization thoroughly |
| Chime not playing at right times | Low | Verify completion_flag logic in tests |

## Open Questions

None - all design decisions resolved through architecture analysis.

## References

- Existing implementation: `src-tauri/src/timer.rs` (lines 1-193)
- Frontend integration: `src/main.ts` (lines 1-163)
- Feature spec: [spec.md](spec.md)
- Constitution: `/.specify/memory/constitution.md`
