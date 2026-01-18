# Research: Stay on Completed Session

**Date**: 2026-01-14  
**Feature**: [spec.md](spec.md)

## Overview

Phase 0 research to resolve technical unknowns and validate design approach before Phase 1 detailed design. This feature removes automatic session transitions at completion, allowing users to manually control when they switch between work and break modes.

---

## Decision 1: Modify Completion Handler Behavior

**Question**: How should the `handle_completion()` method be modified to prevent automatic phase switching while maintaining completion indication?

**Research Findings**:

- Current `handle_completion()` in `timer.rs` automatically switches phases:
  - Work completion → sets phase to Break, status to BreakReady
  - Break completion → sets phase to Work, status to WorkReady
- This auto-transition behavior needs to be removed while preserving:
  - `completion_flag = true` (for completion indication)
  - Remaining time set to 0
  - Status update to indicate completion
- The existing Work/Break toggle buttons (feature 006) provide manual session switching via `set_phase()` method

**Decision**: Modify `handle_completion()` to:

1. Set `completion_flag = true` (preserved for UI indication)
2. Set `remaining_secs = 0` (timer shows 00:00)
3. Set `status = Status::Complete` (indicates completion state)
4. **Remove** phase switching logic (stay in current phase)
5. Update `state_label` to indicate completion in current phase
6. Clear `started_instant` (timer no longer running)
7. **Do NOT** clear `paused_work_secs` or `paused_break_secs` (preserve for future use)

**Note on paused time preservation**: When a session naturally completes (reaches 00:00), we preserve both phases' paused times because the user might switch to the other phase and want to resume where they left off. For example, if the user paused a break session earlier, completed a work session, then switches back to break—they should see their previously paused break time. This is different from `clear()`, which is an explicit "reset everything" action.

**Rationale**: Minimal change to existing code. Leverages existing `Status::Complete` enum value. Work/Break buttons already handle manual phase switching. No new state fields needed.

**Alternatives Considered**:

- Add a new "auto_transition" flag: Rejected—adds state complexity for a simpler behavior (staying put).
- Create a new status like `CompletedWork`/`CompletedBreak`: Rejected—existing `Status::Complete` + `phase` field already provide this information.
- Modify `set_phase()` to handle completion state: Rejected—separation of concerns; completion handling should stay in completion method.

---

## Decision 2: Status Enum Usage

**Question**: Should we use existing `Status::Complete` or modify the status transitions after completion?

**Research Findings**:

- Current `Status` enum has: `WorkReady`, `BreakReady`, `Running`, `Paused`, `Complete`
- `Status::Complete` exists but is currently only set transiently before auto-switching to the opposite phase's Ready state

- After auto-switch, the status becomes `BreakReady` (after work) or `WorkReady` (after break)
- Frontend currently checks `completion_flag` for showing completion indication

**Decision**: Use `Status::Complete` as the persistent status after a session finishes. The timer will show:

- Phase: Work or Break (stays in current phase)
- Status: Complete
- remaining_secs: 0

This provides a clear state for "session completed, waiting for user action."

**Rationale**: Semantic clarity—`Complete` accurately describes the state. Frontend can check `status === 'Complete'` in addition to `completion_flag`. No enum changes needed.

**Alternatives Considered**:

- Set status to `Paused`: Rejected—misleading; completion is not the same as user-initiated pause.
- Set status back to `WorkReady`/`BreakReady`: Rejected—would lose the completion state; Start button behavior would be ambiguous.

---

## Decision 3: Start Button Behavior After Completion

**Question**: What should happen when the user clicks Start after a session completes at 00:00?

**Research Findings**:

- Current `start()` method checks status and starts based on `WorkReady`, `BreakReady`, or `Complete`
- `Complete` status currently leads to starting a work session
- After completion with our changes, we'll be in `Status::Complete` with `phase` set to the completed phase

**Decision**: Modify `start()` to check both `Status::Complete` and current `phase`:

- If `status == Status::Complete && phase == Phase::Work`: Start a fresh work session (25:00)

- If `status == Status::Complete && phase == Phase::Break`: Start a fresh break session (5:00)

This allows users to restart the same session type without needing to click the Work/Break button first.

**Rationale**: Intuitive UX—pressing Start after work completion starts another work session. Users can still switch to break manually via the Break button before pressing Start.

**Alternatives Considered**:

- Always start work session on Start after Complete: Rejected—not intuitive if user completed a break and wants another break.

