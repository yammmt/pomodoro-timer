# Research: Confirm Dialog for Clear Action

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)  
**Phase 0 Output**: Research findings and design decisions

## Decision Summary

All requirements can be satisfied with existing Tauri and browser APIs. No external dialog library required; native HTML/CSS modal pattern is sufficient for the small window size and focused UI. Confirmation logic can be implemented in both Rust backend (timer state management) and TypeScript frontend (UI event handling).

---

## Research Findings

### R1: Modal Dialog Implementation Pattern

**Decision**: Use native HTML/CSS modal with overlay pattern, no external UI library.

**Rationale**:

- Tauri apps use standard web technologies (HTML/CSS/JS); browser-native patterns are performant and accessible.
- Small window size (pomodoro_timer has fixed dimensions) makes complex dialog libraries unnecessary.
- HTML dialog element or flexbox overlay provides full keyboard and pointer support without additional dependencies.
- Matches existing minimal UI aesthetic of the app.

**Alternatives Considered**:

- Web component library (e.g., shoelace-ui, web-components): adds bundle size and external dependency; overkill for single modal.
- Tauri plugin (tauri-plugin-dialog): provides OS-native dialogs (Windows/macOS system panels), but less control over styling and messaging; would appear outside app window.

**Selected Approach**: CSS flexbox overlay with `<button>` elements. Dialog appears as centered panel with semi-transparent background overlay. Accessible via keyboard (Tab navigation, Enter to confirm, Escape to cancel).

---

### R2: Tauri Command for Clear Action

**Decision**: Implement a `clear_timer` command in Rust backend that handles state reset and returns new state to frontend.

**Rationale**:

- Tauri commands are the standard IPC pattern used in this project (see `src-tauri/src/lib.rs`).
- Separates UI logic (dialog presentation) from business logic (timer state management).
- Timer state in `timer.rs` module already has state machine; clear operation is a simple state transition.
- Frontend calls command after user confirms, backend validates and resets timer atomically.

**Alternatives Considered**:

- Synchronous tauri event: limits error handling and response data; less suitable for state mutations.
- Direct DOM manipulation without backend validation: risks inconsistent state if user network lag or racing conditions.

**Selected Approach**: New `clear_timer()` command in `src-tauri/src/lib.rs` that:

1. Validates current timer state (can clear if running/paused or idle).
2. Calls `timer.clear()` method to reset state to idle.
3. Returns confirmation success/failure and new timer state to UI.

---

### R3: Dialog Visibility and Dismissal

**Decision**: Show dialog only when timer has progress (remaining time > 0); escape/outside-click/close button all dismiss without clearing.

**Rationale**:

- Requirement FR-001 specifies dialog only on "destructive" clear (session with remaining time).
- If timer is idle, clear is not destructive; no dialog needed (FR-007).
- Keyboard escape, background overlay click, and close (×) button all have same behavior: dismiss and preserve state.
- Matches common modal UX patterns and accessibility expectations.

**Alternatives Considered**:

- Always show dialog even if timer idle: unnecessary friction for already-cleared state.
- Different dismiss behaviors (escape closes silently vs. button closes): confusing inconsistency.

**Selected Approach**:

- Check timer state before showing dialog: `if (timer.remaining_time > 0) showConfirmDialog()`.
- Three cancel paths (escape, overlay click, close button) all call same `cancelClear()` handler.

---

### R4: Message and Button Labeling

**Decision**: Dialog message: "This will remove the current time and status. Continue?" with buttons "Clear" (confirm) and "Cancel" (dismiss).

**Rationale**:

- Requirement FR-002 requires stating consequence (removing time/status).
- Short, clear language fits small window without scrolling (constraint).
- "Clear" button mirrors existing Clear button label; "Cancel" is familiar cancel pattern.
- Message uses second person ("This will...") for clarity.

**Alternatives Considered**:

