# IPC Contracts: Elapsed Time Display

**Date**: 2026-01-06  
**Feature**: [specs/007-show-elapsed-time/spec.md](specs/007-show-elapsed-time/spec.md)

---

## Shared Types

```typescript
interface TimerState {
  phase: 'work' | 'break';                     // Next/active phase
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  remainingSecs: number;                       // Countdown seconds (ignored when status==='complete')
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
  elapsedSecs?: number;                        // Elapsed since completion (present when status==='complete')
  elapsedRunning: boolean;                     // True when elapsed clock ticking
  lastCompletedPhase?: 'work' | 'break';       // Phase that just finished
}
```

---

## Command: get_state

Returns current timer state, including elapsed info when a session has completed.

**Signature**: `invoke<TimerState>('get_state')`

**Behavior**:
- If `status === 'complete'`, `elapsedSecs` is present and increments when `elapsedRunning` is true.
- `completionFlag` true only on the first state after completion (used for chime edge detection).

---

## Command: start_timer

Starts countdown for the current ready phase.

**Signature**: `invoke<TimerState>('start_timer')`

**Rules**:
- Allowed only when `status` is `workReady` or `breakReady`.
- When `status === 'complete'`, returns an error advising to clear first.

**Errors**:
- "Timer already running"
- "Timer is paused, use resume instead"
- "Timer completed; clear elapsed time before starting"

---

## Command: pause_timer

Pauses either the running countdown or the elapsed clock.

**Signature**: `invoke<TimerState>('pause_timer')`

**Rules**:
- If `status === 'running'`, pauses countdown and stores remaining seconds.
- If `status === 'complete'`, pauses elapsed clock and freezes `elapsedSecs`.
- Errors when timer is not running or elapsed-running.

---

## Command: resume_timer

Resumes a paused countdown or paused elapsed clock.

**Signature**: `invoke<TimerState>('resume_timer')`

**Rules**:
- If `status === 'paused'` from countdown, resumes countdown.
- If `status === 'complete'` with elapsed paused, resumes elapsed clock.
- Errors when nothing is paused.

---

## Command: clear_timer

Clears elapsed state and resets timer to the next phase ready state.

**Signature**: `invoke<TimerState>('clear_timer')`

**Rules**:
- Always allowed; when `status === 'complete'`, clears `elapsedSecs` and resets `phase` to the stored next phase (work→break, break→work) with fresh durations.
- Completion flag cleared.

---

## Usage Guidance (Frontend)

1. Poll `get_state` at 1 Hz.
2. When `status === 'complete'`:
   - Display `elapsedSecs` as `-MM:SS` in red (`#ef4444`).
   - Disable Start; enable Pause/Resume to control elapsed; enable Clear to reset.
3. When `status !== 'complete'`:
   - Display `remainingSecs` as `MM:SS`.
   - Enable/disable buttons per existing rules.
4. Use `completionFlag` edge to trigger chime once.

---

## Testing Scenarios

- Completion transition: verify `elapsedSecs` starts at 1s, `status='complete'`, `completionFlag=true` once.
- Pause elapsed: `elapsedSecs` stops increasing, `elapsedRunning=false`.
- Resume elapsed: `elapsedSecs` increases after resume, no jump backward.
- Clear from complete: state resets to ready for next phase, `elapsedSecs` absent, Start enabled.
- Start protection: calling `start_timer` while `status='complete'` returns error and leaves state unchanged.