- Require clicking Work/Break button before Start: Rejected—adds unnecessary friction; Start should work immediately.

---

## Decision 4: Clear Button Behavior After Completion

**Question**: What should Clear button do when the timer is at 00:00 in Complete status?

**Research Findings**:

- Current `clear()` method resets to work mode with `WorkReady` status
- Always resets to 25:00 work session regardless of current phase

- Spec states: "Clicking Clear while at 00:00 should reset to the standard duration of the current session type without changing the active session mode"

**Decision**: Modify `clear()` to preserve the current phase when resetting:

- If `phase == Phase::Work`: Reset to `WorkReady` with 25:00

- If `phase == Phase::Break`: Reset to `BreakReady` with 5:00
- Clear `completion_flag` and set appropriate ready status
- Clear all paused time fields for both phases

**Note on clearing paused times**: Unlike `handle_completion()` which preserves paused times, `clear()` is an explicit user action that means "I want to start completely fresh". Therefore, we clear both `paused_work_secs` and `paused_break_secs` to ensure no leftover state from previous sessions. This gives users a clean slate.

**Rationale**: Aligns with spec requirement. Maintains consistency with "stay in current session" philosophy. Users can reset the current session without losing their mode selection.

**Alternatives Considered**:

- Keep existing behavior (always reset to work): Rejected—violates spec requirement and breaks UX consistency.
- Only reset the current phase's time: Rejected—Clear should fully reset the timer state, not just current phase.

---

## Decision 5: Work/Break Button Interaction After Completion

**Question**: Do the existing Work/Break buttons need modification to handle `Status::Complete`?

**Research Findings**:

- Current `set_phase()` method handles phase switching
- Checks if status is `Running` (pauses it) or `Paused` (preserves time)

- Switches to the new phase with appropriate status (`WorkReady` or `BreakReady`)
- Already resets `completion_flag = false`

**Decision**: Add handling for `Status::Complete` in `set_phase()`:

- If `status == Status::Complete`, treat it like a ready state transition
- Switch to new phase's ready status (`WorkReady` or `BreakReady`)
- Set duration to new phase's standard duration
- Clear completion flag
- If switching to same phase, reset to standard duration (like a reset)

**Rationale**: Makes phase switching work seamlessly from completion state. Existing logic already handles most cases; just needs Complete status branch.

**Alternatives Considered**:

- Require Clear before phase switching: Rejected—adds unnecessary step; phase switching should work from any state.
- Auto-start new phase after switching: Rejected—spec requires user to press Start explicitly.

---

## Decision 6: Completion Indication Preservation

**Question**: How do we ensure completion indications (visual/audible) still trigger without auto-switching?

**Research Findings**:

- `completion_flag` is set to `true` in `handle_completion()`

- Frontend polls `get_state()` and checks `completion_flag` to trigger indication

- Current implementation works independently of phase switching logic

**Decision**: No changes needed for completion indication. The `completion_flag` will still be set to `true` in the modified `handle_completion()` method, maintaining all existing indication behavior.

**Rationale**: Completion indication is already decoupled from phase transition logic. Simply setting the flag is sufficient.

**Alternatives Considered**:

- Add a new completion callback system: Rejected—over-engineering; existing flag-based approach works.
- Move completion indication to frontend: Rejected—backend is already doing this correctly.

---

## Summary of Implementation Changes

**Files to Modify**:

1. `src-tauri/src/timer.rs`:

   - Modify `handle_completion()` method: Stay in current phase, set `Status::Complete`
   - Modify `start()` method: Handle `Status::Complete` with phase-aware restart
   - Modify `clear()` method: Preserve current phase when resetting
   - Modify `set_phase()` method: Handle `Status::Complete` state transitions

2. `src-tauri/src/timer/tests.rs`:
   - Add test: Work completion stays in work mode
   - Add test: Break completion stays in break mode
   - Add test: Start button restarts same session after completion
   - Add test: Clear button preserves phase after completion
   - Add test: Phase switching works from Complete status

**No Changes Needed**:

- Frontend (`src/index.html`, `src/main.ts`): Existing Work/Break buttons handle manual switching
- IPC commands (`src-tauri/src/lib.rs`): No new commands required
- Status/Phase enums: Existing values cover all needed states

**Core Principle Alignment**:

- **Simplicity**: Reduces complexity by removing auto-transition logic
- **Code Quality**: Minimal, localized changes to existing methods
- **Testing**: Clear test cases for new behavior
- **UX Consistency**: Users get explicit control, matching desktop app conventions
