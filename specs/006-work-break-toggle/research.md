# Research: Work/Break Mode Toggle

**Date**: 2026-01-04  
**Feature**: [specs/006-work-break-toggle/spec.md](specs/006-work-break-toggle/spec.md)

## Overview

Phase 0 research to resolve technical unknowns and validate design approach before Phase 1 detailed design.

---

## Decision 1: Frontend Architecture for Mode Buttons

**Question**: How should the Work/Break mode buttons be integrated into the existing HTML/TypeScript frontend?

**Research Findings**:
- Current `main.ts` uses vanilla TypeScript with direct DOM manipulation (querySelector, event listeners).
- Existing UI structure in `src/index.html` has a simple layout with buttons (start, pause, resume, clear) and displays.
- No framework (React, Vue, Svelte) in use; direct DOM binding preferred.

**Decision**: Add Work/Break toggle buttons as native HTML `<button>` elements in a new `<div id="mode-selector">` container above the timer display. Attach click event listeners in `main.ts` to invoke a new `set_mode(phase: "work" | "break")` Tauri command.

**Rationale**: Minimalist approach matches app architecture; avoids framework overhead; leverages existing event-driven pattern.

**Alternatives Considered**:
- Using a custom component framework: Rejected—adds complexity; single-user desktop app doesn't need it.
- Radio buttons instead of standalone buttons: Rejected—spec calls for left-aligned button row with toggle-like styling.

---

## Decision 2: Backend Mode Switch Implementation

**Question**: How should the Rust timer module handle session/phase switching without losing remaining time?

**Research Findings**:
- `timer.rs` defines `Phase` enum (Work, Break) and `Status` enum (WorkReady, BreakReady, Running, Paused, Complete).
- Current structure uses `TimerService` with fields: `phase`, `status`, `remaining_secs`, `duration_secs`, `paused_remaining`, `started_instant`.
- On `pause_timer()`, the service stores remaining seconds in `paused_remaining` for later resumption.
- Switching phase mid-countdown is not currently supported; new `set_phase()` method must be added.

**Decision**: Implement `set_phase(new_phase: Phase)` in `TimerService` that:
1. If status is Running, pause current timer (preserve remaining secs in `paused_remaining`).
2. Update `phase` field.
3. Reset `duration_secs` to match new phase (25 min for work, 5 min for break).
4. Update `status` to BreakReady or WorkReady.
5. Preserve `paused_remaining` when the user switches back (future resumption).

**Rationale**: Leverages existing `paused_remaining` mechanic; no new storage needed; atomic operation prevents race conditions.

**Alternatives Considered**:
- Create separate Work and Break timer instances: Rejected—adds state management complexity; violates simplicity principle.
- Auto-start new session on switch: Rejected—spec requires pausing on switch, not auto-running.

---

## Decision 3: IPC Command Structure

**Question**: How should the frontend invoke mode switching from the backend?

**Research Findings**:
- Existing commands use `#[tauri::command]` macro in `lib.rs`: `get_state`, `start_timer`, `pause_timer`, `resume_timer`, `clear_timer`.
- Frontend calls via `invoke<TimerState>('command_name')`.
- All commands are synchronous and return `Result<TimerState, String>`.

**Decision**: Add `set_phase` command in `lib.rs` taking phase parameter (string: "work" | "break"), calling `service.set_phase()`, returning updated `TimerState`.

```rust
#[tauri::command]
fn set_phase(phase: String, timer: tauri::State<SharedTimerService>) -> Result<TimerState, String> {
    let phase_enum = match phase.as_str() {
        "work" => Phase::Work,
        "break" => Phase::Break,
        _ => return Err("Invalid phase".to_string()),
    };
    let mut service = timer.lock().map_err(|e| e.to_string())?;
    service.set_phase(phase_enum);
    Ok(service.get_state())
}
```

**Rationale**: Consistent with existing command pattern; string phase simplifies frontend serialization; error handling matches app style.

**Alternatives Considered**:
- Numeric enum from frontend: Rejected—less readable; string matches spec language ("Work", "Break").

---

## Decision 4: Button Styling for Active State

**Question**: How should CSS differentiate active vs. inactive mode buttons?

**Research Findings**:
- Spec specifies "emphasized CSS style (for example, button color is changed to light color and added line edge)."
- No existing CSS framework in project; minimal inline or stylesheet styling.
- Current app uses a simple, clean aesthetic (matching Pomodoro simplicity principle).

**Decision**: CSS approach using `.active` class:
- Active button: lighter background color (e.g., light gray or light primary color), border or outline to indicate edge, no opacity change.
- Inactive button: default styling (darker/normal background).
- Toggle `.active` class on click; remove from previously active button.

**Rationale**: Pure CSS is performant; doesn't require framework; matches spec intent without over-design.

**Alternatives Considered**:
- Using opacity to dim inactive buttons: Rejected—less clear than color change; harder to see state.
- Using icon + label: Rejected—scope creep; spec calls for simple button text.

---

## Decision 5: Preventing Accidental Resets

**Question**: How do we ensure clicking the same session button doesn't reset remaining time?

**Research Findings**:
- Spec edge case: "Clicking the already active session button does not reset or change the remaining time and keeps the timer state unchanged."
- Backend must guard against redundant `set_phase()` calls.

**Decision**: In `set_phase()` method, check if `new_phase == self.phase`. If true, return early without modifying state.

```rust
pub fn set_phase(&mut self, new_phase: Phase) {
    if new_phase == self.phase {
        return; // No-op if already on this phase
    }
    // Pause running timer, update phase, etc.
}
```

**Rationale**: Idempotent operation; prevents accidental data loss; defensive programming.

**Alternatives Considered**:
- Disable button when already active: Rejected—still allows fast re-clicks; backend guard is more robust.

---

## Decision 6: Session State Preservation Across Switches

**Question**: How should the app track and restore remaining time when switching back to a previously paused session?

**Research Findings**:
- Spec assumption: "Switching sessions loads the standard duration for the newly selected session while retaining the paused remaining time for the previously active one."
- Currently, `paused_remaining` field stores one value (the last paused session).
- To support switching back and forth, need to track both phases' paused times.

**Decision**: Extend `TimerService` with two fields instead of one:
```rust
pub(crate) paused_work_secs: Option<u32>,    // Paused remaining time for work phase
pub(crate) paused_break_secs: Option<u32>,   // Paused remaining time for break phase
```

Update `set_phase()` to save current `paused_remaining` to the old phase's field before switching, then restore from the new phase's field.

**Rationale**: Allows users to pause Work at 12:00, switch to Break, then switch back to Work and see 12:00 again. Matches user expectation per spec.

**Alternatives Considered**:
- Single `paused_remaining` field: Would lose one phase's time on every switch; violates spec guarantee.

---

## Summary of Unknowns Resolved

| Unknown | Resolution | Confidence |
|---------|-----------|-----------|
| Frontend button integration | Vanilla HTML + TypeScript in index.html | High |
| Backend phase switching logic | `set_phase()` with bidirectional paused time tracking | High |
| IPC command design | New `set_phase(phase: String)` command | High |
| Active button styling | CSS `.active` class toggle | High |
| Preventing accidental resets | Idempotent phase check in backend | High |
| Bidirectional time preservation | Separate `paused_work_secs` and `paused_break_secs` fields | High |

All decisions align with Constitution principles (simplicity, code quality, testing standards, consistency).

---

## Next Steps

Proceed to Phase 1 to generate:
- `data-model.md` with updated `TimerService` entity
- `contracts/ipc-commands.md` with `set_phase` command spec
- `quickstart.md` with mode toggle setup steps