- "Are you sure you want to clear?" — vague about consequence; doesn't explicitly mention loss of time.
- Longer message with details about session loss — risks scrolling; unnecessary detail.

**Selected Approach**: "This will remove the current time and status. Continue?" with "Clear" and "Cancel" buttons.

---

### R5: Keyboard Accessibility

**Decision**: Tab cycles through buttons, Enter confirms focused button, Escape cancels.

**Rationale**:

- Requirement FR-006 requires keyboard operability.
- HTML `<button>` elements are natively focusable and keyboard-interactive.
- Escape is standard cancel key in modal dialogs.
- No special ARIA attributes needed for simple 2-button dialog.

**Alternatives Considered**:

- Custom role="dialog" ARIA attributes: useful for complex dialogs; HTML dialog element or properly structured divs with tabindex="-1" overlay provide sufficient semantics.

**Selected Approach**:

- Dialog container: standard `<div>` with `display: none` / `display: flex` toggle.
- Buttons: native `<button>` elements; browser handles Tab focus automatically.
- Event listeners: `keydown` handler for Escape; `click` handlers for buttons.
- On dialog open, focus first button (confirm) for tab-first accessibility.

---

### R6: Timer State Query

**Decision**: Reuse existing `get_timer_state()` command (if available) or extend with query-only operation; no new command for read-only state.

**Rationale**:

- Existing timer state is already accessible to frontend (UI displays time/status).
- No new state structure needed; timer.rs already tracks remaining time and status (running/paused/idle).
- Frontend can check local cached state before calling clear (optimization), or always query backend for safety.

**Alternatives Considered**:

- Always query backend before showing dialog: safest (prevents race conditions); slight latency (likely <10ms on local Tauri IPC).
- Use only cached frontend state: faster; risks stale data if state changed externally.

**Selected Approach**: Frontend checks local state before showing dialog (fast path); backend validates again on clear command (safe path).

---

### R7: Testing Strategy

**Decision**:

- Rust unit tests in `timer/tests.rs` for `clear_timer` state transition.
- Manual/integration tests for dialog UI interaction (keyboard and pointer).
- No end-to-end test framework added (out of scope for Phase 1).

**Rationale**:

- Project already uses `cargo test` for timer logic (tests.rs exists).
- Dialog UI is simple enough for manual testing in `cargo tauri dev` workflow.
- Requirement SC-001, SC-002 are verifiable via manual runs (100% confirmation, 0 unintended resets).
- Automated UI testing would require webdriver or headless browser setup; disproportionate complexity.

**Alternatives Considered**:

- Tauri test framework (e.g., webdriver-based): adds CI/CD complexity; not justified for single dialog.
- Screenshot-based visual regression: overkill for small, simple dialog.

**Selected Approach**: Unit tests for Rust clear logic; manual interaction tests for UI (escape, enter, click).

---

## Technical Dependencies and Integration Points

| Component | Dependency | Integration | Risk |
|-----------|-----------|------------|------|
| Frontend Dialog | HTML/CSS/JS | New modal in `src/index.html` + event handler in `main.ts` | Low (standard web tech) |
| Tauri Command | tauri 2.9 | New `clear_timer()` in `src-tauri/src/lib.rs` | Low (existing pattern) |
| Timer State | timer.rs | New `clear()` method or extend existing reset logic | Low (simple state transition) |
| Testing | cargo test | New test in `src-tauri/src/timer/tests.rs` | Low (existing framework) |

---

## Unknowns Resolved

- ✅ Dialog implementation: Use native HTML/CSS modal (no library needed).
- ✅ IPC pattern: Use tauri command `clear_timer()` (standard pattern).
- ✅ State management: Timer.rs already supports state mutations; add `clear()` method.
- ✅ Keyboard handling: Native button focus + Escape keydown listener.
- ✅ Message content: "This will remove the current time and status. Continue?"
- ✅ Testing approach: Unit tests for Rust, manual tests for UI.

**Status**: All research questions resolved. Ready for Phase 1 design.
