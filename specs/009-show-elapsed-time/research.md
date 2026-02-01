# Research: Show Elapsed Time After Session Completion

**Feature**: 009-show-elapsed-time  
**Date**: 2026-01-31  
**Phase**: 0 - Research and Discovery

## Research Questions

### Q1: How to track overtime after session completion?

**Decision**: Store completion timestamp when timer reaches 0:00, then calculate elapsed overtime on each update.

**Rationale**:

- Leverages existing `Instant` usage in `started_instant`
- Minimal state addition (single `Option<Instant>` field for `completed_at`)
- Calculation happens in existing `update_remaining()` method
- Natural integration with current polling pattern (1Hz frontend updates)

**Alternatives Considered**:

- **Negative remaining_secs**: Rejected because `remaining_secs` is `u32` (unsigned), would require type change and affect all existing code
- **Separate overtime counter**: Rejected as redundant; elapsed time calculation from timestamp is cleaner
- **Frontend-only tracking**: Rejected because backend owns timer state truth; frontend should only display

**Implementation Notes**:

- Add `completed_at: Option<Instant>` to `TimerService`
- Set `completed_at = Some(Instant::now())` in `handle_completion()`
- Clear `completed_at = None` on `start()`, `resume()`, `clear()`, or phase switch
- In `update_remaining()`, if `status == Complete && completed_at.is_some()`, calculate elapsed and cap at 3599 (59:59)

---

### Q2: How to represent overtime in TimerState for frontend?

**Decision**: Add `overtime_secs: Option<u32>` to `TimerState`. When `Some(secs)`, frontend displays with minus prefix and red color.

**Rationale**:

- Clean separation: `remaining_secs` stays normal countdown, `overtime_secs` represents elapsed overtime
- Optional type makes it clear when overtime is active vs normal timer mode
- Frontend logic is simple: check for `Some(overtime)` and format accordingly
- No breaking changes to existing frontend code that reads `remaining_secs`

**Alternatives Considered**:

- **Overload remaining_secs with special values**: Rejected, confusing semantics
- **Add boolean flag + overtime field**: Redundant; `Option<u32>` serves both purposes
- **String field with formatted time**: Rejected; frontend should control formatting for localization/styling

**Implementation Notes**:

- Add `overtime_secs: Option<u32>` to `TimerState` struct
- Add `#[serde(skip_serializing_if = "Option::is_none")]` to keep JSON clean when not in overtime
- In `get_state()`, calculate `overtime_secs` from `completed_at` if in `Complete` status

---

### Q3: How to style overtime display (red color, minus prefix)?

**Decision**: Conditional CSS class and text formatting in frontend TypeScript.

**Rationale**:

- Uses existing CSS capabilities; no new dependencies
- Follows project convention of Tauri backend for state, TypeScript frontend for display
- Easy to test visually and modify styling without backend changes

**Alternatives Considered**:

- **Inline styles**: Rejected; CSS classes are more maintainable
- **Backend provides formatted string**: Rejected; violates separation of concerns
- **New UI element**: Rejected per spec constraint (use existing timer display area)

**Implementation Notes**:

- Add CSS class `.overtime { color: #dc2626; }` (red color from Tailwind red-600)
- In `formatTime()` function, check if `state.overtimeSecs` is present
- If present, format as `-MM:SS` and apply `.overtime` class to `timerDisplay` element
- Remove class when not in overtime mode

---

### Q4: What are edge cases for 59:59 cap?

**Decision**: When `elapsed >= 3599 seconds`, clamp `overtime_secs` to 3599 and stop incrementing.

**Rationale**:

- Spec requirement: "stop at -59:59, limit value"
- Simple clamp: `min(elapsed_secs, 3599)`
- Prevents display issues with larger numbers
- Reasonable limit; users unlikely to leave timer in overtime for 1+ hours

**Alternatives Considered**:

- **No cap**: Rejected per spec requirements
- **Hours format (HH:MM:SS)**: Initially considered but rejected per spec clarification
- **Stop timer updates at cap**: Rejected; cap applies to display only, internal tracking continues

**Implementation Notes**:

- In overtime calculation: `let overtime = min(elapsed_secs, 3599)`
- This caps display while allowing internal `completed_at` timestamp to remain accurate
- If user later resumes, overtime will have been capped for display purposes

---

## Technology Decisions

### Timer State Management

**Current Implementation**:

- Rust backend with `TimerService` managing state machine
- `Status` enum: `WorkReady`, `BreakReady`, `Running`, `Paused`, `Complete`
- `Complete` status already exists; perfect anchor point for overtime mode

**Extension Strategy**:

- No new status needed; overtime is a sub-state of `Complete`
- Detection: `status == Complete && completed_at.is_some()`
- Maintains existing state machine transitions

### Time Calculation

**Current Pattern**:

- Uses `std::time::Instant` for high-precision elapsed time
- `started_instant: Option<Instant>` tracks when timer starts
- `update_remaining()` calculates countdown from elapsed time

**Extension Pattern**:

- Add `completed_at: Option<Instant>` parallel to `started_instant`
- Same calculation pattern: `completed_at.elapsed().as_secs()`
- Consistent with existing codebase patterns

### Frontend Display Update

**Current Pattern**:

- 1Hz polling via `setInterval(updateUI, 1000)`
- Calls backend `get_state` command
- Updates DOM elements based on state

**Extension Pattern**:

- No change to polling frequency
- Add conditional formatting in `updateUI()`
- Applies/removes CSS class based on `overtimeSecs` presence

---

## Dependencies

**No new dependencies required.**

Existing dependencies sufficient:

- `std::time::Instant` (Rust standard library)
- `serde` (already in use for serialization)
- CSS styling (no framework needed)
- TypeScript (existing frontend language)

---

## Best Practices Applied

### Rust

- **Option types**: Use `Option<Instant>` and `Option<u32>` for optional state
- **Immutability**: Calculation functions don't mutate unrelated state
- **Type safety**: Keep `remaining_secs` as `u32`, add separate overtime field

### TypeScript

- **Type guards**: Check `state.overtimeSecs !== undefined` before formatting
- **Separation of concerns**: Backend provides data, frontend handles presentation
- **CSS classes**: Prefer classes over inline styles

### Testing

- **Unit tests**: Add tests for overtime calculation edge cases (0s, 30s, 3599s, 3600s+)
- **Integration tests**: Verify frontend-backend contract (TimerState serialization)
- **Manual testing**: Visual verification of red color and minus prefix

---

## Open Questions

**None remaining.** All technical decisions finalized.

---

## Summary

The overtime feature integrates cleanly into the existing timer architecture:

1. Backend adds `completed_at: Option<Instant>` and `overtime_secs: Option<u32>`
2. Frontend checks for `overtimeSecs` and applies conditional formatting
3. No new dependencies or architectural changes required
4. Follows existing patterns and constitution principles (simplicity, no over-engineering)

Ready to proceed to **Phase 1: Design & Contracts**.
